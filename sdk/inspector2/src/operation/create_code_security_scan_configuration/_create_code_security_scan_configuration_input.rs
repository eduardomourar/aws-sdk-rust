// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateCodeSecurityScanConfigurationInput {
    /// <p>The name of the scan configuration.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The security level for the scan configuration.</p>
    pub level: ::std::option::Option<crate::types::ConfigurationLevel>,
    /// <p>The configuration settings for the code security scan.</p>
    pub configuration: ::std::option::Option<crate::types::CodeSecurityScanConfiguration>,
    /// <p>The scope settings that define which repositories will be scanned. Include this parameter to create a default scan configuration. Otherwise Amazon Inspector creates a general scan configuration.</p>
    /// <p>A default scan configuration automatically applies to all existing and future projects imported into Amazon Inspector. Use the <code>BatchAssociateCodeSecurityScanConfiguration</code> operation to associate a general scan configuration with projects.</p>
    pub scope_settings: ::std::option::Option<crate::types::ScopeSettings>,
    /// <p>The tags to apply to the scan configuration.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl CreateCodeSecurityScanConfigurationInput {
    /// <p>The name of the scan configuration.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The security level for the scan configuration.</p>
    pub fn level(&self) -> ::std::option::Option<&crate::types::ConfigurationLevel> {
        self.level.as_ref()
    }
    /// <p>The configuration settings for the code security scan.</p>
    pub fn configuration(&self) -> ::std::option::Option<&crate::types::CodeSecurityScanConfiguration> {
        self.configuration.as_ref()
    }
    /// <p>The scope settings that define which repositories will be scanned. Include this parameter to create a default scan configuration. Otherwise Amazon Inspector creates a general scan configuration.</p>
    /// <p>A default scan configuration automatically applies to all existing and future projects imported into Amazon Inspector. Use the <code>BatchAssociateCodeSecurityScanConfiguration</code> operation to associate a general scan configuration with projects.</p>
    pub fn scope_settings(&self) -> ::std::option::Option<&crate::types::ScopeSettings> {
        self.scope_settings.as_ref()
    }
    /// <p>The tags to apply to the scan configuration.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
}
impl CreateCodeSecurityScanConfigurationInput {
    /// Creates a new builder-style object to manufacture [`CreateCodeSecurityScanConfigurationInput`](crate::operation::create_code_security_scan_configuration::CreateCodeSecurityScanConfigurationInput).
    pub fn builder() -> crate::operation::create_code_security_scan_configuration::builders::CreateCodeSecurityScanConfigurationInputBuilder {
        crate::operation::create_code_security_scan_configuration::builders::CreateCodeSecurityScanConfigurationInputBuilder::default()
    }
}

/// A builder for [`CreateCodeSecurityScanConfigurationInput`](crate::operation::create_code_security_scan_configuration::CreateCodeSecurityScanConfigurationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateCodeSecurityScanConfigurationInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) level: ::std::option::Option<crate::types::ConfigurationLevel>,
    pub(crate) configuration: ::std::option::Option<crate::types::CodeSecurityScanConfiguration>,
    pub(crate) scope_settings: ::std::option::Option<crate::types::ScopeSettings>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl CreateCodeSecurityScanConfigurationInputBuilder {
    /// <p>The name of the scan configuration.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the scan configuration.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the scan configuration.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The security level for the scan configuration.</p>
    /// This field is required.
    pub fn level(mut self, input: crate::types::ConfigurationLevel) -> Self {
        self.level = ::std::option::Option::Some(input);
        self
    }
    /// <p>The security level for the scan configuration.</p>
    pub fn set_level(mut self, input: ::std::option::Option<crate::types::ConfigurationLevel>) -> Self {
        self.level = input;
        self
    }
    /// <p>The security level for the scan configuration.</p>
    pub fn get_level(&self) -> &::std::option::Option<crate::types::ConfigurationLevel> {
        &self.level
    }
    /// <p>The configuration settings for the code security scan.</p>
    /// This field is required.
    pub fn configuration(mut self, input: crate::types::CodeSecurityScanConfiguration) -> Self {
        self.configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration settings for the code security scan.</p>
    pub fn set_configuration(mut self, input: ::std::option::Option<crate::types::CodeSecurityScanConfiguration>) -> Self {
        self.configuration = input;
        self
    }
    /// <p>The configuration settings for the code security scan.</p>
    pub fn get_configuration(&self) -> &::std::option::Option<crate::types::CodeSecurityScanConfiguration> {
        &self.configuration
    }
    /// <p>The scope settings that define which repositories will be scanned. Include this parameter to create a default scan configuration. Otherwise Amazon Inspector creates a general scan configuration.</p>
    /// <p>A default scan configuration automatically applies to all existing and future projects imported into Amazon Inspector. Use the <code>BatchAssociateCodeSecurityScanConfiguration</code> operation to associate a general scan configuration with projects.</p>
    pub fn scope_settings(mut self, input: crate::types::ScopeSettings) -> Self {
        self.scope_settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>The scope settings that define which repositories will be scanned. Include this parameter to create a default scan configuration. Otherwise Amazon Inspector creates a general scan configuration.</p>
    /// <p>A default scan configuration automatically applies to all existing and future projects imported into Amazon Inspector. Use the <code>BatchAssociateCodeSecurityScanConfiguration</code> operation to associate a general scan configuration with projects.</p>
    pub fn set_scope_settings(mut self, input: ::std::option::Option<crate::types::ScopeSettings>) -> Self {
        self.scope_settings = input;
        self
    }
    /// <p>The scope settings that define which repositories will be scanned. Include this parameter to create a default scan configuration. Otherwise Amazon Inspector creates a general scan configuration.</p>
    /// <p>A default scan configuration automatically applies to all existing and future projects imported into Amazon Inspector. Use the <code>BatchAssociateCodeSecurityScanConfiguration</code> operation to associate a general scan configuration with projects.</p>
    pub fn get_scope_settings(&self) -> &::std::option::Option<crate::types::ScopeSettings> {
        &self.scope_settings
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to apply to the scan configuration.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags to apply to the scan configuration.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags to apply to the scan configuration.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`CreateCodeSecurityScanConfigurationInput`](crate::operation::create_code_security_scan_configuration::CreateCodeSecurityScanConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_code_security_scan_configuration::CreateCodeSecurityScanConfigurationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_code_security_scan_configuration::CreateCodeSecurityScanConfigurationInput {
                name: self.name,
                level: self.level,
                configuration: self.configuration,
                scope_settings: self.scope_settings,
                tags: self.tags,
            },
        )
    }
}
