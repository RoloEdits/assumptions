#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[doc(hidden)]
pub mod hint;

use std::{
    borrow::Cow,
    fmt::{Debug, Display, Formatter},
    panic::Location,
};

use std::assert as invariant;

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
/// # Examples
///
/// ```rust,ignore
/// assume!(!episode.panels.is_empty(), "published episodes must have have at least one panel to be submitted");
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
            let message = format!("`{}`: {}", stringify!($cond), format!($fmt));
            return Err($crate::Assumption::from(message).into());
        }
    };
    ($cond:expr, $fmt:literal, $($arg:tt)+) => {
        if $crate::hint::unlikely(!$cond) {
            let message = format!("`{}`: {}", stringify!($cond), format!($fmt, $($arg)+));
            return Err($crate::Assumption::from(message).into());
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
/// # Examples
///
/// ```rust,ignore
/// assume_eq!(
///     label.as_str(), "view",
///     "label preceding first `em.cnt` on webtoon homepage should be `view`"
/// );
///
/// // Return a concrete error instead:
/// assume_eq!(header.magic, MAGIC, ParseError::BadMagicBytes);
/// ```
#[macro_export]
macro_rules! assume_eq {
    ($left:expr, $right:expr, $fmt:literal $(,)?) => {
        match (&$left, &$right) {
            (left, right) => {
                if $crate::hint::unlikely(left != right) {
                    let message = format!("{}: left = {:?}, right = {:?}", format!($fmt), left, right);
                    return Err($crate::Assumption::from(message).into());
                }
            }
        }
    };
    ($left:expr, $right:expr, $fmt:literal, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left, right) => {
                if $crate::hint::unlikely(left != right) {
                    let message = format!("{}: left = {:?}, right = {:?}", format!($fmt, $($arg)+), left, right);
                    return Err($crate::Assumption::from(message).into());
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
/// # Examples
///
/// ```rust,ignore
/// assume_ne!(old_id, new_id, "regenerated id returned from [external service] should always differ from the original");
/// // Return a concrete error instead:
/// assume_ne!(old_id, new_id, IdError::RegenerationCollision);
/// ```
#[macro_export]
macro_rules! assume_ne {
    ($left:expr, $right:expr, $fmt:literal $(,)?) => {
        match (&$left, &$right) {
            (left, right) => {
                if $crate::hint::unlikely(left == right) {
                    let message = format!("{}: left = {:?}, right = {:?}", format!($fmt), left, right);
                    return Err($crate::Assumption::from(message).into());
                }
            }
        }
    };
    ($left:expr, $right:expr, $fmt:literal, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left, right) => {
                if $crate::hint::unlikely(left == right) {
                    let message = format!("{}: left = {:?}, right = {:?}", format!($fmt, $($arg)+), left, right);
                    return Err($crate::Assumption::from(message).into());
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
/// Supports an optional match guard via `if`.
///
/// # Errors
///
/// Returns early with an [`Assumption`] if the value does not match the
/// pattern or the guard fails. When passed a concrete error value instead of
/// a message, returns that error directly.
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
///     "episode publish year should be 2014 or later as website only started in 2014"
/// );
///
/// // Return a concrete error instead:
/// assume_matches!(value, Json::Object(_), ParseError::ExpectedObject);
/// ```
#[macro_export]
macro_rules! assume_matches {
    ($expr:expr, $pat:pat $(if $guard:expr)?, $fmt:literal $(,)?) => {
        match $expr {
            #[allow(unused_variables)]
            $pat $(if $guard)? => {}
            ref got => {
                let message = format!("{}, got: `{:?}`", format!($fmt), got);
                return Err($crate::Assumption::from(message).into());
            }
        }
    };
    ($expr:expr, $pat:pat $(if $guard:expr)?, $fmt:literal, $($arg:tt)+) => {
        match $expr {
            #[allow(unused_variables)]
            $pat $(if $guard)? => {}
            ref got => {
                let message = format!("{}, got: `{:?}`", format!($fmt, $($arg)+), got);
                return Err($crate::Assumption::from(message).into());
            }
        }
    };
    ($expr:expr, $pat:pat $(if $guard:expr)?, $err:expr $(,)?) => {
        match $expr {
            #[allow(unused_variables)]
            $pat $(if $guard)? => {}
            _ => return Err($err.into()),
        }
    };
}

/// Guards a logical implication: if `$a` is true, then `$b` must also be true.
///
/// Equivalent to `if $a { assume!($b, ...) }`. Use this when an assumption
/// only applies conditionally: when one fact being true requires another to
/// also hold.
///
/// # Errors
///
/// Returns early with an [`Assumption`] if `$a` is true and `$b` is false.
/// Has no effect if `$a` is false.
///
/// # Examples
///
/// ```rust,ignore
/// // If the episode is published, it must have a publish date.
/// assume_implies!(
///     episode.is_published() => episode.published_at.is_some(),
///     "a published episode should always have a corresponding publish date"
/// );
/// ```
#[macro_export]
macro_rules! assume_implies {
    ($a:expr => $b:expr, $fmt:literal $(,)?) => {
        if $a {
            $crate::assume!($b, $fmt);
        }
    };
    ($a:expr => $b:expr, $fmt:literal, $($arg:tt)+) => {
        if $a {
            $crate::assume!($b, $fmt, $($arg)+);
        }
    };
    ($a:expr => $b:expr, $err:expr $(,)?) => {
        if $a {
            $crate::assume!($b, $err);
        }
    };
}

/// Unconditionally returns an error.
///
/// Use this when you've already determined something is wrong. The error-returning
/// equivalent of `anyhow::bail!`.
///
/// # Errors
///
/// Always returns early with an [`Assumption`]. When passed a concrete error
/// value instead of a message, returns that error directly.
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
///     other => assumption!(ParseError::UnknownTag(other)),
/// }
/// ```
#[macro_export]
macro_rules! assumption {
    ($fmt:literal $(,)?) => {
        return Err($crate::Assumption::from(format!($fmt)).into())
    };
    ($fmt:literal, $($arg:tt)+) => {
        return Err($crate::Assumption::from(format!($fmt, $($arg)+)).into())
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
/// If you encounter this error in the wild, please open an issue for the crate
/// where this was from, with the full output of the message.
#[repr(transparent)]
pub struct Assumption {
    repr: Box<Repr>,
}

struct Repr {
    message: Cow<'static, str>,
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
    pub fn new(message: Cow<'static, str>) -> Self {
        const { invariant!(size_of::<Self>() == size_of::<usize>()) }

        let repr = Repr {
            message,
            location: Location::caller(),
        };

        Self {
            repr: Box::new(repr),
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
        let assumption = self.repr.as_ref();
        &assumption.message
    }

    /// The source location where the assumption was checked.
    ///
    /// Always points at the [`assume!`], [`assumption!`], or [`Assume`] call
    /// site in user code, captured via `#[track_caller]`.
    #[must_use]
    pub fn location(&self) -> &Location<'static> {
        let assumption = self.repr.as_ref();
        assumption.location
    }
}

impl Debug for Assumption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let assumption = self.repr.as_ref();
        f.debug_struct("Assumption")
            .field("message", &assumption.message)
            .field("location", &assumption.location)
            .finish()
    }
}

impl Display for Assumption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let assumption = self.repr.as_ref();
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
/// when the message needs formatting.
///
/// For `Result`, the original error's display output is appended to the
/// message automatically.
///
/// # Examples
///
/// ```rust,ignore
/// let panel = episode.panels.first()
///     .assumption("published episode must have at least one panel in order to be published")?;
///
/// let tag = page.find_tag(name)
///     .with_assumption(|| format!("webtoon homepage should contain a `{name}` tag for the title"))?;
/// ```
#[expect(clippy::missing_errors_doc)]
pub trait Assume<T>: private::Sealed {
    #[track_caller]
    fn assumption(self, assumption: &'static str) -> Result<T, Assumption>;

    #[track_caller]
    fn with_assumption(self, assumption: impl FnOnce() -> String) -> Result<T, Assumption>;
}

impl<T> Assume<T> for Option<T> {
    #[inline]
    #[track_caller]
    fn assumption(self, assumption: &'static str) -> Result<T, Assumption> {
        let option = self;
        #[expect(
            clippy::option_if_let_else,
            reason = "for `track_caller` to work properly, no closures can be used"
        )]
        match option {
            Some(some) => Ok(some),
            None => Err(Assumption::from(String::from(assumption))),
        }
    }

    #[inline]
    #[track_caller]
    fn with_assumption(self, assumption: impl FnOnce() -> String) -> Result<T, Assumption> {
        let option = self;
        #[expect(
            clippy::option_if_let_else,
            reason = "for `track_caller` to work properly, no closures can be used"
        )]
        match option {
            Some(some) => Ok(some),
            None => Err(Assumption::from(assumption())),
        }
    }
}

impl<T, E: Display> Assume<T> for Result<T, E> {
    #[inline]
    #[track_caller]
    fn assumption(self, assumption: &'static str) -> Result<T, Assumption> {
        let result = self;
        match result {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Assumption::from(format!("{assumption}: {err}"))),
        }
    }

    #[inline]
    #[track_caller]
    fn with_assumption(self, assumption: impl FnOnce() -> String) -> Result<T, Assumption> {
        let result = self;
        match result {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Assumption::from(format!("{}: {err}", assumption()))),
        }
    }
}

impl From<String> for Assumption {
    #[inline]
    #[track_caller]
    fn from(assumption: String) -> Self {
        Self::new(Cow::Owned(assumption))
    }
}

impl From<&'static str> for Assumption {
    #[inline]
    #[track_caller]
    fn from(assumption: &'static str) -> Self {
        Self::new(Cow::Borrowed(assumption))
    }
}

mod private {
    pub trait Sealed {}
    impl<T> Sealed for Option<T> {}
    impl<T, E> Sealed for Result<T, E> {}
}
