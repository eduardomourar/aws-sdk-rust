// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `TamsGapHandling`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let tamsgaphandling = unimplemented!();
/// match tamsgaphandling {
///     TamsGapHandling::FillWithBlack => { /* ... */ },
///     TamsGapHandling::HoldLastFrame => { /* ... */ },
///     TamsGapHandling::SkipGaps => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `tamsgaphandling` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `TamsGapHandling::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `TamsGapHandling::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `TamsGapHandling::NewFeature` is defined.
/// Specifically, when `tamsgaphandling` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `TamsGapHandling::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
///
/// Specify how MediaConvert handles gaps between media segments in your TAMS source. Gaps can occur in live streams due to network issues or other interruptions. Choose from the following options: * Skip gaps - Default. Skip over gaps and join segments together. This creates a continuous output with no blank frames, but may cause timeline discontinuities. * Fill with black - Insert black frames to fill gaps between segments. This maintains timeline continuity but adds black frames where content is missing. * Hold last frame - Repeat the last frame before a gap until the next segment begins. This maintains visual continuity during gaps.
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub enum TamsGapHandling {
    #[allow(missing_docs)] // documentation missing in model
    FillWithBlack,
    #[allow(missing_docs)] // documentation missing in model
    HoldLastFrame,
    #[allow(missing_docs)] // documentation missing in model
    SkipGaps,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for TamsGapHandling {
    fn from(s: &str) -> Self {
        match s {
            "FILL_WITH_BLACK" => TamsGapHandling::FillWithBlack,
            "HOLD_LAST_FRAME" => TamsGapHandling::HoldLastFrame,
            "SKIP_GAPS" => TamsGapHandling::SkipGaps,
            other => TamsGapHandling::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for TamsGapHandling {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(TamsGapHandling::from(s))
    }
}
impl TamsGapHandling {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            TamsGapHandling::FillWithBlack => "FILL_WITH_BLACK",
            TamsGapHandling::HoldLastFrame => "HOLD_LAST_FRAME",
            TamsGapHandling::SkipGaps => "SKIP_GAPS",
            TamsGapHandling::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["FILL_WITH_BLACK", "HOLD_LAST_FRAME", "SKIP_GAPS"]
    }
}
impl ::std::convert::AsRef<str> for TamsGapHandling {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl TamsGapHandling {
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
impl ::std::fmt::Display for TamsGapHandling {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            TamsGapHandling::FillWithBlack => write!(f, "FILL_WITH_BLACK"),
            TamsGapHandling::HoldLastFrame => write!(f, "HOLD_LAST_FRAME"),
            TamsGapHandling::SkipGaps => write!(f, "SKIP_GAPS"),
            TamsGapHandling::Unknown(value) => write!(f, "{}", value),
        }
    }
}
