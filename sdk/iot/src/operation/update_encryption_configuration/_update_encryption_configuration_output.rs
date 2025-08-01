// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateEncryptionConfigurationOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for UpdateEncryptionConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateEncryptionConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`UpdateEncryptionConfigurationOutput`](crate::operation::update_encryption_configuration::UpdateEncryptionConfigurationOutput).
    pub fn builder() -> crate::operation::update_encryption_configuration::builders::UpdateEncryptionConfigurationOutputBuilder {
        crate::operation::update_encryption_configuration::builders::UpdateEncryptionConfigurationOutputBuilder::default()
    }
}

/// A builder for [`UpdateEncryptionConfigurationOutput`](crate::operation::update_encryption_configuration::UpdateEncryptionConfigurationOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateEncryptionConfigurationOutputBuilder {
    _request_id: Option<String>,
}
impl UpdateEncryptionConfigurationOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateEncryptionConfigurationOutput`](crate::operation::update_encryption_configuration::UpdateEncryptionConfigurationOutput).
    pub fn build(self) -> crate::operation::update_encryption_configuration::UpdateEncryptionConfigurationOutput {
        crate::operation::update_encryption_configuration::UpdateEncryptionConfigurationOutput {
            _request_id: self._request_id,
        }
    }
}
