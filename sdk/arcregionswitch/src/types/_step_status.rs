// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `StepStatus`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let stepstatus = unimplemented!();
/// match stepstatus {
///     StepStatus::Cancelled => { /* ... */ },
///     StepStatus::Completed => { /* ... */ },
///     StepStatus::Failed => { /* ... */ },
///     StepStatus::NotStarted => { /* ... */ },
///     StepStatus::PendingApproval => { /* ... */ },
///     StepStatus::Running => { /* ... */ },
///     StepStatus::Skipped => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `stepstatus` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `StepStatus::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `StepStatus::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `StepStatus::NewFeature` is defined.
/// Specifically, when `stepstatus` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `StepStatus::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
///
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub enum StepStatus {
    #[allow(missing_docs)] // documentation missing in model
    Cancelled,
    #[allow(missing_docs)] // documentation missing in model
    Completed,
    #[allow(missing_docs)] // documentation missing in model
    Failed,
    #[allow(missing_docs)] // documentation missing in model
    NotStarted,
    #[allow(missing_docs)] // documentation missing in model
    PendingApproval,
    #[allow(missing_docs)] // documentation missing in model
    Running,
    #[allow(missing_docs)] // documentation missing in model
    Skipped,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for StepStatus {
    fn from(s: &str) -> Self {
        match s {
            "canceled" => StepStatus::Cancelled,
            "completed" => StepStatus::Completed,
            "failed" => StepStatus::Failed,
            "notStarted" => StepStatus::NotStarted,
            "pendingApproval" => StepStatus::PendingApproval,
            "running" => StepStatus::Running,
            "skipped" => StepStatus::Skipped,
            other => StepStatus::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for StepStatus {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(StepStatus::from(s))
    }
}
impl StepStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            StepStatus::Cancelled => "canceled",
            StepStatus::Completed => "completed",
            StepStatus::Failed => "failed",
            StepStatus::NotStarted => "notStarted",
            StepStatus::PendingApproval => "pendingApproval",
            StepStatus::Running => "running",
            StepStatus::Skipped => "skipped",
            StepStatus::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["canceled", "completed", "failed", "notStarted", "pendingApproval", "running", "skipped"]
    }
}
impl ::std::convert::AsRef<str> for StepStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl StepStatus {
    /// Parses the enum value while disallowing unknown variants.
    ///
    /// Unknown variants will result in an error.
    pub fn try_parse(value: &str) -> ::std::result::Result<Self, crate::error::UnknownVariantError> {
        match Self::from(value) {
            #[allow(deprecated)]
            Self::Unknown(_) => ::std::result::Result::Err(crate::error::UnknownVariantError::new(value)),
            known => Ok(known),
        }
    }
}
impl ::std::fmt::Display for StepStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            StepStatus::Cancelled => write!(f, "canceled"),
            StepStatus::Completed => write!(f, "completed"),
            StepStatus::Failed => write!(f, "failed"),
            StepStatus::NotStarted => write!(f, "notStarted"),
            StepStatus::PendingApproval => write!(f, "pendingApproval"),
            StepStatus::Running => write!(f, "running"),
            StepStatus::Skipped => write!(f, "skipped"),
            StepStatus::Unknown(value) => write!(f, "{}", value),
        }
    }
}
