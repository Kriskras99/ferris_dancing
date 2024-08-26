#![allow(
    clippy::pedantic,
    clippy::nursery,
    clippy::arithmetic_side_effects,
    clippy::missing_safety_doc,
    clippy::as_conversions,
    reason = "code is adapted from rust-lang including style"
)]
//! Path manipulation for virtual filesystems.
//!
//! This module provides two types, [`VirtualPathBuf`] and [`VirtualPath`] (akin to [`String`]
//! and [`str`]), for working with paths abstractly. These types are thin wrappers
//! around [`String`] and [`str`] respectively, meaning that they work directly
//! on strings according to the path syntax.
//!
//! Paths can be parsed into [`Component`]s by iterating over the structure
//! returned by the [`components`] method on [`VirtualPath`]. [`Component`]s roughly
//! correspond to the substrings between path separators (`/`). You can
//! reconstruct an equivalent path from components with the [`push`] method on
//! [`VirtualPathBuf`]; note that the paths may differ syntactically by the
//! normalization described in the documentation for the [`components`] method.
//!
//! [`VirtualPath`]s do not have a root component. If they are initialized with a root component
//! it will be removed upon creation. Relative components ('.' and '..') are not supported and
//! will cause a panic on creation.
//!
//! ## Case sensitivity
//!
//! Unless otherwise indicated path methods are case-sensitive no
//! matter the platform or filesystem.
//!
//! ## Simple usage
//!
//! Path manipulation includes both parsing components from slices and building
//! new owned paths.
//!
//! To parse a path, you can create a [`VirtualPath`] slice from a [`str`]
//! slice and start asking questions:
//!
//! ```
//! use dotstar_toolkit_utils::vfs::VirtualPath;
//!
//! let path = VirtualPath::new("/tmp/foo/bar.txt");
//!
//! let parent = path.parent();
//! assert_eq!(parent, Some(VirtualPath::new("tmp/foo")));
//!
//! let file_stem = path.file_stem();
//! assert_eq!(file_stem, Some("bar"));
//!
//! let extension = path.extension();
//! assert_eq!(extension, Some("txt"));
//! ```
//!
//! To build or modify paths, use [`VirtualPathBuf`]:
//!
//! ```
//! use dotstar_toolkit_utils::vfs::VirtualPathBuf;
//!
//! let mut path = VirtualPathBuf::from("windows");
//! path.push("system32");
//! path.set_extension("dll");
//! ```
//!
//! Implementation based on [`the Rust implementation`](https://github.com/rust-lang/rust), licensed under MIT|Apache-2.0
//!
//! [`components`]: VirtualPath::components
//! [`push`]: VirtualPathBuf::push

use std::{
    borrow::{Borrow, Cow},
    cmp,
    collections::TryReserveError,
    fmt::{Display, Formatter},
    iter::FusedIterator,
    ops::{Deref, DerefMut},
    path::Path,
    rc::Rc,
    str::{FromStr, Utf8Error},
    sync::Arc,
};

/// Create a [`VirtualPathBuf`] from a [`format!`] string
///
/// This expands to `VirtualPathBuf::from(format!(/* macro arguments */))`
#[macro_export]
macro_rules! path {
    ($($t:tt)*) => {{
        VirtualPathBuf::from(format!($($t)*))
    }};
}

////////////////////////////////////////////////////////////////////////////////
// Exposed parsing helpers
////////////////////////////////////////////////////////////////////////////////

/// The separator of path components
pub const SEPARATOR: char = '/';

////////////////////////////////////////////////////////////////////////////////
// Misc helpers
////////////////////////////////////////////////////////////////////////////////

// Iterate through `iter` while it matches `prefix`; return `None` if `prefix`
// is not a prefix of `iter`, otherwise return `Some(iter_after_prefix)` giving
// `iter` after having exhausted `prefix`.
fn iter_after<'a, 'b, I, J>(mut iter: I, mut prefix: J) -> Option<I>
where
    I: Iterator<Item = Component<'a>> + Clone,
    J: Iterator<Item = Component<'b>>,
{
    loop {
        let mut iter_next = iter.clone();
        match (iter_next.next(), prefix.next()) {
            (Some(ref x), Some(ref y)) if x == y => (),
            (Some(_), Some(_)) => return None,
            (Some(_), None) => return Some(iter),
            (None, None) => return Some(iter),
            (None, Some(_)) => return None,
        }
        iter = iter_next;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cross-platform, iterator-independent parsing
////////////////////////////////////////////////////////////////////////////////

// basic workhorse for splitting stem and extension
fn rsplit_file_at_dot(file: &str) -> (Option<&str>, Option<&str>) {
    if file == ".." {
        return (Some(file), None);
    }

    // The unsafety here stems from converting between &str and &[u8]
    // and back. This is safe to do because (1) we only look at ASCII
    // contents of the encoding and (2) new &str values are produced
    // only from ASCII-bounded slices of existing &str values.
    let mut iter = file.rsplitn(2, '.');
    let after = iter.next();
    let before = iter.next();
    if before == Some("") {
        (Some(file), None)
    } else {
        (before, after)
    }
}

////////////////////////////////////////////////////////////////////////////////
// The core iterators
////////////////////////////////////////////////////////////////////////////////

/// Component parsing works by a double-ended state machine; the cursors at the
/// front and back of the path each keep track of what parts of the path have
/// been consumed so far.
///
/// Going front to back, a path is made up of a prefix, a starting
/// directory component, and a body (of normal components)
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
enum State {
    StartDir = 1, // / or . or nothing
    Body = 2,     // foo/bar/baz
    Done = 3,
}

/// A single component of a path.
///
/// A `Component` roughly corresponds to a substring between path separators
/// (`/` or `\`).
///
/// This `enum` is created by iterating over [`Components`], which in turn is
/// created by the [`components`](VirtualPath::components) method on [`VirtualPath`].
///
/// # Examples
///
/// ```rust
/// use dotstar_toolkit_utils::vfs::{Component, VirtualPath};
///
/// let path = VirtualPath::new("/tmp/foo/bar.txt");
/// let components = path.components().map(Component::as_str).collect::<Vec<_>>();
/// assert_eq!(&components, &["tmp", "foo", "bar.txt" ]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Component<'a> {
    inner: &'a str,
}

impl<'a> Component<'a> {
    /// Extracts the underlying [`str`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// let path = VirtualPath::new("tmp/foo/bar.txt");
    /// let components: Vec<_> = path.components().map(|comp| comp.as_str()).collect();
    /// assert_eq!(&components, &["tmp", "foo", "bar.txt"]);
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn as_str(self) -> &'a str {
        self.inner
    }
}

impl AsRef<str> for Component<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<VirtualPath> for Component<'_> {
    #[inline]
    fn as_ref(&self) -> &VirtualPath {
        self.as_str().as_ref()
    }
}

/// An iterator over the [`Component`]s of a [`VirtualPath`].
///
/// This `struct` is created by the [`components`] method on [`VirtualPath`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use dotstar_toolkit_utils::vfs::VirtualPath;
///
/// let path = VirtualPath::new("/tmp/foo/bar.txt");
///
/// for component in path.components() {
///     println!("{component:?}");
/// }
/// ```
///
/// [`components`]: VirtualPath::components
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Components<'a> {
    // The path left to parse components from
    path: &'a str,

    // The iterator is double-ended, and these two states keep track of what has
    // been produced from either end
    front: State,
    back: State,
}

