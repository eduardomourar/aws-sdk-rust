// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains extraction configuration information for a memory strategy.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum ExtractionConfiguration {
    /// <p>The custom extraction configuration.</p>
    CustomExtractionConfiguration(crate::types::CustomExtractionConfiguration),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl ExtractionConfiguration {
    #[allow(irrefutable_let_patterns)]
    /// Tries to convert the enum instance into [`CustomExtractionConfiguration`](crate::types::ExtractionConfiguration::CustomExtractionConfiguration), extracting the inner [`CustomExtractionConfiguration`](crate::types::CustomExtractionConfiguration).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_custom_extraction_configuration(&self) -> ::std::result::Result<&crate::types::CustomExtractionConfiguration, &Self> {
        if let ExtractionConfiguration::CustomExtractionConfiguration(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`CustomExtractionConfiguration`](crate::types::ExtractionConfiguration::CustomExtractionConfiguration).
    pub fn is_custom_extraction_configuration(&self) -> bool {
        self.as_custom_extraction_configuration().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
