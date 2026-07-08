#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[doc(hidden)]
pub mod hint;

use std::{
    fmt::{Display, Formatter},
    panic::Location,
};

/// Guards an invariant, returning an error if violated.
///
/// The runtime equivalent of `assert!`. Checks a boolean condition and
/// returns early with an error if it's false, instead of panicking.
///
/// # Errors
///
/// Returns early with an [`Assumption`] if the condition is false. When passed
/// a concrete error value instead of a message, returns that error directly.
///
/// # Recommended Message Style
///
/// Phrase the message as if it completes the sentence "assume that...".
///
/// ```rust,ignore
/// // "assume that episode has at least one panel"
/// assume!(!episode.panels.is_empty(), "episode should have at least one panel");
/// // Avoid: describes the failure rather than the assumption.
/// assume!(!episode.panels.is_empty(), "panels list is empty");
/// ```
///
/// For more detail see [Recommended Message Style](crate#recommended-message-style).
///
/// # Examples
///
/// ```rust,ignore
/// assume!(!episode.panels.is_empty(), "episode should have at least one panel");
///
/// assume!(
///     date.ends_with("GMT"),
///     "all known date formats end with `GMT`"
/// );
///
/// // Return a concrete error instead:
/// assume!(!episode.panels.is_empty(), WebtoonError::EmptyEpisode);
/// ```
#[macro_export]
macro_rules! assume {
    ($cond:expr, $fmt:literal $(,)?) => {
        if $crate::hint::unlikely(!$cond) {
            return Err($crate::Assumption::new(format!(
                "`{}`: {}", stringify!($cond), format!($fmt)
            )).into());
        }
    };
    ($cond:expr, $fmt:literal, $($arg:tt)+) => {
        if $crate::hint::unlikely(!$cond) {
            return Err($crate::Assumption::new(format!(
                "`{}`: {}", stringify!($cond), format!($fmt, $($arg)+)
            )).into());
        }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if $crate::hint::unlikely(!$cond) {
            return Err($err.into());
        }
    };
}

/// Guards that two values are equal, returning an error if not.
///
/// The runtime equivalent of `assert_eq!`, but returns an error instead of
/// panicking. On failure, both values are automatically included in the message
/// as `left = {:?}, right = {:?}`. Requires `Debug` on both sides.
///
/// # Errors
///
/// Returns early with an [`Assumption`] if the values are not equal. When
/// passed a concrete error value instead of a message, returns that error
/// directly.
///
/// # Recommended Message Style
///
/// Phrase the message as if it completes the sentence "assume that...". The
/// values themselves are included automatically, so the message should state
/// the invariant, not the mismatch.
///
/// ```rust,ignore
/// // "assume that label preceding `em.cnt` should be `view`"
/// assume_eq!(label.as_str(), "view", "label preceding `em.cnt` should be `view`");
/// // Avoid: describes the failure rather than the assumption.
/// assume_eq!(label.as_str(), "view", "label wasn't `view`");
/// ```
///
/// For more detail see [Recommended Message Style](crate#recommended-message-style).
///
/// # Examples
///
/// ```rust,ignore
/// assume_eq!(
///     label.as_str(), "view",
///     "label preceding first `em.cnt` on webtoon homepage should be `view`"
/// );
///
/// // Return a concrete error instead:
/// assume_eq!(header.magic, EXPECTED_MAGIC, ParseError::BadMagicBytes);
/// ```
#[macro_export]
macro_rules! assume_eq {
    ($left:expr, $right:expr, $fmt:literal $(,)?) => {
        match (&$left, &$right) {
            (left, right) => {
                if $crate::hint::unlikely(left != right) {
                    return Err($crate::Assumption::new(format!(
                        "{}: left = {:?}, right = {:?}", format!($fmt), left, right
                    )).into());
                }
            }
        }
    };
    ($left:expr, $right:expr, $fmt:literal, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left, right) => {
                if $crate::hint::unlikely(left != right) {
                    return Err($crate::Assumption::new(format!(
                        "{}: left = {:?}, right = {:?}", format!($fmt, $($arg)+), left, right
                    )).into());
                }
            }
        }
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        match (&$left, &$right) {
            (left, right) => {
                if $crate::hint::unlikely(left != right) {
                    return Err($err.into());
                }
            }
        }
    };
}

