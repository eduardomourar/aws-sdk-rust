// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the current hybrid directory configuration settings for a directory.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HybridSettingsDescription {
    /// <p>The IP addresses of the DNS servers in your self-managed AD environment.</p>
    pub self_managed_dns_ip_addrs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The identifiers of the self-managed instances with SSM used for hybrid directory operations.</p>
    pub self_managed_instance_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl HybridSettingsDescription {
    /// <p>The IP addresses of the DNS servers in your self-managed AD environment.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.self_managed_dns_ip_addrs.is_none()`.
    pub fn self_managed_dns_ip_addrs(&self) -> &[::std::string::String] {
        self.self_managed_dns_ip_addrs.as_deref().unwrap_or_default()
    }
    /// <p>The identifiers of the self-managed instances with SSM used for hybrid directory operations.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.self_managed_instance_ids.is_none()`.
    pub fn self_managed_instance_ids(&self) -> &[::std::string::String] {
        self.self_managed_instance_ids.as_deref().unwrap_or_default()
    }
}
impl HybridSettingsDescription {
    /// Creates a new builder-style object to manufacture [`HybridSettingsDescription`](crate::types::HybridSettingsDescription).
    pub fn builder() -> crate::types::builders::HybridSettingsDescriptionBuilder {
        crate::types::builders::HybridSettingsDescriptionBuilder::default()
    }
}

/// A builder for [`HybridSettingsDescription`](crate::types::HybridSettingsDescription).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct HybridSettingsDescriptionBuilder {
    pub(crate) self_managed_dns_ip_addrs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) self_managed_instance_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl HybridSettingsDescriptionBuilder {
    /// Appends an item to `self_managed_dns_ip_addrs`.
    ///
    /// To override the contents of this collection use [`set_self_managed_dns_ip_addrs`](Self::set_self_managed_dns_ip_addrs).
    ///
    /// <p>The IP addresses of the DNS servers in your self-managed AD environment.</p>
    pub fn self_managed_dns_ip_addrs(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.self_managed_dns_ip_addrs.unwrap_or_default();
        v.push(input.into());
        self.self_managed_dns_ip_addrs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IP addresses of the DNS servers in your self-managed AD environment.</p>
    pub fn set_self_managed_dns_ip_addrs(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.self_managed_dns_ip_addrs = input;
        self
    }
    /// <p>The IP addresses of the DNS servers in your self-managed AD environment.</p>
    pub fn get_self_managed_dns_ip_addrs(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.self_managed_dns_ip_addrs
    }
    /// Appends an item to `self_managed_instance_ids`.
    ///
    /// To override the contents of this collection use [`set_self_managed_instance_ids`](Self::set_self_managed_instance_ids).
    ///
    /// <p>The identifiers of the self-managed instances with SSM used for hybrid directory operations.</p>
    pub fn self_managed_instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.self_managed_instance_ids.unwrap_or_default();
        v.push(input.into());
        self.self_managed_instance_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The identifiers of the self-managed instances with SSM used for hybrid directory operations.</p>
    pub fn set_self_managed_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.self_managed_instance_ids = input;
        self
    }
    /// <p>The identifiers of the self-managed instances with SSM used for hybrid directory operations.</p>
    pub fn get_self_managed_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.self_managed_instance_ids
    }
    /// Consumes the builder and constructs a [`HybridSettingsDescription`](crate::types::HybridSettingsDescription).
    pub fn build(self) -> crate::types::HybridSettingsDescription {
        crate::types::HybridSettingsDescription {
            self_managed_dns_ip_addrs: self.self_managed_dns_ip_addrs,
            self_managed_instance_ids: self.self_managed_instance_ids,
        }
    }
}