/// An iterator over the [`Component`]s of a [`VirtualPath`], as [`str`] slices.
///
/// This `struct` is created by the [`iter`] method on [`VirtualPath`].
/// See its documentation for more.
///
/// [`iter`]: VirtualPath::iter
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a> {
    inner: Components<'a>,
}

impl<'a> Components<'a> {
    // is the iteration complete?
    #[inline]
    fn finished(&self) -> bool {
        self.front == State::Done || self.back == State::Done || self.front > self.back
    }

    /// Extracts a slice corresponding to the portion of the path remaining for iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// let mut components = Path::new("tmp/foo/bar.txt").components();
    /// components.next();
    ///
    /// assert_eq!(Path::new("foo/bar.txt"), components.as_path());
    /// ```
    #[must_use]
    pub fn as_path(&self) -> &'a VirtualPath {
        let mut comps = self.clone();
        if comps.front == State::Body {
            comps.trim_left();
        }
        if comps.back == State::Body {
            comps.trim_right();
        }
        VirtualPath::new(comps.path)
    }

    // parse a given byte sequence following the str encoding into the
    // corresponding path component
    fn parse_single_component<'b>(&self, comp: &'b str) -> Option<Component<'b>> {
        match comp {
            "." => None, // . components are normalized away, except at
            // the beginning of a path, which is treated
            // separately via `include_cur_dir`
            ".." => panic!("Relative path component (..) is not supported!"),
            "" => None,
            _ => Some(Component { inner: comp }),
        }
    }

    // parse a component from the left, saying how many bytes to consume to
    // remove the component
    fn parse_next_component(&self) -> (usize, Option<Component<'a>>) {
        debug_assert!(self.front == State::Body, "should be a body!");
        let (extra, comp) = match self.path.find('/') {
            None => (0, self.path),
            Some(i) => (1, &self.path[..i]),
        };

        (comp.len() + extra, self.parse_single_component(comp))
    }

    // parse a component from the right, saying how many bytes to consume to
    // remove the component
    fn parse_next_component_back(&self) -> (usize, Option<Component<'a>>) {
        debug_assert!(self.back == State::Body, "should start with body!");

        let (extra, comp) = match self.path.rfind('/') {
            None => (0, self.path),
            Some(i) => (1, &self.path[i + 1..]),
        };

        (comp.len() + extra, self.parse_single_component(comp))
    }

    // trim away repeated separators (i.e., empty components) on the left
    fn trim_left(&mut self) {
        while !self.path.is_empty() {
            let (size, comp) = self.parse_next_component();
            if comp.is_some() {
                return;
            } else {
                self.path = &self.path[size..];
            }
        }
    }

    // trim away repeated separators (i.e., empty components) on the right
    fn trim_right(&mut self) {
        while !self.path.is_empty() {
            let (size, comp) = self.parse_next_component_back();
            if comp.is_some() {
                return;
            } else {
                self.path = &self.path[..self.path.len() - size];
            }
        }
    }
}

impl AsRef<VirtualPath> for Components<'_> {
    #[inline]
    fn as_ref(&self) -> &VirtualPath {
        self.as_path()
    }
}

impl AsRef<str> for Components<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_path().as_str()
    }
}

impl<'a> Iter<'a> {
    /// Extracts a slice corresponding to the portion of the path remaining for iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// let mut iter = VirtualPath::new("tmp/foo/bar.txt").iter();
    /// iter.next();
    ///
    /// assert_eq!(VirtualPath::new("foo/bar.txt"), iter.as_path());
    /// ```
    #[must_use]
    #[inline]
    pub fn as_path(&self) -> &'a VirtualPath {
        self.inner.as_path()
    }
}

impl AsRef<VirtualPath> for Iter<'_> {
    #[inline]
    fn as_ref(&self) -> &VirtualPath {
        self.as_path()
    }
}

impl AsRef<str> for Iter<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_path().as_str()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        self.inner.next().map(Component::as_str)
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a str> {
        self.inner.next_back().map(Component::as_str)
    }
}

impl FusedIterator for Iter<'_> {}

impl<'a> Iterator for Components<'a> {
    type Item = Component<'a>;

    fn next(&mut self) -> Option<Component<'a>> {
        while !self.finished() {
            match self.front {
                State::StartDir => {
                    self.front = State::Body;
                }
                State::Body if !self.path.is_empty() => {
                    let (size, comp) = self.parse_next_component();
                    self.path = &self.path[size..];
                    if comp.is_some() {
                        return comp;
                    }
                }
                State::Body => {
                    self.front = State::Done;
                }
                State::Done => unreachable!(),
            }
        }
        None
    }
}

impl<'a> DoubleEndedIterator for Components<'a> {
    fn next_back(&mut self) -> Option<Component<'a>> {
        while !self.finished() {
            match self.back {
                State::Body if !self.path.is_empty() => {
                    let (size, comp) = self.parse_next_component_back();
                    self.path = &self.path[..self.path.len() - size];
                    if comp.is_some() {
                        return comp;
                    }
                }
                State::Body => {
                    self.back = State::StartDir;
                }
                State::StartDir => {
                    self.back = State::Done;
                }
                State::Done => unreachable!(),
            }
        }
        None
    }
}

impl FusedIterator for Components<'_> {}

impl<'a> PartialEq for Components<'a> {
    #[inline]
    fn eq(&self, other: &Components<'a>) -> bool {
        let Components {
            path: _,
            front: _,
            back: _,
        } = self;

        // Fast path for exact matches, e.g. for hashmap lookups.
        if self.path.len() == other.path.len()
            && self.front == other.front
            && self.back == State::Body
            && other.back == State::Body
        {
            // possible future improvement: this could bail out earlier if there were a
            // reverse memcmp/bcmp comparing back to front
            if self.path == other.path {
                return true;
            }
        }

        // compare back to front since absolute paths often share long prefixes
        Iterator::eq(self.clone().rev(), other.clone().rev())
    }
}

impl Eq for Components<'_> {}

impl<'a> PartialOrd for Components<'a> {
    #[inline]
    fn partial_cmp(&self, other: &Components<'a>) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Components<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        compare_components(self.clone(), other.clone())
    }
}

fn compare_components(mut left: Components<'_>, mut right: Components<'_>) -> cmp::Ordering {
    // Fast path for long shared prefixes
    //
    // - compare raw strings to find first mismatch
    // - backtrack to find separator before mismatch to avoid ambiguous parsings of '.' character
    // - if found update state to only do a component-wise comparison on the remainder,
    //   otherwise do it on the full path
    if left.front == right.front {
        let first_difference = match left
            .path
            .chars()
            .zip(right.path.chars())
            .position(|(a, b)| a != b)
        {
            None if left.path.len() == right.path.len() => return cmp::Ordering::Equal,
            None => left.path.len().min(right.path.len()),
            Some(diff) => diff,
        };

        if let Some(previous_sep) = left.path[..first_difference].rfind('/') {
            let mismatched_component_start = previous_sep + 1;
            left.path = &left.path[mismatched_component_start..];
            left.front = State::Body;
            right.path = &right.path[mismatched_component_start..];
            right.front = State::Body;
        }
    }

    Iterator::cmp(left, right)
}