/// Guards that two values are not equal, returning an error if they are.
///
/// The inverse of [`assume_eq!`]. On failure, both values are automatically
/// included in the message as `left = {:?}, right = {:?}`. Requires `Debug`
/// on both sides.
///
/// # Errors
///
/// Returns early with an [`Assumption`] if the values are equal. When passed
/// a concrete error value instead of a message, returns that error directly.
///
/// # Recommended Message Style
///
/// Phrase the message as if it completes the sentence "assume that...".
///
/// ```rust,ignore
/// // "assume that regenerated id should differ from the original"
/// assume_ne!(old_id, new_id, "regenerated id should differ from the original");
/// ```
///
/// For more detail see [Recommended Message Style](crate#recommended-message-style).
///
/// # Examples
///
/// ```rust,ignore
/// assume_ne!(old_id, new_id, "regenerated id should differ from the original");
///
/// // Return a concrete error instead:
/// assume_ne!(old_id, new_id, IdError::RegenerationCollision);
/// ```
#[macro_export]
macro_rules! assume_ne {
    ($left:expr, $right:expr, $fmt:literal $(,)?) => {
        match (&$left, &$right) {
            (left, right) => {
                if $crate::hint::unlikely(left == right) {
                    return Err($crate::Assumption::new(format!(
                        "{}: left = {:?}, right = {:?}", format!($fmt), left, right
                    )).into());
                }
            }
        }
    };
    ($left:expr, $right:expr, $fmt:literal, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left, right) => {
                if $crate::hint::unlikely(left == right) {
                    return Err($crate::Assumption::new(format!(
                        "{}: left = {:?}, right = {:?}", format!($fmt, $($arg)+), left, right
                    )).into());
                }
            }
        }
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        match (&$left, &$right) {
            (left, right) => {
                if $crate::hint::unlikely(left == right) {
                    return Err($err.into());
                }
            }
        }
    };
}
/// Guards a pattern match, returning an error if the value doesn't match.
///
/// The runtime equivalent of `assert_matches!`, but returns an error instead
/// of panicking. On failure, the actual value is automatically included via
/// `{:?}`. Requires `Debug` on the expression's type.
///
/// Does not bind variables from the pattern into the surrounding scope. If
/// you need the matched value afterward, re-match it explicitly once the
/// assumption holds. Supports an optional match guard via `if`.
///
/// # Errors
///
/// Returns early with an [`Assumption`] if the value does not match the
/// pattern or the guard fails. When passed a concrete error value instead of
/// a message, returns that error directly.
///
/// # Recommended Message Style
///
/// Phrase the message as if it completes the sentence "assume that...". The
/// actual value is included automatically, so the message should state the
/// invariant, not the mismatch.
///
/// ```rust,ignore
/// // "assume that post should be published before indexing"
/// assume_matches!(status, Status::Published, "post should be published before indexing");
/// // Avoid: describes the failure rather than the assumption.
/// assume_matches!(status, Status::Published, "status wasn't Published");
/// ```
///
/// For more detail see [Recommended Message Style](crate#recommended-message-style).
///
/// # Examples
///
/// ```rust,ignore
/// assume_matches!(status, Status::Published, "post should be published before indexing");
///
/// // With a guard:
/// assume_matches!(
///     episode.published,
///     Some(date) if date.year() >= 2014,
///     "episode publish year should be 2014 or later"
/// );
///
/// // Need the bound value afterward? Re-match once the assumption holds:
/// assume_matches!(value, Json::Object(_), "top-level response should be a JSON object");
/// let Json::Object(map) = value else { unreachable!() };
///
/// // Return a concrete error instead:
/// assume_matches!(value, Json::Object(_), ParseError::ExpectedObject);
/// ```
#[macro_export]
macro_rules! assume_matches {
    ($expr:expr, $pat:pat $(if $guard:expr)?, $fmt:literal $(,)?) => {
        if $crate::hint::unlikely(!matches!($expr, $pat $(if $guard)?)) {
            return Err($crate::Assumption::new(format!(
                "{}, got: `{:?}`", format!($fmt), &$expr
            )).into());
        }
    };
    ($expr:expr, $pat:pat $(if $guard:expr)?, $fmt:literal, $($arg:tt)+) => {
        if $crate::hint::unlikely(!matches!($expr, $pat $(if $guard)?)) {
            return Err($crate::Assumption::new(format!(
                "{}, got: `{:?}`", format!($fmt, $($arg)+), &$expr
            )).into());
        }
    };
    ($expr:expr, $pat:pat $(if $guard:expr)?, $err:expr $(,)?) => {
        if $crate::hint::unlikely(!matches!($expr, $pat $(if $guard)?)) {
            return Err($err.into());
        }
    };
}

