// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Lightsail throws this exception when an operation is performed on resources in an opt-in Region that is currently being set up.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegionSetupInProgressException {
    #[allow(missing_docs)] // documentation missing in model
    pub code: ::std::option::Option<::std::string::String>,
    /// <p><a href="https://docs.aws.amazon.com/lightsail/latest/userguide/understanding-regions-and-availability-zones-in-amazon-lightsail.html">Regions and Availability Zones for Lightsail</a></p>
    pub docs: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::option::Option<::std::string::String>,
    /// <p>Opt-in Regions typically take a few minutes to finish setting up before you can work with them. Wait a few minutes and try again.</p>
    pub tip: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl RegionSetupInProgressException {
    #[allow(missing_docs)] // documentation missing in model
    pub fn code(&self) -> ::std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p><a href="https://docs.aws.amazon.com/lightsail/latest/userguide/understanding-regions-and-availability-zones-in-amazon-lightsail.html">Regions and Availability Zones for Lightsail</a></p>
    pub fn docs(&self) -> ::std::option::Option<&str> {
        self.docs.as_deref()
    }
    /// <p>Opt-in Regions typically take a few minutes to finish setting up before you can work with them. Wait a few minutes and try again.</p>
    pub fn tip(&self) -> ::std::option::Option<&str> {
        self.tip.as_deref()
    }
}
impl RegionSetupInProgressException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for RegionSetupInProgressException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "RegionSetupInProgressException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for RegionSetupInProgressException {}
impl ::aws_types::request_id::RequestId for crate::types::error::RegionSetupInProgressException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for RegionSetupInProgressException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl RegionSetupInProgressException {
    /// Creates a new builder-style object to manufacture [`RegionSetupInProgressException`](crate::types::error::RegionSetupInProgressException).
    pub fn builder() -> crate::types::error::builders::RegionSetupInProgressExceptionBuilder {
        crate::types::error::builders::RegionSetupInProgressExceptionBuilder::default()
    }
}

/// A builder for [`RegionSetupInProgressException`](crate::types::error::RegionSetupInProgressException).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RegionSetupInProgressExceptionBuilder {
    pub(crate) code: ::std::option::Option<::std::string::String>,
    pub(crate) docs: ::std::option::Option<::std::string::String>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) tip: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl RegionSetupInProgressExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.code
    }
    /// <p><a href="https://docs.aws.amazon.com/lightsail/latest/userguide/understanding-regions-and-availability-zones-in-amazon-lightsail.html">Regions and Availability Zones for Lightsail</a></p>
    pub fn docs(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.docs = ::std::option::Option::Some(input.into());
        self
    }
    /// <p><a href="https://docs.aws.amazon.com/lightsail/latest/userguide/understanding-regions-and-availability-zones-in-amazon-lightsail.html">Regions and Availability Zones for Lightsail</a></p>
    pub fn set_docs(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.docs = input;
        self
    }
    /// <p><a href="https://docs.aws.amazon.com/lightsail/latest/userguide/understanding-regions-and-availability-zones-in-amazon-lightsail.html">Regions and Availability Zones for Lightsail</a></p>
    pub fn get_docs(&self) -> &::std::option::Option<::std::string::String> {
        &self.docs
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// <p>Opt-in Regions typically take a few minutes to finish setting up before you can work with them. Wait a few minutes and try again.</p>
    pub fn tip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.tip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Opt-in Regions typically take a few minutes to finish setting up before you can work with them. Wait a few minutes and try again.</p>
    pub fn set_tip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.tip = input;
        self
    }
    /// <p>Opt-in Regions typically take a few minutes to finish setting up before you can work with them. Wait a few minutes and try again.</p>
    pub fn get_tip(&self) -> &::std::option::Option<::std::string::String> {
        &self.tip
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`RegionSetupInProgressException`](crate::types::error::RegionSetupInProgressException).
    pub fn build(self) -> crate::types::error::RegionSetupInProgressException {
        crate::types::error::RegionSetupInProgressException {
            code: self.code,
            docs: self.docs,
            message: self.message,
            tip: self.tip,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