/// An iterator over [`VirtualPath`] and its ancestors.
///
/// This `struct` is created by the [`ancestors`] method on [`VirtualPath`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use dotstar_toolkit_utils::vfs::VirtualPath;
///
/// let path = VirtualPath::new("foo/bar");
///
/// for ancestor in path.ancestors() {
///     println!("{ancestor}");
/// }
/// ```
///
/// [`ancestors`]: Path::ancestors
#[derive(Copy, Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Ancestors<'a> {
    next: Option<&'a VirtualPath>,
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = &'a VirtualPath;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = next.and_then(VirtualPath::parent);
        next
    }
}

impl FusedIterator for Ancestors<'_> {}

////////////////////////////////////////////////////////////////////////////////
// Basic types and traits
////////////////////////////////////////////////////////////////////////////////

/// An owned, mutable path (akin to [`String`]).
///
/// This type provides methods like [`push`] and [`set_extension`] that mutate
/// the path in place. It also implements [`Deref`] to [`VirtualPath`], meaning that
/// all methods on [`Path`] slices are available on `PathBuf` values as well.
///
/// [`push`]: VirtualPathBuf::push
/// [`set_extension`]: VirtualPathBuf::set_extension
///
/// More details about the overall approach can be found in
/// the [module documentation](self).
///
/// # Examples
///
/// You can use [`push`] to build up a `VirtualPathBuf` from
/// components:
///
/// ```
/// use dotstar_toolkit_utils::vfs::VirtualPathBuf;
///
/// let mut path = VirtualPathBuf::new();
///
/// path.push("windows");
/// path.push("system32");
///
/// path.set_extension("dll");
/// ```
///
/// However, [`push`] is best used for dynamic situations. This is a better way
/// to do this when you know all of the components ahead of time:
///
/// ```
/// use dotstar_toolkit_utils::vfs::VirtualPathBuf;
///
/// let path = VirtualPathBuf::from("windows/system32.dll");
/// ```
#[repr(transparent)]
pub struct VirtualPathBuf {
    inner: String,
}

impl VirtualPathBuf {
    /// Allocates an empty `VirtualPathBuf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPathBuf;
    ///
    /// let path = VirtualPathBuf::new();
    /// ```
    #[must_use]
    #[inline]
    pub fn new() -> VirtualPathBuf {
        VirtualPathBuf {
            inner: String::new(),
        }
    }

    /// Creates a new `VirtualPathBuf` with a given capacity used to create the
    /// internal [`String`]. See [`with_capacity`] defined on [`String`].
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPathBuf;
    ///
    /// let mut path = VirtualPathBuf::with_capacity(10);
    /// let capacity = path.capacity();
    ///
    /// // This push is done without reallocating
    /// path.push("file.rs");
    ///
    /// assert_eq!(capacity, path.capacity());
    /// ```
    ///
    /// [`with_capacity`]: String::with_capacity
    #[must_use]
    #[inline]
    pub fn with_capacity(capacity: usize) -> VirtualPathBuf {
        VirtualPathBuf {
            inner: String::with_capacity(capacity),
        }
    }

    /// Coerces to a [`VirtualPath`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
    ///
    /// let p = VirtualPathBuf::from("test");
    /// assert_eq!(VirtualPath::new("test"), p.as_path());
    /// ```
    #[must_use]
    #[inline]
    pub fn as_path(&self) -> &VirtualPath {
        self
    }

    /// Extends `self` with `path`.
    ///
    /// Consider using [`VirtualPath::join`] if you need a new `VirtualPathBuf` instead of
    /// using this function on a cloned `VirtualPathBuf`.
    ///
    /// # Example
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPathBuf;
    ///
    /// let mut path = VirtualPathBuf::from("tmp");
    /// path.push("file.bk");
    /// assert_eq!(path, VirtualPathBuf::from("tmp/file.bk"));
    /// ```
    pub fn push<P: AsRef<VirtualPath>>(&mut self, path: P) {
        self._push(path.as_ref());
    }

    fn _push(&mut self, path: &VirtualPath) {
        // in general, a separator is needed if the rightmost char of self is not a separator and
        // the leftmost char of path is not a separator
        let need_sep = self.inner.chars().last().is_some_and(|c| c != SEPARATOR)
            && self.inner.chars().next().is_some_and(|c| c != SEPARATOR);

        // `path` is a pure relative path
        if need_sep {
            self.inner.push(SEPARATOR);
        }

        self.inner.push_str(&path.inner);
        self.check_path();
    }

    /// Truncates `self` to [`self.parent`].
    ///
    /// Returns `false` and does nothing if [`self.parent`] is [`None`].
    /// Otherwise, returns `true`.
    ///
    /// [`self.parent`]: VirtualPath::parent
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
    ///
    /// let mut p = VirtualPathBuf::from("spirited/away.rs");
    ///
    /// p.pop();
    /// assert_eq!(VirtualPath::new("spirited"), p);
    /// p.pop();
    /// assert_eq!(VirtualPath::new(""), p);
    /// assert_eq!(false, p.pop());
    /// ```
    pub fn pop(&mut self) -> bool {
        match self.parent().map(|p| p.inner.len()) {
            Some(len) => {
                self.inner.truncate(len);
                true
            }
            None => false,
        }
    }

    /// Updates [`self.file_name`] to `file_name`.
    ///
    /// If [`self.file_name`] was [`None`], this is equivalent to pushing
    /// `file_name`.
    ///
    /// Otherwise, it is equivalent to calling [`pop`] and then pushing
    /// `file_name`. The new path will be a sibling of the original path.
    /// (That is, it will have the same parent.)
    ///
    /// [`self.file_name`]: VirtualPath::file_name
    /// [`pop`]: VirtualPathBuf::pop
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPathBuf;
    ///
    /// let mut buf = VirtualPathBuf::from("");
    /// assert_eq!(buf.file_name(), None);
    ///
    /// buf.set_file_name("foo.txt");
    /// assert_eq!(buf, VirtualPathBuf::from("foo.txt"));
    /// assert!(buf.file_name().is_some());
    ///
    /// buf.set_file_name("bar.txt");
    /// assert_eq!(buf, VirtualPathBuf::from("bar.txt"));
    ///
    /// buf.set_file_name("baz");
    /// assert_eq!(buf, VirtualPathBuf::from("baz"));
    /// ```
    pub fn set_file_name<S: AsRef<str>>(&mut self, file_name: S) {
        self._set_file_name(file_name.as_ref());
    }

    fn _set_file_name(&mut self, file_name: &str) {
        if self.file_name().is_some() {
            let popped = self.pop();
            debug_assert!(popped, "should've popped");
        }
        self.push(file_name);
        self.check_path();
    }

