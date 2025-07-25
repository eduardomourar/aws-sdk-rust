// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration for a credential provider. This structure defines how the gateway authenticates with the target endpoint.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CredentialProviderConfiguration {
    /// <p>The type of credential provider. This field specifies which authentication method the gateway uses.</p>
    pub credential_provider_type: crate::types::CredentialProviderType,
    /// <p>The credential provider. This field contains the specific configuration for the credential provider type.</p>
    pub credential_provider: ::std::option::Option<crate::types::CredentialProvider>,
}
impl CredentialProviderConfiguration {
    /// <p>The type of credential provider. This field specifies which authentication method the gateway uses.</p>
    pub fn credential_provider_type(&self) -> &crate::types::CredentialProviderType {
        &self.credential_provider_type
    }
    /// <p>The credential provider. This field contains the specific configuration for the credential provider type.</p>
    pub fn credential_provider(&self) -> ::std::option::Option<&crate::types::CredentialProvider> {
        self.credential_provider.as_ref()
    }
}
impl CredentialProviderConfiguration {
    /// Creates a new builder-style object to manufacture [`CredentialProviderConfiguration`](crate::types::CredentialProviderConfiguration).
    pub fn builder() -> crate::types::builders::CredentialProviderConfigurationBuilder {
        crate::types::builders::CredentialProviderConfigurationBuilder::default()
    }
}

/// A builder for [`CredentialProviderConfiguration`](crate::types::CredentialProviderConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CredentialProviderConfigurationBuilder {
    pub(crate) credential_provider_type: ::std::option::Option<crate::types::CredentialProviderType>,
    pub(crate) credential_provider: ::std::option::Option<crate::types::CredentialProvider>,
}
impl CredentialProviderConfigurationBuilder {
    /// <p>The type of credential provider. This field specifies which authentication method the gateway uses.</p>
    /// This field is required.
    pub fn credential_provider_type(mut self, input: crate::types::CredentialProviderType) -> Self {
        self.credential_provider_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of credential provider. This field specifies which authentication method the gateway uses.</p>
    pub fn set_credential_provider_type(mut self, input: ::std::option::Option<crate::types::CredentialProviderType>) -> Self {
        self.credential_provider_type = input;
        self
    }
    /// <p>The type of credential provider. This field specifies which authentication method the gateway uses.</p>
    pub fn get_credential_provider_type(&self) -> &::std::option::Option<crate::types::CredentialProviderType> {
        &self.credential_provider_type
    }
    /// <p>The credential provider. This field contains the specific configuration for the credential provider type.</p>
    pub fn credential_provider(mut self, input: crate::types::CredentialProvider) -> Self {
        self.credential_provider = ::std::option::Option::Some(input);
        self
    }
    /// <p>The credential provider. This field contains the specific configuration for the credential provider type.</p>
    pub fn set_credential_provider(mut self, input: ::std::option::Option<crate::types::CredentialProvider>) -> Self {
        self.credential_provider = input;
        self
    }
    /// <p>The credential provider. This field contains the specific configuration for the credential provider type.</p>
    pub fn get_credential_provider(&self) -> &::std::option::Option<crate::types::CredentialProvider> {
        &self.credential_provider
    }
    /// Consumes the builder and constructs a [`CredentialProviderConfiguration`](crate::types::CredentialProviderConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`credential_provider_type`](crate::types::builders::CredentialProviderConfigurationBuilder::credential_provider_type)
    pub fn build(self) -> ::std::result::Result<crate::types::CredentialProviderConfiguration, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::CredentialProviderConfiguration {
            credential_provider_type: self.credential_provider_type.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "credential_provider_type",
                    "credential_provider_type was not specified but it is required when building CredentialProviderConfiguration",
                )
            })?,
            credential_provider: self.credential_provider,
        })
    }
}
