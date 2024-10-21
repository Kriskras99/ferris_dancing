/// Tests that two expressions are equal to each other (using [`PartialEq`]).
///
/// This macro returns a [`TestResult`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// This macro has a second form, where extra information can be provided.
///
/// # Examples
///
/// ```
/// let a = 3;
/// let b = 1 + 2;
/// let c = b * 2;
/// test_eq!(a, b)?;
/// println!("{}", test_eq!(a, c, "and b is {}", b));
/// // prints:
/// // [src/main.rs:5:1]: Test failed: a != c: and b is 3
/// // a: 3
/// // c: 6
/// ```
#[macro_export]
macro_rules! test_eq {
    ($left:expr, $right:literal $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right));

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::None))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:literal, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right));

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right));

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:literal, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right));
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_one_ident(message, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:expr, $right:literal, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right));
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 != b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " != ", ::std::stringify!($right));
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
}

/// Tests that two expressions are not equal to each other (using [`PartialEq`]).
///
/// This macro returns a [`TestResult`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// This macro has a second form, where extra information can be provided.
///
/// # Examples
///
/// ```
/// let a = 3;
/// let b = 1 + 2;
/// let c = b * 2;
/// test_ne!(a, c)?;
/// println!("{}", test_ne!(a, b, "and c is {}", c));
/// // prints:
/// // [src/main.rs:5:1]: Test failed: a == b: and c is 6
/// // a: 3
/// // b: 3
/// ```
#[macro_export]
macro_rules! test_ne {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val != right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 == b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " == ", ::std::stringify!($right));

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val != right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 == b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " == ", ::std::stringify!($right));
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
}

/// Tests that the left expression is any of the values in the right expression.
///
/// The right expression can be anything that results in an item that has a `.contains()` function.
/// For example, slices, [`Vec`]s, ranges, ...
///
/// This macro returns a [`TestResult`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// This macro has a second form, where extra information can be provided.
///
/// # Examples
///
/// ```
/// let a = 3;
/// let b = a * 2;
/// test_any!(a, [1, 3, 5, 7])?;
/// println!("{}", test_any!(b, [1, 3, 5, 7], "and a is {}", a));
/// // prints:
/// // [src/main.rs:5:1]: Test failed: ![1, 3, 5, 7].contains(b): and a is 3
/// // b: 6
/// // [1, 3, 5, 7]: [1, 3, 5, 7]
/// ```
#[macro_export]
macro_rules! test_any {
    ($left:expr, $right:literal $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: !", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')');

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::None))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: !", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')');

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:expr, $right:literal, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: !", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')');
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_one_ident(message, ::std::stringify!($left), &*left_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: !", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')');
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
}

/// Tests that the expression is not any of the values in the slice (using `.contains()` on the right expression).
///
/// This macro returns a [`TestResult`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// This macro has a second form, where extra information can be provided.
///
/// # Examples
///
/// ```
/// let a = 3;
/// let b = a * 2;
/// test_any!(b, [1, 3, 5, 7])?;
/// println!("{}", test_any!(a, [1, 3, 5, 7], "and b is {}", b));
/// // prints:
/// // [src/main.rs:5:1]: Test failed: [1, 3, 5, 7].contains(a): and b is 6
/// // a: 3
/// // [1, 3, 5, 7]: [1, 3, 5, 7]
/// ```
#[macro_export]
macro_rules! test_not_any {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if ((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: [5, 10, 15].contains(unk1)"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')');

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if ((right_val).contains(left_val)) {
                    // "[src/main:2:5]: Test failed: ![5, 10, 15].contains(unk1)"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($right), ".contains(", ::std::stringify!($left), ')');
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
}

/// Tests that the left expression is smaller or equal to the right expression (using [`PartialOrd`]).
///
/// This macro returns a [`TestResult`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// This macro has a second form, where extra information can be provided.
///
/// # Examples
///
/// ```
/// let a = 3;
/// let b = 2;
/// let c = b * 2;
/// test_le!(a, c)?;
/// println!("{}", test_le!(a, b, "and c is {}", c));
/// // prints:
/// // [src/main.rs:5:1]: Test failed: a > b: and c is 6
/// // a: 3
/// // b: 2
/// ```
#[macro_export]
macro_rules! test_le {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val <= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 > b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " > ", ::std::stringify!($right));

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val <= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 > b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " > ", ::std::stringify!($right));
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
}

/// Asserts that the left expression is greater or equal to the right expression (using [`PartialOrd`]).
///
/// This macro returns a [`TestResult`] and hints the compiler that the failure
/// case is unlikely to happen.
///
/// This macro has a second form, where extra information can be provided.
///
/// # Examples
///
/// ```
/// let a = 3;
/// let b = 2;
/// let c = b * 2;
/// test_le!(a, b)?;
/// println!("{}", test_le!(a, c, "and b is {}", b));
/// // prints:
/// // [src/main.rs:5:1]: Test failed: a < c: and b is 2
/// // a: 3
/// // c: 4
/// ```
#[macro_export]
macro_rules! test_ge {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val >= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 < b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " < ", ::std::stringify!($right));

                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::None))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val >= right_val) {
                    // "[src/main:2:5]: Test failed: a * 2 < b * 5"
                    let message = ::std::concat!('[', ::std::file!(), ':', ::std::line!(), ':', ::std::column!(), "]: Test failed: ", ::std::stringify!($left), " < ", ::std::stringify!($right));
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    $crate::testing::TestResult::Err($crate::testing::TestError::test_failed_two_idents(message, ::std::stringify!($left), &*left_val, ::std::stringify!($right), &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+))))
                } else {
                    $crate::testing::TestResult::Ok
                }
            }
        }
    }};
}