    /// Updates [`self.extension`] to `Some(extension)` or to `None` if
    /// `extension` is empty.
    ///
    /// Returns `false` and does nothing if [`self.file_name`] is [`None`],
    /// returns `true` and updates the extension otherwise.
    ///
    /// If [`self.extension`] is [`None`], the extension is added; otherwise
    /// it is replaced.
    ///
    /// If `extension` is the empty string, [`self.extension`] will be [`None`]
    /// afterwards, not `Some("")`.
    ///
    /// # Caveats
    ///
    /// The new `extension` may contain dots and will be used in its entirety,
    /// but only the part after the final dot will be reflected in
    /// [`self.extension`].
    ///
    /// If the file stem contains internal dots and `extension` is empty, part
    /// of the old file stem will be considered the new [`self.extension`].
    ///
    /// See the examples below.
    ///
    /// [`self.file_name`]: VirtualPath::file_name
    /// [`self.extension`]: VirtualPath::extension
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
    ///
    /// let mut p = VirtualPathBuf::from("/feel/the");
    ///
    /// p.set_extension("force");
    /// assert_eq!(VirtualPath::new("/feel/the.force"), p.as_path());
    ///
    /// p.set_extension("dark.side");
    /// assert_eq!(VirtualPath::new("/feel/the.dark.side"), p.as_path());
    ///
    /// p.set_extension("cookie");
    /// assert_eq!(VirtualPath::new("/feel/the.dark.cookie"), p.as_path());
    ///
    /// p.set_extension("");
    /// assert_eq!(VirtualPath::new("/feel/the.dark"), p.as_path());
    ///
    /// p.set_extension("");
    /// assert_eq!(VirtualPath::new("/feel/the"), p.as_path());
    ///
    /// p.set_extension("");
    /// assert_eq!(VirtualPath::new("/feel/the"), p.as_path());
    /// ```
    pub fn set_extension<S: AsRef<str>>(&mut self, extension: S) -> bool {
        self._set_extension(extension.as_ref())
    }

    fn _set_extension(&mut self, extension: &str) -> bool {
        let file_stem = match self.file_stem() {
            None => return false,
            Some(f) => f,
        };
        let file_name = self.file_name().unwrap_or_else(|| unreachable!());

        // truncate until right after the file stem
        self.inner
            .truncate(self.inner.len() - file_name.len() + file_stem.len());

        // add the new extension, if any
        if !extension.is_empty() {
            self.inner.reserve_exact(extension.len() + 1);
            self.inner.push('.');
            self.inner.push_str(extension);
        }

        self.check_path();

        true
    }

    /// Converts this `VirtualPathBuf` into a [boxed](Box) [`VirtualPath`].
    #[must_use = "`self` will be dropped if the result is not used"]
    #[inline]
    pub fn into_boxed_path(self) -> Box<VirtualPath> {
        let rw = Box::into_raw(self.inner.into_boxed_str()) as *mut VirtualPath;
        unsafe { Box::from_raw(rw) }
    }

    /// Invokes [`capacity`] on the underlying instance of [`String`].
    ///
    /// [`capacity`]: String::capacity
    #[must_use]
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Invokes [`clear`] on the underlying instance of [`String`].
    ///
    /// [`clear`]: String::clear
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Invokes [`reserve`] on the underlying instance of [`String`].
    ///
    /// [`reserve`]: String::reserve
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Invokes [`try_reserve`] on the underlying instance of [`String`].
    ///
    /// [`try_reserve`]: String::try_reserve
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    /// Invokes [`reserve_exact`] on the underlying instance of [`String`].
    ///
    /// [`reserve_exact`]: String::reserve_exact
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    /// Invokes [`try_reserve_exact`] on the underlying instance of [`String`].
    ///
    /// [`try_reserve_exact`]: String::try_reserve_exact
    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    /// Invokes [`shrink_to_fit`] on the underlying instance of [`String`].
    ///
    /// [`shrink_to_fit`]: String::shrink_to_fit
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Invokes [`shrink_to`] on the underlying instance of [`String`].
    ///
    /// [`shrink_to`]: String::shrink_to
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }

    /// Get the inner string
    #[inline]
    pub fn into_string(self) -> String {
        self.inner
    }
}

impl Display for VirtualPathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl std::fmt::Debug for VirtualPathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl Clone for VirtualPathBuf {
    #[inline]
    fn clone(&self) -> Self {
        VirtualPathBuf {
            inner: self.inner.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.inner.clone_from(&source.inner);
    }
}

impl From<&VirtualPath> for Box<VirtualPath> {
    /// Creates a boxed [`VirtualPath`] from a reference.
    ///
    /// This will allocate and clone `path` to it.
    fn from(path: &VirtualPath) -> Box<VirtualPath> {
        let boxed: Box<str> = path.inner.into();
        let rw = Box::into_raw(boxed) as *mut VirtualPath;
        unsafe { Box::from_raw(rw) }
    }
}

impl From<Cow<'_, VirtualPath>> for Box<VirtualPath> {
    /// Creates a boxed [`VirtualPath`] from a clone-on-write pointer.
    ///
    /// Converting from a `Cow::Owned` does not clone or allocate.
    #[inline]
    fn from(cow: Cow<'_, VirtualPath>) -> Box<VirtualPath> {
        match cow {
            Cow::Borrowed(path) => Box::from(path),
            Cow::Owned(path) => Box::from(path),
        }
    }
}

impl From<Box<VirtualPath>> for VirtualPathBuf {
    /// Converts a <code>[Box]&lt;[VirtualPath]&gt;</code> into a [`VirtualPathBuf`].
    ///
    /// This conversion does not allocate or copy memory.
    #[inline]
    fn from(boxed: Box<VirtualPath>) -> VirtualPathBuf {
        boxed.into_path_buf()
    }
}

impl From<VirtualPathBuf> for Box<VirtualPath> {
    /// Converts a [`VirtualPathBuf`] into a <code>[Box]&lt;[VirtualPath]&gt;</code>.
    ///
    /// This conversion currently should not allocate memory,
    /// but this behavior is not guaranteed on all platforms or in all future versions.
    #[inline]
    fn from(p: VirtualPathBuf) -> Box<VirtualPath> {
        p.into_boxed_path()
    }
}

impl Clone for Box<VirtualPath> {
    #[inline]
    fn clone(&self) -> Self {
        self.to_path_buf().into_boxed_path()
    }
}

impl From<&str> for VirtualPathBuf {
    /// Converts a borrowed [`str`] to a [`VirtualPathBuf`].
    ///
    /// Allocates a [`VirtualPathBuf`] and copies the data into it.
    #[inline]
    fn from(s: &str) -> VirtualPathBuf {
        let s = s.strip_prefix('/').unwrap_or(s);
        let path = Self {
            inner: s.to_string(),
        };
        path.check_path();
        path
    }
}

impl From<Cow<'_, str>> for VirtualPathBuf {
    /// Converts a [`Cow<str>`] to a [`VirtualPathBuf`].
    ///
    /// Reuses the allocation if the [`Cow`] is owned.
    #[inline]
    fn from(s: Cow<'_, str>) -> VirtualPathBuf {
        match s {
            Cow::Borrowed(s) => Self::from(s),
            Cow::Owned(s) => Self::from(s),
        }
    }
}

impl From<&String> for VirtualPathBuf {
    /// Converts a borrowed [`String`] to a [`VirtualPathBuf`].
    ///
    /// Allocates a [`VirtualPathBuf`] and copies the data into it.
    #[inline]
    fn from(s: &String) -> VirtualPathBuf {
        Self::from(s.as_str())
    }
}

