//! # Overlay Filesystem
//! Implements a filesystem that overlays two filesystems, preferring the upper filesystem for operations.
use std::{
    borrow::Cow,
    collections::HashSet,
    io::{self, ErrorKind},
    marker::PhantomData,
    path::Path,
};

use super::{VirtualFile, VirtualFileSystem};
use crate::bytes_newer4::read::{ReadError, TrivialClone, ZeroCopyReadAt};

/// A filesystem that overlays two filesystems, preferring the upper filesystem for operations.
pub struct OverlayFs<'fsa, 'fsb, FsA: VirtualFileSystem, FsB: VirtualFileSystem> {
    /// The upper filesystem, checked first for paths
    upper: &'fsa FsA,
    /// The lower filesystem, checked if upper doesn't know about a path
    lower: &'fsb FsB,
}

impl<'fsa, 'fsb, FsA: VirtualFileSystem, FsB: VirtualFileSystem> OverlayFs<'fsa, 'fsb, FsA, FsB> {
    /// Create a new overlay from two filesystems
    pub fn new(upper: &'fsa FsA, lower: &'fsb FsB) -> Self {
        Self { upper, lower }
    }
}

impl<'fsa, 'fsb, FsA: VirtualFileSystem, FsB: VirtualFileSystem> VirtualFileSystem
    for OverlayFs<'fsa, 'fsb, FsA, FsB>
{
    type VirtualFile<'fs> = OverlayFile<'fs, FsA::VirtualFile<'fs>, FsB::VirtualFile<'fs>> where FsA: 'fs, FsB: 'fs, Self: 'fs;

    fn open<'fs>(&'fs self, path: &Path) -> io::Result<Self::VirtualFile<'fs>> {
        if let Ok(file) = self.upper.open(path) {
            Ok(OverlayFile::Upper(file))
        } else if let Ok(file) = self.lower.open(path) {
            Ok(OverlayFile::Lower(file))
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }

    fn list_files<'fs>(&'fs self, path: &Path) -> io::Result<impl Iterator<Item = &'fs Path>> {
        let mut paths = self.upper.list_files(path)?.collect::<HashSet<_>>();
        paths.extend(self.lower.list_files(path)?);
        Ok(paths.into_iter())
    }

    fn exists(&self, path: &Path) -> bool {
        self.upper.exists(path) || self.lower.exists(path)
    }
}

#[derive(Debug, Clone)]
pub enum OverlayFile<'fs, VfA: VirtualFile<'fs>, VfB: VirtualFile<'fs>> {
    Upper(VfA),
    Lower(VfB),
    Phantom(PhantomData<&'fs u8>),
}

impl<'fs, VfA: VirtualFile<'fs>, VfB: VirtualFile<'fs>> TrivialClone
    for OverlayFile<'fs, VfA, VfB>
{
}

impl<'fs, VfA: VirtualFile<'fs>, VfB: VirtualFile<'fs>> ZeroCopyReadAt
    for OverlayFile<'fs, VfA, VfB>
{
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<Cow<'rf, str>, ReadError> {
        match self {
            OverlayFile::Upper(upper) => upper.read_null_terminated_string_at(position),
            OverlayFile::Lower(lower) => lower.read_null_terminated_string_at(position),
            OverlayFile::Phantom(_) => unreachable!(),
        }
    }

    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        match self {
            OverlayFile::Upper(upper) => upper.read_slice_at(position, len),
            OverlayFile::Lower(lower) => lower.read_slice_at(position, len),
            OverlayFile::Phantom(_) => unreachable!(),
        }
    }
}

impl<'fs, VfA: VirtualFile<'fs>, VfB: VirtualFile<'fs>> VirtualFile<'fs>
    for OverlayFile<'fs, VfA, VfB>
{
}