/// Unconditionally returns an error.
///
/// Use this when you've already determined something is wrong. Typically used in
/// the wildcard arm of a `match` where no condition remains to check. The
/// error-returning equivalent of `anyhow::bail!`.
///
/// # Errors
///
/// Always returns early with an [`Assumption`]. When passed a concrete error
/// value instead of a message, returns that error directly.
///
/// # Recommended Message Style
///
/// State the invariant that was expected to hold at this point in the code.
///
/// ```rust,ignore
/// // Good: states what should have been true.
/// assumption!("tag should be `episode` or `season`, got: `{other}`");
/// // Avoid: describes what happened rather than what was assumed.
/// assumption!("unrecognized tag: `{other}`");
/// ```
///
/// For more detail see [Recommended Message Style](crate#recommended-message-style).
///
/// # Examples
///
/// ```rust,ignore
/// match tag.as_str() {
///     "episode" => Ok(Tag::Episode),
///     "season"  => Ok(Tag::Season),
///     other => assumption!(
///         "unrecognized tag, expected `episode` or `season`, got: `{other}`"
///     ),
/// }
///
/// // Return a concrete error instead:
/// match tag.as_str() {
///     "episode" => Ok(Tag::Episode),
///     _         => assumption!(ParseError::UnknownTag),
/// }
/// ```
#[macro_export]
macro_rules! assumption {
    ($fmt:literal $(,)?) => {
        return Err($crate::Assumption::new(format!($fmt)).into())
    };
    ($fmt:literal, $($arg:tt)+) => {
        return Err($crate::Assumption::new(format!($fmt, $($arg)+)).into())
    };
    ($err:expr $(,)?) => {
        return Err($err.into())
    };
}

/// A violated invariant about an external, uncontrolled system.
///
/// Always indicates a bug triggered by an external change, not actionable from
/// user code. See [Where This Fits](crate#where-this-fits) for the full
/// reasoning.
///
/// [`Display`] renders the location and message together as a single line:
///
/// ```text
/// internal assumption violated at src/webtoon/page.rs:42:5: label preceding first `em.cnt` on webtoon homepage should be `view`
/// ```
///
/// If you encounter this error in the wild, the location and message together
/// point directly at the assumption that needs revisiting. Please open an issue
/// with the full output.
#[derive(Debug)]
pub struct Assumption {
    message: Box<str>,
    location: &'static Location<'static>,
}

impl Assumption {
    /// Constructs a new `Assumption` from a message, capturing the caller's
    /// source location.
    ///
    /// Prefer [`assume!`], [`assumption!`], or the [`Assume`] trait methods
    /// over calling this directly. They propagate the call site location
    /// correctly so the captured location always points at your code, not
    /// inside this crate.
    #[must_use]
    #[track_caller]
    pub fn new(message: String) -> Self {
        Self {
            message: message.into_boxed_str(),
            location: Location::caller(),
        }
    }

    /// The invariant message, without the `internal assumption violated at
    /// {location}:` prefix that [`Display`] adds.
    ///
    /// Useful in tests and structured logging where you want to assert on the
    /// message independently of the source location, which shifts whenever the
    /// file is edited.
    #[must_use]
    pub fn message(&self) -> &str {
        let assumption = self;
        &assumption.message
    }