impl From<String> for VirtualPathBuf {
    /// Converts a [`String`] into a [`VirtualPathBuf`]
    ///
    /// This conversion does not allocate or copy memory.
    #[inline]
    fn from(mut s: String) -> VirtualPathBuf {
        if s.starts_with('/') {
            s.remove(0);
        }
        let path = VirtualPathBuf { inner: s };
        path.check_path();
        path
    }
}

impl FromStr for VirtualPathBuf {
    type Err = core::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(VirtualPathBuf::from(s))
    }
}

impl AsRef<Path> for VirtualPathBuf {
    fn as_ref(&self) -> &Path {
        self.inner.as_ref()
    }
}

impl AsRef<str> for VirtualPathBuf {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

impl From<VirtualPathBuf> for std::ffi::OsString {
    fn from(value: VirtualPathBuf) -> Self {
        value.inner.into()
    }
}

impl Deref for VirtualPathBuf {
    type Target = VirtualPath;
    #[inline]
    fn deref(&self) -> &VirtualPath {
        VirtualPath::new(&self.inner)
    }
}

impl DerefMut for VirtualPathBuf {
    #[inline]
    fn deref_mut(&mut self) -> &mut VirtualPath {
        VirtualPath::from_inner_mut(&mut self.inner)
    }
}

impl Borrow<VirtualPath> for VirtualPathBuf {
    #[inline]
    fn borrow(&self) -> &VirtualPath {
        self.deref()
    }
}

impl Default for VirtualPathBuf {
    #[inline]
    fn default() -> Self {
        VirtualPathBuf::new()
    }
}

impl<'a> From<&'a VirtualPath> for Cow<'a, VirtualPath> {
    /// Creates a clone-on-write pointer from a reference to
    /// [`VirtualPath`].
    ///
    /// This conversion does not clone or allocate.
    #[inline]
    fn from(s: &'a VirtualPath) -> Cow<'a, VirtualPath> {
        Cow::Borrowed(s)
    }
}

impl<'a> From<VirtualPathBuf> for Cow<'a, VirtualPath> {
    /// Creates a clone-on-write pointer from an owned
    /// instance of [`VirtualPathBuf`].
    ///
    /// This conversion does not clone or allocate.
    #[inline]
    fn from(s: VirtualPathBuf) -> Cow<'a, VirtualPath> {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a VirtualPathBuf> for Cow<'a, VirtualPath> {
    /// Creates a clone-on-write pointer from a reference to
    /// [`VirtualPathBuf`].
    ///
    /// This conversion does not clone or allocate.
    #[inline]
    fn from(p: &'a VirtualPathBuf) -> Cow<'a, VirtualPath> {
        Cow::Borrowed(p.as_path())
    }
}

impl<'a> From<Cow<'a, VirtualPath>> for VirtualPathBuf {
    /// Converts a clone-on-write pointer to an owned path.
    ///
    /// Converting from a `Cow::Owned` does not clone or allocate.
    #[inline]
    fn from(p: Cow<'a, VirtualPath>) -> Self {
        p.into_owned()
    }
}

impl From<VirtualPathBuf> for Arc<VirtualPath> {
    /// Converts a [`VirtualPathBuf`] into an <code>[Arc]<[VirtualPath]></code> by moving the [`VirtualPathBuf`] data
    /// into a new [`Arc`] buffer.
    #[inline]
    fn from(s: VirtualPathBuf) -> Arc<VirtualPath> {
        let arc: Arc<str> = Arc::from(s.into_string());
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const VirtualPath) }
    }
}

impl From<&VirtualPath> for Arc<VirtualPath> {
    /// Converts a [`VirtualPath`] into an [`Arc`] by copying the [`VirtualPath`] data into a new [`Arc`] buffer.
    #[inline]
    fn from(s: &VirtualPath) -> Arc<VirtualPath> {
        let arc: Arc<str> = Arc::from(s.as_str());
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const VirtualPath) }
    }
}

impl From<VirtualPathBuf> for Rc<VirtualPath> {
    /// Converts a [`VirtualPathBuf`] into an <code>[Rc]<[VirtualPath]></code> by moving the [`VirtualPathBuf`] data into
    /// a new [`Rc`] buffer.
    #[inline]
    fn from(s: VirtualPathBuf) -> Rc<VirtualPath> {
        let rc: Rc<str> = Rc::from(s.into_string());
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const VirtualPath) }
    }
}

impl From<&VirtualPath> for Rc<VirtualPath> {
    /// Converts a [`VirtualPath`] into an [`Rc`] by copying the [`VirtualPath`] data into a new [`Rc`] buffer.
    #[inline]
    fn from(s: &VirtualPath) -> Rc<VirtualPath> {
        let rc: Rc<str> = Rc::from(s.as_str());
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const VirtualPath) }
    }
}

impl ToOwned for VirtualPath {
    type Owned = VirtualPathBuf;
    #[inline]
    fn to_owned(&self) -> VirtualPathBuf {
        self.to_path_buf()
    }
    #[inline]
    fn clone_into(&self, target: &mut VirtualPathBuf) {
        self.inner.clone_into(&mut target.inner);
    }
}

impl PartialEq for VirtualPathBuf {
    #[inline]
    fn eq(&self, other: &VirtualPathBuf) -> bool {
        self.components() == other.components()
    }
}

impl std::hash::Hash for VirtualPathBuf {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        self.as_path().hash(h);
    }
}

impl Eq for VirtualPathBuf {}

