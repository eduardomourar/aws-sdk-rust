// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct GetSpaceOutput {
    /// <p>The unique ID of the private re:Post.</p>
    pub space_id: ::std::string::String,
    /// <p>The ARN of the private re:Post.</p>
    pub arn: ::std::string::String,
    /// <p>The name of the private re:Post.</p>
    pub name: ::std::string::String,
    /// <p>The creation or deletion status of the private re:Post.</p>
    pub status: ::std::string::String,
    /// <p>The configuration status of the private re:Post.</p>
    pub configuration_status: crate::types::ConfigurationStatus,
    /// <p>The Identity Center identifier for the Application Instance.</p>
    pub client_id: ::std::string::String,
    /// <p></p>
    pub identity_store_id: ::std::option::Option<::std::string::String>,
    /// <p></p>
    pub application_arn: ::std::option::Option<::std::string::String>,
    /// <p>The description of the private re:Post.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The approval status of the custom subdomain.</p>
    pub vanity_domain_status: crate::types::VanityDomainStatus,
    /// <p>The custom subdomain that you use to access your private re:Post. All custom subdomains must be approved by AWS before use.</p>
    pub vanity_domain: ::std::string::String,
    /// <p>The AWS generated subdomain of the private re:Post</p>
    pub random_domain: ::std::string::String,
    /// <p>The IAM role that grants permissions to the private re:Post to convert unanswered questions into AWS support tickets.</p>
    pub customer_role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The date when the private re:Post was created.</p>
    pub create_date_time: ::aws_smithy_types::DateTime,
    /// <p>The date when the private re:Post was deleted.</p>
    pub delete_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The pricing tier of the private re:Post.</p>
    pub tier: crate::types::TierLevel,
    /// <p>The storage limit of the private re:Post.</p>
    pub storage_limit: i64,
    /// <p>The list of users that are administrators of the private re:Post.</p>
    #[deprecated(note = "This property has been depracted and will be replaced by the roles property.")]
    pub user_admins: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The list of groups that are administrators of the private re:Post.</p>
    #[deprecated(note = "This property has been depracted and will be replaced by the roles property.")]
    pub group_admins: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A map of accessor identifiers and their roles.</p>
    pub roles: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<crate::types::Role>>>,
    /// <p>The custom AWS KMS key ARN that’s used for the AWS KMS encryption.</p>
    pub user_kms_key: ::std::option::Option<::std::string::String>,
    /// <p>The number of users that have onboarded to the private re:Post.</p>
    pub user_count: ::std::option::Option<i32>,
    /// <p>The content size of the private re:Post.</p>
    pub content_size: ::std::option::Option<i64>,
    /// <p></p>
    pub supported_email_domains: ::std::option::Option<crate::types::SupportedEmailDomainsStatus>,
    _request_id: Option<String>,
}
impl GetSpaceOutput {
    /// <p>The unique ID of the private re:Post.</p>
    pub fn space_id(&self) -> &str {
        use std::ops::Deref;
        self.space_id.deref()
    }
    /// <p>The ARN of the private re:Post.</p>
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }
    /// <p>The name of the private re:Post.</p>
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// <p>The creation or deletion status of the private re:Post.</p>
    pub fn status(&self) -> &str {
        use std::ops::Deref;
        self.status.deref()
    }
    /// <p>The configuration status of the private re:Post.</p>
    pub fn configuration_status(&self) -> &crate::types::ConfigurationStatus {
        &self.configuration_status
    }
    /// <p>The Identity Center identifier for the Application Instance.</p>
    pub fn client_id(&self) -> &str {
        use std::ops::Deref;
        self.client_id.deref()
    }
    /// <p></p>
    pub fn identity_store_id(&self) -> ::std::option::Option<&str> {
        self.identity_store_id.as_deref()
    }
    /// <p></p>
    pub fn application_arn(&self) -> ::std::option::Option<&str> {
        self.application_arn.as_deref()
    }
    /// <p>The description of the private re:Post.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The approval status of the custom subdomain.</p>
    pub fn vanity_domain_status(&self) -> &crate::types::VanityDomainStatus {
        &self.vanity_domain_status
    }
    /// <p>The custom subdomain that you use to access your private re:Post. All custom subdomains must be approved by AWS before use.</p>
    pub fn vanity_domain(&self) -> &str {
        use std::ops::Deref;
        self.vanity_domain.deref()
    }
    /// <p>The AWS generated subdomain of the private re:Post</p>
    pub fn random_domain(&self) -> &str {
        use std::ops::Deref;
        self.random_domain.deref()
    }
    /// <p>The IAM role that grants permissions to the private re:Post to convert unanswered questions into AWS support tickets.</p>
    pub fn customer_role_arn(&self) -> ::std::option::Option<&str> {
        self.customer_role_arn.as_deref()
    }
    /// <p>The date when the private re:Post was created.</p>
    pub fn create_date_time(&self) -> &::aws_smithy_types::DateTime {
        &self.create_date_time
    }
    /// <p>The date when the private re:Post was deleted.</p>
    pub fn delete_date_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.delete_date_time.as_ref()
    }
    /// <p>The pricing tier of the private re:Post.</p>
    pub fn tier(&self) -> &crate::types::TierLevel {
        &self.tier
    }
    /// <p>The storage limit of the private re:Post.</p>
    pub fn storage_limit(&self) -> i64 {
        self.storage_limit
    }
    /// <p>The list of users that are administrators of the private re:Post.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.user_admins.is_none()`.
    #[deprecated(note = "This property has been depracted and will be replaced by the roles property.")]
    pub fn user_admins(&self) -> &[::std::string::String] {
        self.user_admins.as_deref().unwrap_or_default()
    }
    /// <p>The list of groups that are administrators of the private re:Post.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.group_admins.is_none()`.
    #[deprecated(note = "This property has been depracted and will be replaced by the roles property.")]
    pub fn group_admins(&self) -> &[::std::string::String] {
        self.group_admins.as_deref().unwrap_or_default()
    }
    /// <p>A map of accessor identifiers and their roles.</p>
    pub fn roles(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::vec::Vec<crate::types::Role>>> {
        self.roles.as_ref()
    }
    /// <p>The custom AWS KMS key ARN that’s used for the AWS KMS encryption.</p>
    pub fn user_kms_key(&self) -> ::std::option::Option<&str> {
        self.user_kms_key.as_deref()
    }
    /// <p>The number of users that have onboarded to the private re:Post.</p>
    pub fn user_count(&self) -> ::std::option::Option<i32> {
        self.user_count
    }
    /// <p>The content size of the private re:Post.</p>
    pub fn content_size(&self) -> ::std::option::Option<i64> {
        self.content_size
    }
    /// <p></p>
    pub fn supported_email_domains(&self) -> ::std::option::Option<&crate::types::SupportedEmailDomainsStatus> {
        self.supported_email_domains.as_ref()
    }
}
impl ::std::fmt::Debug for GetSpaceOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetSpaceOutput");
        formatter.field("space_id", &self.space_id);
        formatter.field("arn", &self.arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("status", &self.status);
        formatter.field("configuration_status", &self.configuration_status);
        formatter.field("client_id", &self.client_id);
        formatter.field("identity_store_id", &self.identity_store_id);
        formatter.field("application_arn", &self.application_arn);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("vanity_domain_status", &self.vanity_domain_status);
        formatter.field("vanity_domain", &self.vanity_domain);
        formatter.field("random_domain", &self.random_domain);
        formatter.field("customer_role_arn", &self.customer_role_arn);
        formatter.field("create_date_time", &self.create_date_time);
        formatter.field("delete_date_time", &self.delete_date_time);
        formatter.field("tier", &self.tier);
        formatter.field("storage_limit", &self.storage_limit);
        formatter.field("user_admins", &self.user_admins);
        formatter.field("group_admins", &self.group_admins);
        formatter.field("roles", &self.roles);
        formatter.field("user_kms_key", &self.user_kms_key);
        formatter.field("user_count", &self.user_count);
        formatter.field("content_size", &self.content_size);
        formatter.field("supported_email_domains", &self.supported_email_domains);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for GetSpaceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetSpaceOutput {
    /// Creates a new builder-style object to manufacture [`GetSpaceOutput`](crate::operation::get_space::GetSpaceOutput).
    pub fn builder() -> crate::operation::get_space::builders::GetSpaceOutputBuilder {
        crate::operation::get_space::builders::GetSpaceOutputBuilder::default()
    }
}

/// A builder for [`GetSpaceOutput`](crate::operation::get_space::GetSpaceOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct GetSpaceOutputBuilder {
    pub(crate) space_id: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) configuration_status: ::std::option::Option<crate::types::ConfigurationStatus>,
    pub(crate) client_id: ::std::option::Option<::std::string::String>,
    pub(crate) identity_store_id: ::std::option::Option<::std::string::String>,
    pub(crate) application_arn: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) vanity_domain_status: ::std::option::Option<crate::types::VanityDomainStatus>,
    pub(crate) vanity_domain: ::std::option::Option<::std::string::String>,
    pub(crate) random_domain: ::std::option::Option<::std::string::String>,
    pub(crate) customer_role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) create_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) delete_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) tier: ::std::option::Option<crate::types::TierLevel>,
    pub(crate) storage_limit: ::std::option::Option<i64>,
    pub(crate) user_admins: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) group_admins: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) roles: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<crate::types::Role>>>,
    pub(crate) user_kms_key: ::std::option::Option<::std::string::String>,
    pub(crate) user_count: ::std::option::Option<i32>,
    pub(crate) content_size: ::std::option::Option<i64>,
    pub(crate) supported_email_domains: ::std::option::Option<crate::types::SupportedEmailDomainsStatus>,
    _request_id: Option<String>,
}
impl GetSpaceOutputBuilder {
    /// <p>The unique ID of the private re:Post.</p>
    /// This field is required.
    pub fn space_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.space_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique ID of the private re:Post.</p>
    pub fn set_space_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.space_id = input;
        self
    }
    /// <p>The unique ID of the private re:Post.</p>
    pub fn get_space_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.space_id
    }
    /// <p>The ARN of the private re:Post.</p>
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the private re:Post.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The ARN of the private re:Post.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The name of the private re:Post.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the private re:Post.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the private re:Post.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The creation or deletion status of the private re:Post.</p>
    /// This field is required.
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The creation or deletion status of the private re:Post.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The creation or deletion status of the private re:Post.</p>
    pub fn get_status(&self) -> &::std::option::Option<::std::string::String> {
        &self.status
    }
    /// <p>The configuration status of the private re:Post.</p>
    /// This field is required.
    pub fn configuration_status(mut self, input: crate::types::ConfigurationStatus) -> Self {
        self.configuration_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration status of the private re:Post.</p>
    pub fn set_configuration_status(mut self, input: ::std::option::Option<crate::types::ConfigurationStatus>) -> Self {
        self.configuration_status = input;
        self
    }
    /// <p>The configuration status of the private re:Post.</p>
    pub fn get_configuration_status(&self) -> &::std::option::Option<crate::types::ConfigurationStatus> {
        &self.configuration_status
    }
    /// <p>The Identity Center identifier for the Application Instance.</p>
    /// This field is required.
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Identity Center identifier for the Application Instance.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_id = input;
        self
    }
    /// <p>The Identity Center identifier for the Application Instance.</p>
    pub fn get_client_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_id
    }
    /// <p></p>
    pub fn identity_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.identity_store_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p></p>
    pub fn set_identity_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.identity_store_id = input;
        self
    }
    /// <p></p>
    pub fn get_identity_store_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.identity_store_id
    }
    /// <p></p>
    pub fn application_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.application_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p></p>
    pub fn set_application_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.application_arn = input;
        self
    }
    /// <p></p>
    pub fn get_application_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.application_arn
    }
    /// <p>The description of the private re:Post.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the private re:Post.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the private re:Post.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The approval status of the custom subdomain.</p>
    /// This field is required.
    pub fn vanity_domain_status(mut self, input: crate::types::VanityDomainStatus) -> Self {
        self.vanity_domain_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The approval status of the custom subdomain.</p>
    pub fn set_vanity_domain_status(mut self, input: ::std::option::Option<crate::types::VanityDomainStatus>) -> Self {
        self.vanity_domain_status = input;
        self
    }
    /// <p>The approval status of the custom subdomain.</p>
    pub fn get_vanity_domain_status(&self) -> &::std::option::Option<crate::types::VanityDomainStatus> {
        &self.vanity_domain_status
    }
    /// <p>The custom subdomain that you use to access your private re:Post. All custom subdomains must be approved by AWS before use.</p>
    /// This field is required.
    pub fn vanity_domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vanity_domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The custom subdomain that you use to access your private re:Post. All custom subdomains must be approved by AWS before use.</p>
    pub fn set_vanity_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vanity_domain = input;
        self
    }
    /// <p>The custom subdomain that you use to access your private re:Post. All custom subdomains must be approved by AWS before use.</p>
    pub fn get_vanity_domain(&self) -> &::std::option::Option<::std::string::String> {
        &self.vanity_domain
    }
    /// <p>The AWS generated subdomain of the private re:Post</p>
    /// This field is required.
    pub fn random_domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.random_domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The AWS generated subdomain of the private re:Post</p>
    pub fn set_random_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.random_domain = input;
        self
    }
    /// <p>The AWS generated subdomain of the private re:Post</p>
    pub fn get_random_domain(&self) -> &::std::option::Option<::std::string::String> {
        &self.random_domain
    }
    /// <p>The IAM role that grants permissions to the private re:Post to convert unanswered questions into AWS support tickets.</p>
    pub fn customer_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.customer_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IAM role that grants permissions to the private re:Post to convert unanswered questions into AWS support tickets.</p>
    pub fn set_customer_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.customer_role_arn = input;
        self
    }
    /// <p>The IAM role that grants permissions to the private re:Post to convert unanswered questions into AWS support tickets.</p>
    pub fn get_customer_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.customer_role_arn
    }
    /// <p>The date when the private re:Post was created.</p>
    /// This field is required.
    pub fn create_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.create_date_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date when the private re:Post was created.</p>
    pub fn set_create_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.create_date_time = input;
        self
    }
    /// <p>The date when the private re:Post was created.</p>
    pub fn get_create_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.create_date_time
    }
    /// <p>The date when the private re:Post was deleted.</p>
    pub fn delete_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.delete_date_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date when the private re:Post was deleted.</p>
    pub fn set_delete_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.delete_date_time = input;
        self
    }
    /// <p>The date when the private re:Post was deleted.</p>
    pub fn get_delete_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.delete_date_time
    }
    /// <p>The pricing tier of the private re:Post.</p>
    /// This field is required.
    pub fn tier(mut self, input: crate::types::TierLevel) -> Self {
        self.tier = ::std::option::Option::Some(input);
        self
    }
    /// <p>The pricing tier of the private re:Post.</p>
    pub fn set_tier(mut self, input: ::std::option::Option<crate::types::TierLevel>) -> Self {
        self.tier = input;
        self
    }
    /// <p>The pricing tier of the private re:Post.</p>
    pub fn get_tier(&self) -> &::std::option::Option<crate::types::TierLevel> {
        &self.tier
    }
    /// <p>The storage limit of the private re:Post.</p>
    /// This field is required.
    pub fn storage_limit(mut self, input: i64) -> Self {
        self.storage_limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The storage limit of the private re:Post.</p>
    pub fn set_storage_limit(mut self, input: ::std::option::Option<i64>) -> Self {
        self.storage_limit = input;
        self
    }
    /// <p>The storage limit of the private re:Post.</p>
    pub fn get_storage_limit(&self) -> &::std::option::Option<i64> {
        &self.storage_limit
    }
    /// Appends an item to `user_admins`.
    ///
    /// To override the contents of this collection use [`set_user_admins`](Self::set_user_admins).
    ///
    /// <p>The list of users that are administrators of the private re:Post.</p>
    #[deprecated(note = "This property has been depracted and will be replaced by the roles property.")]
    pub fn user_admins(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.user_admins.unwrap_or_default();
        v.push(input.into());
        self.user_admins = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of users that are administrators of the private re:Post.</p>
    #[deprecated(note = "This property has been depracted and will be replaced by the roles property.")]
    pub fn set_user_admins(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.user_admins = input;
        self
    }
    /// <p>The list of users that are administrators of the private re:Post.</p>
    #[deprecated(note = "This property has been depracted and will be replaced by the roles property.")]
    pub fn get_user_admins(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.user_admins
    }
    /// Appends an item to `group_admins`.
    ///
    /// To override the contents of this collection use [`set_group_admins`](Self::set_group_admins).
    ///
    /// <p>The list of groups that are administrators of the private re:Post.</p>
    #[deprecated(note = "This property has been depracted and will be replaced by the roles property.")]
    pub fn group_admins(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.group_admins.unwrap_or_default();
        v.push(input.into());
        self.group_admins = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of groups that are administrators of the private re:Post.</p>
    #[deprecated(note = "This property has been depracted and will be replaced by the roles property.")]
    pub fn set_group_admins(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.group_admins = input;
        self
    }
    /// <p>The list of groups that are administrators of the private re:Post.</p>
    #[deprecated(note = "This property has been depracted and will be replaced by the roles property.")]
    pub fn get_group_admins(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.group_admins
    }
    /// Adds a key-value pair to `roles`.
    ///
    /// To override the contents of this collection use [`set_roles`](Self::set_roles).
    ///
    /// <p>A map of accessor identifiers and their roles.</p>
    pub fn roles(mut self, k: impl ::std::convert::Into<::std::string::String>, v: ::std::vec::Vec<crate::types::Role>) -> Self {
        let mut hash_map = self.roles.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.roles = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map of accessor identifiers and their roles.</p>
    pub fn set_roles(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<crate::types::Role>>>,
    ) -> Self {
        self.roles = input;
        self
    }
    /// <p>A map of accessor identifiers and their roles.</p>
    pub fn get_roles(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<crate::types::Role>>> {
        &self.roles
    }
    /// <p>The custom AWS KMS key ARN that’s used for the AWS KMS encryption.</p>
    pub fn user_kms_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_kms_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The custom AWS KMS key ARN that’s used for the AWS KMS encryption.</p>
    pub fn set_user_kms_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_kms_key = input;
        self
    }
    /// <p>The custom AWS KMS key ARN that’s used for the AWS KMS encryption.</p>
    pub fn get_user_kms_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.user_kms_key
    }
    /// <p>The number of users that have onboarded to the private re:Post.</p>
    pub fn user_count(mut self, input: i32) -> Self {
        self.user_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of users that have onboarded to the private re:Post.</p>
    pub fn set_user_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.user_count = input;
        self
    }
    /// <p>The number of users that have onboarded to the private re:Post.</p>
    pub fn get_user_count(&self) -> &::std::option::Option<i32> {
        &self.user_count
    }
    /// <p>The content size of the private re:Post.</p>
    pub fn content_size(mut self, input: i64) -> Self {
        self.content_size = ::std::option::Option::Some(input);
        self
    }
    /// <p>The content size of the private re:Post.</p>
    pub fn set_content_size(mut self, input: ::std::option::Option<i64>) -> Self {
        self.content_size = input;
        self
    }
    /// <p>The content size of the private re:Post.</p>
    pub fn get_content_size(&self) -> &::std::option::Option<i64> {
        &self.content_size
    }
    /// <p></p>
    pub fn supported_email_domains(mut self, input: crate::types::SupportedEmailDomainsStatus) -> Self {
        self.supported_email_domains = ::std::option::Option::Some(input);
        self
    }
    /// <p></p>
    pub fn set_supported_email_domains(mut self, input: ::std::option::Option<crate::types::SupportedEmailDomainsStatus>) -> Self {
        self.supported_email_domains = input;
        self
    }
    /// <p></p>
    pub fn get_supported_email_domains(&self) -> &::std::option::Option<crate::types::SupportedEmailDomainsStatus> {
        &self.supported_email_domains
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetSpaceOutput`](crate::operation::get_space::GetSpaceOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`space_id`](crate::operation::get_space::builders::GetSpaceOutputBuilder::space_id)
    /// - [`arn`](crate::operation::get_space::builders::GetSpaceOutputBuilder::arn)
    /// - [`name`](crate::operation::get_space::builders::GetSpaceOutputBuilder::name)
    /// - [`status`](crate::operation::get_space::builders::GetSpaceOutputBuilder::status)
    /// - [`configuration_status`](crate::operation::get_space::builders::GetSpaceOutputBuilder::configuration_status)
    /// - [`client_id`](crate::operation::get_space::builders::GetSpaceOutputBuilder::client_id)
    /// - [`vanity_domain_status`](crate::operation::get_space::builders::GetSpaceOutputBuilder::vanity_domain_status)
    /// - [`vanity_domain`](crate::operation::get_space::builders::GetSpaceOutputBuilder::vanity_domain)
    /// - [`random_domain`](crate::operation::get_space::builders::GetSpaceOutputBuilder::random_domain)
    /// - [`create_date_time`](crate::operation::get_space::builders::GetSpaceOutputBuilder::create_date_time)
    /// - [`tier`](crate::operation::get_space::builders::GetSpaceOutputBuilder::tier)
    /// - [`storage_limit`](crate::operation::get_space::builders::GetSpaceOutputBuilder::storage_limit)
    pub fn build(self) -> ::std::result::Result<crate::operation::get_space::GetSpaceOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_space::GetSpaceOutput {
            space_id: self.space_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "space_id",
                    "space_id was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            configuration_status: self.configuration_status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "configuration_status",
                    "configuration_status was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            client_id: self.client_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "client_id",
                    "client_id was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            identity_store_id: self.identity_store_id,
            application_arn: self.application_arn,
            description: self.description,
            vanity_domain_status: self.vanity_domain_status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "vanity_domain_status",
                    "vanity_domain_status was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            vanity_domain: self.vanity_domain.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "vanity_domain",
                    "vanity_domain was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            random_domain: self.random_domain.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "random_domain",
                    "random_domain was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            customer_role_arn: self.customer_role_arn,
            create_date_time: self.create_date_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "create_date_time",
                    "create_date_time was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            delete_date_time: self.delete_date_time,
            tier: self.tier.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "tier",
                    "tier was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            storage_limit: self.storage_limit.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "storage_limit",
                    "storage_limit was not specified but it is required when building GetSpaceOutput",
                )
            })?,
            user_admins: self.user_admins,
            group_admins: self.group_admins,
            roles: self.roles,
            user_kms_key: self.user_kms_key,
            user_count: self.user_count,
            content_size: self.content_size,
            supported_email_domains: self.supported_email_domains,
            _request_id: self._request_id,
        })
    }
}
impl ::std::fmt::Debug for GetSpaceOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetSpaceOutputBuilder");
        formatter.field("space_id", &self.space_id);
        formatter.field("arn", &self.arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("status", &self.status);
        formatter.field("configuration_status", &self.configuration_status);
        formatter.field("client_id", &self.client_id);
        formatter.field("identity_store_id", &self.identity_store_id);
        formatter.field("application_arn", &self.application_arn);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("vanity_domain_status", &self.vanity_domain_status);
        formatter.field("vanity_domain", &self.vanity_domain);
        formatter.field("random_domain", &self.random_domain);
        formatter.field("customer_role_arn", &self.customer_role_arn);
        formatter.field("create_date_time", &self.create_date_time);
        formatter.field("delete_date_time", &self.delete_date_time);
        formatter.field("tier", &self.tier);
        formatter.field("storage_limit", &self.storage_limit);
        formatter.field("user_admins", &self.user_admins);
        formatter.field("group_admins", &self.group_admins);
        formatter.field("roles", &self.roles);
        formatter.field("user_kms_key", &self.user_kms_key);
        formatter.field("user_count", &self.user_count);
        formatter.field("content_size", &self.content_size);
        formatter.field("supported_email_domains", &self.supported_email_domains);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