    /// The source location where the assumption was checked.
    ///
    /// Always points at the [`assume!`], [`assumption!`], or [`Assume`] call
    /// site in user code, captured via `#[track_caller]`.
    #[must_use]
    pub const fn location(&self) -> &Location<'static> {
        let assumption = self;
        assumption.location
    }
}

impl Display for Assumption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let assumption = self;
        write!(
            f,
            "internal assumption violated at {}: {}",
            assumption.location, assumption.message
        )
    }
}

impl std::error::Error for Assumption {}

/// Attaches an [`Assumption`] context message to an `Option` or `Result`.
///
/// For converting a value you already have rather than checking a condition
/// inline. The equivalent of `anyhow::Context::context` and
/// `anyhow::Context::with_context`, but producing an [`Assumption`] instead
/// of an `anyhow::Error`.
///
/// Use `.assumption(msg)` for a static message. Use `.with_assumption(|| ...)`
/// when the message needs formatting. The closure only runs on failure.
///
/// For `Result`, the original error's display output is appended to the
/// message automatically.
///
/// # Recommended Message Style
///
/// State the invariant that was expected to hold at this point in the code.
///
/// ```rust,ignore
/// // Good: states what should have been true.
/// episode.panels.first().assumption("episode should have at least one panel")?;
/// // Avoid: describes what happened rather than what was assumed.
/// episode.panels.first().assumption("no panels found")?;
/// ```
///
/// For more detail see [Recommended Message Style](crate#recommended-message-style).
///
/// # Examples
///
/// ```rust,ignore
/// let panel = episode.panels.first()
///     .assumption("episode should have at least one panel")?;
///
/// let tag = page.find_tag(name)
///     .with_assumption(|| format!("page should contain a `{name}` tag"))?;
/// ```
pub trait Assume<T>: private::Sealed {
    type Output;

    /// Converts `self` into a `Result<T, Assumption>`.
    ///
    /// For `Result`, the original error is appended: `"{assumption}: {err}"`.
    #[track_caller]
    fn assumption(self, assumption: &'static str) -> Self::Output;

    /// Like [`assumption`](Assume::assumption), but constructs the message
    /// lazily. The closure only runs if `self` is `None` or `Err`.
    #[track_caller]
    fn with_assumption(self, assumption: impl FnOnce() -> String) -> Self::Output;
}

impl<T> Assume<T> for Option<T> {
    type Output = Result<T, Assumption>;

    #[inline]
    #[track_caller]
    fn assumption(self, assumption: &'static str) -> Self::Output {
        let option = self;
        #[expect(
            clippy::option_if_let_else,
            reason = "for `track_caller` to work properly, no closures can be used"
        )]
        match option {
            Some(some) => Ok(some),
            None => Err(Assumption::new(String::from(assumption))),
        }
    }

    #[inline]
    #[track_caller]
    fn with_assumption(self, assumption: impl FnOnce() -> String) -> Self::Output {
        let option = self;
        #[expect(
            clippy::option_if_let_else,
            reason = "for `track_caller` to work properly, no closures can be used"
        )]
        match option {
            Some(some) => Ok(some),
            None => Err(Assumption::new(assumption())),
        }
    }
}

impl<T, E: Display> Assume<T> for Result<T, E> {
    type Output = Result<T, Assumption>;

    #[inline]
    #[track_caller]
    fn assumption(self, assumption: &'static str) -> Self::Output {
        let result = self;
        match result {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Assumption::new(format!("{assumption}: {err}"))),
        }
    }

    #[inline]
    #[track_caller]
    fn with_assumption(self, assumption: impl FnOnce() -> String) -> Self::Output {
        let result = self;
        match result {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Assumption::new(format!("{}: {err}", assumption()))),
        }
    }
}

impl From<String> for Assumption {
    #[inline]
    #[track_caller]
    fn from(assumption: String) -> Self {
        Self::new(assumption)
    }
}

mod private {
    pub trait Sealed {}
    impl<T> Sealed for Option<T> {}
    impl<T, E> Sealed for Result<T, E> {}
}