impl PartialOrd for VirtualPathBuf {
    #[inline]
    fn partial_cmp(&self, other: &VirtualPathBuf) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VirtualPathBuf {
    #[inline]
    fn cmp(&self, other: &VirtualPathBuf) -> cmp::Ordering {
        compare_components(self.components(), other.components())
    }
}

/// A slice of a path (akin to [`str`]).
///
/// This type supports a number of operations for inspecting a path, including
/// breaking the path into its components (separated by `/`), extracting the file name,
/// determining whether the path is absolute, and so on.
///
/// This is an *unsized* type, meaning that it must always be used behind a
/// pointer like `&` or [`Box`]. For an owned version of this type,
/// see [`VirtualPathBuf`].
///
/// More details about the overall approach can be found in
/// the [module documentation](self).
///
/// # Examples
///
/// ```
/// use dotstar_toolkit_utils::vfs::VirtualPath;
///
/// let path = VirtualPath::new("foo/bar.txt");
///
/// let parent = path.parent();
/// assert_eq!(parent, Some(VirtualPath::new("foo")));
///
/// let file_stem = path.file_stem();
/// assert_eq!(file_stem, Some("bar"));
///
/// let extension = path.extension();
/// assert_eq!(extension, Some("txt"));
/// ```
#[repr(transparent)]
pub struct VirtualPath {
    inner: str,
}

/// An error returned from [`VirtualPath::strip_prefix`] if the prefix was not found.
///
/// This `struct` is created by the [`strip_prefix`] method on [`VirtualPath`].
/// See its documentation for more.
///
/// [`strip_prefix`]: VirtualPath::strip_prefix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StripPrefixError(());

impl VirtualPath {
    // The following (private!) function reveals the byte encoding used for str.
    fn as_u8_slice(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    /// Directly wraps a string slice as a `VirtualPath` slice.
    ///
    /// This is a cost-free conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// VirtualPath::new("foo.txt");
    /// ```
    ///
    /// You can create `VirtualPath`s from `String`s, or even other `VirtualPath`s:
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// let string = String::from("foo.txt");
    /// let from_string = VirtualPath::new(&string);
    /// let from_path = VirtualPath::new(&from_string);
    /// assert_eq!(from_string, from_path);
    /// ```
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> &VirtualPath {
        let mut s = s.as_ref();
        if s.starts_with('/') {
            s = &s[1..];
        }
        let path = unsafe { &*(s as *const str as *const VirtualPath) };
        path.check_path();
        path
    }

    fn from_inner_mut(inner: &mut str) -> &mut VirtualPath {
        // SAFETY: VirtualPath is just a wrapper around str,
        // therefore converting &mut str to &mut VirtualPath is safe.
        let path = unsafe { &mut *(inner as *mut str as *mut VirtualPath) };
        path.check_path();
        path
    }

    /// Yields the underlying [`str`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// let string = VirtualPath::new("foo.txt").as_str();
    /// assert_eq!(string, "foo.txt");
    /// ```
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Converts a `VirtualPath` to an owned [`VirtualPathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
    ///
    /// let path_buf = VirtualPath::new("foo.txt").to_path_buf();
    /// assert_eq!(path_buf, VirtualPathBuf::from("foo.txt"));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub fn to_path_buf(&self) -> VirtualPathBuf {
        VirtualPathBuf::from(self.inner.to_string())
    }

    /// Returns the `VirtualPath` without its final component, if there is one.
    ///
    /// This means it returns `Some("")` for relative paths with one component.
    ///
    /// Returns [`None`] if it is the empty string.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// let path = VirtualPath::new("foo/bar");
    /// let parent = path.parent().unwrap();
    /// assert_eq!(path.parent(), Some(VirtualPath::new("foo")));
    ///
    /// let grand_parent = parent.parent().unwrap();
    /// assert_eq!(grand_parent, VirtualPath::new(""));
    /// assert_eq!(grand_parent.parent(), None);
    ///
    /// let relative_path = VirtualPath::new("foo/bar");
    /// let parent = relative_path.parent();
    /// assert_eq!(parent, Some(VirtualPath::new("foo")));
    /// let grand_parent = parent.and_then(VirtualPath::parent);
    /// assert_eq!(grand_parent, Some(VirtualPath::new("")));
    /// let great_grand_parent = grand_parent.and_then(VirtualPath::parent);
    /// assert_eq!(great_grand_parent, None);
    /// ```
    #[doc(alias = "dirname")]
    #[must_use]
    pub fn parent(&self) -> Option<&VirtualPath> {
        let mut comps = self.components();
        comps.next_back().map(|_| comps.as_path())
    }

    /// Produces an iterator over `VirtualPath` and its ancestors.
    ///
    /// The iterator will yield the `VirtualPath` that is returned if the [`parent`] method is used zero
    /// or more times. If the [`parent`] method returns [`None`], the iterator will do likewise.
    /// The iterator will always yield at least one value, namely `Some(&self)`. Next it will yield
    /// `&self.parent()`, `&self.parent().and_then(VirtualPath::parent)` and so on.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// let mut ancestors = VirtualPath::new("foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(VirtualPath::new("foo/bar")));
    /// assert_eq!(ancestors.next(), Some(VirtualPath::new("foo")));
    /// assert_eq!(ancestors.next(), Some(VirtualPath::new("")));
    /// assert_eq!(ancestors.next(), None);
    /// ```
    ///
    /// [`parent`]: VirtualPath::parent
    #[inline]
    pub fn ancestors(&self) -> Ancestors<'_> {
        Ancestors { next: Some(self) }
    }

    /// Returns the final component of the `VirtualPath`, if there is one.
    ///
    /// If the path is a normal file, this is the file name. If it's the path of a directory, this
    /// is the directory name.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// assert_eq!(Some("bin"), VirtualPath::new("/usr/bin/").file_name());
    /// assert_eq!(Some("foo.txt"), VirtualPath::new("tmp/foo.txt").file_name());
    /// assert_eq!(Some("foo.txt"), VirtualPath::new("foo.txt/").file_name());
    /// assert_eq!(Some("foo.txt"), VirtualPath::new("foo.txt//").file_name());
    /// assert_eq!(None, VirtualPath::new("").file_name());
    /// ```
    #[doc(alias = "basename")]
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        let filename = self.components().next_back().map(Component::as_str);
        filename
    }

    /// Returns a path that, when joined onto `base`, yields `self`.
    ///
    /// # Errors
    ///
    /// If `base` is not a prefix of `self` (i.e., [`starts_with`]
    /// returns `false`), returns [`Err`].
    ///
    /// [`starts_with`]: Path::starts_with
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
    ///
    /// let path = VirtualPath::new("test/haha/foo.txt");
    ///
    /// assert_eq!(path.strip_prefix("test"), Ok(VirtualPath::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("test/"), Ok(VirtualPath::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("test/haha/foo.txt"), Ok(VirtualPath::new("")));
    /// assert_eq!(path.strip_prefix("test/haha/foo.txt/"), Ok(VirtualPath::new("")));
    ///
    /// assert!(path.strip_prefix("haha").is_err());
    ///
    /// let prefix = VirtualPathBuf::from("test/");
    /// assert_eq!(path.strip_prefix(prefix), Ok(VirtualPath::new("haha/foo.txt")));
    /// ```
    pub fn strip_prefix<P>(&self, base: P) -> Result<&VirtualPath, StripPrefixError>
    where
        P: AsRef<VirtualPath>,
    {
        self._strip_prefix(base.as_ref())
    }

    fn _strip_prefix(&self, base: &VirtualPath) -> Result<&VirtualPath, StripPrefixError> {
        iter_after(self.components(), base.components())
            .map(|c| c.as_path())
            .ok_or(StripPrefixError(()))
    }

    /// Determines whether `base` is a prefix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// let path = VirtualPath::new("etc/passwd");
    ///
    /// assert!(path.starts_with("etc"));
    /// assert!(path.starts_with("etc/"));
    /// assert!(path.starts_with("etc/passwd"));
    /// assert!(path.starts_with("etc/passwd/")); // extra slash is okay
    /// assert!(path.starts_with("etc/passwd///")); // multiple extra slashes are okay
    ///
    /// assert!(!path.starts_with("e"));
    /// assert!(!path.starts_with("etc/passwd.txt"));
    ///
    /// assert!(!VirtualPath::new("etc/foo.rs").starts_with("etc/foo"));
    /// ```
    #[must_use]
    pub fn starts_with<P: AsRef<VirtualPath>>(&self, base: P) -> bool {
        self._starts_with(base.as_ref())
    }

    fn _starts_with(&self, base: &VirtualPath) -> bool {
        iter_after(self.components(), base.components()).is_some()
    }

    /// Determines whether `child` is a suffix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// let path = VirtualPath::new("etc/resolv.conf");
    ///
    /// assert!(path.ends_with("resolv.conf"));
    /// assert!(path.ends_with("etc/resolv.conf"));
    /// assert!(path.ends_with("etc/resolv.conf"));
    ///
    /// assert!(!path.ends_with("tc/resolv.conf"));
    /// assert!(!path.ends_with("conf")); // use .extension() instead
    /// ```
    #[must_use]
    pub fn ends_with<P: AsRef<VirtualPath>>(&self, child: P) -> bool {
        self._ends_with(child.as_ref())
    }

    fn _ends_with(&self, child: &VirtualPath) -> bool {
        iter_after(self.components().rev(), child.components().rev()).is_some()
    }

    /// Extracts the stem (non-extension) portion of [`self.file_name`].
    ///
    /// [`self.file_name`]: Path::file_name
    ///
    /// The stem is:
    ///
    /// * [`None`], if there is no file name;
    /// * The entire file name if there is no embedded `.`;
    /// * The entire file name if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name before the final `.`
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// assert_eq!(Some("foo"), VirtualPath::new("foo.rs").file_stem());
    /// assert_eq!(Some("foo.tar"), VirtualPath::new("foo.tar.gz").file_stem());
    /// assert_eq!(Some(".foo"), VirtualPath::new(".foo").file_stem());
    /// assert_eq!(Some(".foo"), VirtualPath::new(".foo.rs").file_stem());
    /// ```
    #[must_use]
    pub fn file_stem(&self) -> Option<&str> {
        self.file_name()
            .map(rsplit_file_at_dot)
            .and_then(|(before, after)| before.or(after))
    }

    /// Extracts the extension (without the leading dot) of [`self.file_name`], if possible.
    ///
    /// The extension is:
    ///
    /// * [`None`], if there is no file name;
    /// * [`None`], if there is no embedded `.`;
    /// * [`None`], if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name after the final `.`
    ///
    /// [`self.file_name`]: VirtualPath::file_name
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::VirtualPath;
    ///
    /// assert_eq!("rs", VirtualPath::new("foo.rs").extension().unwrap());
    /// assert_eq!("gz", VirtualPath::new("foo.tar.gz").extension().unwrap());
    /// ```
    #[must_use]
    pub fn extension(&self) -> Option<&str> {
        self.file_name()
            .map(rsplit_file_at_dot)
            .and_then(|(before, after)| before.and(after))
    }

    /// Creates an owned [`VirtualPathBuf`] with `path` adjoined to `self`.
    ///
    /// If `path` is absolute, it replaces the current path.
    ///
    /// See [`VirtualPathBuf::push`] for more details on what it means to adjoin a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
    ///
    /// assert_eq!(VirtualPath::new("etc").join("passwd"), VirtualPathBuf::from("etc/passwd"));
    /// assert_eq!(VirtualPath::new("etc").join("/bin/sh"), VirtualPathBuf::from("etc/bin/sh"));
    /// ```
    #[must_use]
    pub fn join<P: AsRef<VirtualPath>>(&self, path: P) -> VirtualPathBuf {
        self._join(path.as_ref())
    }

    fn _join(&self, path: &VirtualPath) -> VirtualPathBuf {
        let mut buf = self.to_path_buf();
        buf.push(path);
        buf.check_path();
        buf
    }

    /// Creates an owned [`VirtualPathBuf`] like `self` but with the given file name.
    ///
    /// See [`VirtualPathBuf::set_file_name`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
    ///
    /// let path = VirtualPath::new("tmp/foo.png");
    /// assert_eq!(path.with_file_name("bar"), VirtualPathBuf::from("tmp/bar"));
    /// assert_eq!(path.with_file_name("bar.txt"), VirtualPathBuf::from("tmp/bar.txt"));
    ///
    /// let path = VirtualPath::new("tmp");
    /// assert_eq!(path.with_file_name("var"), VirtualPathBuf::from("var"));
    /// ```
    #[must_use]
    pub fn with_file_name<S: AsRef<str>>(&self, file_name: S) -> VirtualPathBuf {
        self._with_file_name(file_name.as_ref())
    }

    fn _with_file_name(&self, file_name: &str) -> VirtualPathBuf {
        let mut buf = self.to_path_buf();
        buf.set_file_name(file_name);
        buf.check_path();
        buf
    }

    /// Creates an owned [`VirtualPathBuf`] like `self` but with the given extension.
    ///
    /// See [`VirtualPathBuf::set_extension`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
    ///
    /// let path = VirtualPath::new("foo.rs");
    /// assert_eq!(path.with_extension("txt"), VirtualPathBuf::from("foo.txt"));
    ///
    /// let path = VirtualPath::new("foo.tar.gz");
    /// assert_eq!(path.with_extension(""), VirtualPathBuf::from("foo.tar"));
    /// assert_eq!(path.with_extension("xz"), VirtualPathBuf::from("foo.tar.xz"));
    /// assert_eq!(path.with_extension("").with_extension("txt"), VirtualPathBuf::from("foo.txt"));
    /// ```
    pub fn with_extension<S: AsRef<str>>(&self, extension: S) -> VirtualPathBuf {
        self._with_extension(extension.as_ref())
    }

    fn _with_extension(&self, extension: &str) -> VirtualPathBuf {
        let self_len = self.inner.len();
        // Get the old and new extension lengths, including the dot
        let old_ext_len = self
            .extension()
            .map(str::len)
            .map(|u| u + 1)
            .unwrap_or_default();
        let mut new_ext_len = extension.len();
        if new_ext_len != 0 {
            new_ext_len += 1;
        }

        let mut new_capacity = self_len;
        // remove the capacity of the old extension, if any
        new_capacity -= old_ext_len;
        // add capacity for the new extension, if any
        new_capacity += new_ext_len;

        let mut new_path = VirtualPathBuf::with_capacity(new_capacity);
        // push the existing path, excluding the old extension and dot
        new_path
            .inner
            .push_str(&self.inner[..self_len - old_ext_len]);
        if new_ext_len != 0 {
            new_path.inner.push('.');
            new_path.inner.push_str(extension);
        }

        new_path.check_path();
        new_path
    }

    /// Produces an iterator over the [`Component`]s of the path.
    ///
    /// When parsing the path, there is a small amount of normalization:
    ///
    /// * Repeated separators are ignored, so `a/b` and `a//b` both have
    ///   `a` and `b` as components.
    ///
    /// * A trailing slash is normalized away, `/a/b` and `/a/b/` are equivalent.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::{VirtualPath, Component};
    ///
    /// let mut components = VirtualPath::new("tmp//foo.txt/").components();
    ///
    /// assert_eq!(components.next().map(Component::as_str), Some("tmp"));
    /// assert_eq!(components.next().map(Component::as_str), Some("foo.txt"));
    /// assert_eq!(components.next().map(Component::as_str), None)
    /// ```
    pub fn components(&self) -> Components<'_> {
        Components {
            path: self.as_str(),
            front: State::StartDir,
            back: State::Body,
        }
    }

    /// Produces an iterator over the path's components viewed as [`str`]
    /// slices.
    ///
    /// For more information about the particulars of how the path is separated
    /// into components, see [`components`].
    ///
    /// [`components`]: VirtualPath::components
    ///
    /// # Examples
    ///
    /// ```
    /// use dotstar_toolkit_utils::vfs::{path, VirtualPath};
    ///
    /// let mut it = VirtualPath::new("tmp/foo.txt").iter();
    /// assert_eq!(it.next(), Some("tmp"));
    /// assert_eq!(it.next(), Some("foo.txt"));
    /// assert_eq!(it.next(), None)
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.components(),
        }
    }

    /// Converts a [`Box<VirtualPath>`](Box) into a [`VirtualPathBuf`] without copying or
    /// allocating.
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_path_buf(self: Box<VirtualPath>) -> VirtualPathBuf {
        let rw = Box::into_raw(self) as *mut str;
        let inner = unsafe { Box::from_raw(rw) };
        VirtualPathBuf {
            inner: String::from(inner),
        }
    }

    /// Clean the path from `//`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
    /// assert_eq!(VirtualPath::new("foo///bar").clean(), VirtualPathBuf::from("foo/bar"));
    /// ```
    #[must_use]
    pub fn clean(&self) -> VirtualPathBuf {
        let mut path = VirtualPathBuf::with_capacity(self.inner.len());

        for comp in self.components() {
            path.push(comp)
        }

        path.check_path();
        path
    }

    /// Checks the path for '..' and '.' and panics if it finds them
    fn check_path(&self) {
        if self.inner.starts_with("../") || self.inner.ends_with("/..") {
            panic!("Path is not allowed to have a relative path (..): {self}");
        } else if self.inner.starts_with("./") || self.inner.ends_with("/.") {
            panic!("Path is not allowed to have a current directory (.): {self}");
        } else if self.inner.contains("/../") {
            panic!("Path is not allowed to have a relative path (..): {self}");
        } else if self.inner.contains("/./") {
            panic!("Path is not allowed to have a current directory (.): {self}");
        }
    }
}

impl Display for VirtualPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl std::fmt::Debug for VirtualPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl PartialEq for VirtualPath {
    #[inline]
    fn eq(&self, other: &VirtualPath) -> bool {
        self.components() == other.components()
    }
}

impl std::hash::Hash for VirtualPath {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        let bytes = self.as_u8_slice();

        let mut component_start = 0;
        let mut bytes_hashed = 0;

        for i in 0..bytes.len() {
            let is_sep = bytes[i] == b'/';
            if is_sep {
                if i > component_start {
                    let to_hash = &bytes[component_start..i];
                    h.write(to_hash);
                    bytes_hashed += to_hash.len();
                }

                // skip over separator and optionally a following CurDir item
                // since components() would normalize these away.
                component_start = i + 1;

                let tail = &bytes[component_start..];

                component_start += match tail {
                    [b'.'] => 1,
                    [b'.', sep, ..] if *sep == b'/' => 1,
                    _ => 0,
                };
            }
        }

        if component_start < bytes.len() {
            let to_hash = &bytes[component_start..];
            h.write(to_hash);
            bytes_hashed += to_hash.len();
        }

        h.write_usize(bytes_hashed);
    }
}

impl Eq for VirtualPath {}

impl PartialOrd for VirtualPath {
    #[inline]
    fn partial_cmp(&self, other: &VirtualPath) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VirtualPath {
    #[inline]
    fn cmp(&self, other: &VirtualPath) -> cmp::Ordering {
        compare_components(self.components(), other.components())
    }
}

impl AsRef<VirtualPath> for VirtualPath {
    #[inline]
    fn as_ref(&self) -> &VirtualPath {
        self
    }
}

impl AsRef<str> for VirtualPath {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl AsRef<VirtualPath> for str {
    #[inline]
    fn as_ref(&self) -> &VirtualPath {
        let path = VirtualPath::new(self);
        path.check_path();
        path
    }
}

impl AsRef<VirtualPath> for String {
    #[inline]
    fn as_ref(&self) -> &VirtualPath {
        VirtualPath::new(self)
    }
}

impl AsRef<VirtualPath> for VirtualPathBuf {
    #[inline]
    fn as_ref(&self) -> &VirtualPath {
        self
    }
}

impl AsRef<std::ffi::OsStr> for VirtualPath {
    #[inline]
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.inner.as_ref()
    }
}

impl TryFrom<&Path> for VirtualPathBuf {
    type Error = Utf8Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        #[cfg(target_os = "windows")]
        {
            let mut new_path = Self::with_capacity(value.as_os_str().len());
            for component in value.components() {
                match component {
                    std::path::Component::Prefix(_) => panic!("Prefix not allowed in path"),
                    std::path::Component::RootDir => new_path.push("/"),
                    std::path::Component::CurDir => {
                        panic!("Current working directory not allowed in path (.)")
                    }
                    std::path::Component::ParentDir => panic!("Relative path not allowed (..)"),
                    std::path::Component::Normal(comp) => {
                        new_path.push(TryInto::<&str>::try_into(comp)?)
                    }
                }
            }
            Ok(new_path)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let string: &str = value.as_os_str().try_into()?;
            let path = Self::from(string);
            path.check_path();
            Ok(path)
        }
    }
}

impl<'a> IntoIterator for &'a VirtualPathBuf {
    type Item = &'a str;
    type IntoIter = Iter<'a>;
    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a VirtualPath {
    type Item = &'a str;
    type IntoIter = Iter<'a>;
    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

macro_rules! impl_cmp {
    (<$($life:lifetime),*> $lhs:ty, $rhs: ty) => {
        impl<$($life),*> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <VirtualPath as PartialEq>::eq(self, other)
            }
        }

        impl<$($life),*> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <VirtualPath as PartialEq>::eq(self, other)
            }
        }

        impl<$($life),*> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<cmp::Ordering> {
                <VirtualPath as PartialOrd>::partial_cmp(self, other)
            }
        }

        impl<$($life),*> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<cmp::Ordering> {
                <VirtualPath as PartialOrd>::partial_cmp(self, other)
            }
        }
    };
}

impl_cmp!(<> VirtualPathBuf, VirtualPath);
impl_cmp!(<'a> VirtualPathBuf, &'a VirtualPath);
impl_cmp!(<'a> Cow<'a, VirtualPath>, VirtualPath);
impl_cmp!(<'a, 'b> Cow<'a, VirtualPath>, &'b VirtualPath);
impl_cmp!(<'a> Cow<'a, VirtualPath>, VirtualPathBuf);

macro_rules! impl_cmp_str {
    (<$($life:lifetime),*> $lhs:ty, $rhs: ty) => {
        impl<$($life),*> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <VirtualPath as PartialEq>::eq(self, other.as_ref())
            }
        }

        impl<$($life),*> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <VirtualPath as PartialEq>::eq(self.as_ref(), other)
            }
        }

        impl<$($life),*> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<cmp::Ordering> {
                <VirtualPath as PartialOrd>::partial_cmp(self, other.as_ref())
            }
        }

        impl<$($life),*> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<cmp::Ordering> {
                <VirtualPath as PartialOrd>::partial_cmp(self.as_ref(), other)
            }
        }
    };
}

impl_cmp_str!(<> VirtualPathBuf, str);
impl_cmp_str!(<'a> VirtualPathBuf, &'a str);
impl_cmp_str!(<> VirtualPathBuf, String);
impl_cmp_str!(<> VirtualPath, str);
impl_cmp_str!(<'a> VirtualPath, &'a str);
impl_cmp_str!(<> VirtualPath, String);
impl_cmp_str!(<'a> &'a VirtualPath, str);
impl_cmp_str!(<'a> &'a VirtualPath, String);
