// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_key::_create_key_output::CreateKeyOutputBuilder;

pub use crate::operation::create_key::_create_key_input::CreateKeyInputBuilder;

impl crate::operation::create_key::builders::CreateKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_key::CreateKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_key::CreateKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateKey`.
///
/// <p>Creates an API key resource in your Amazon Web Services account, which lets you grant actions for Amazon Location resources to the API key bearer.</p><note>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/location/previous/developerguide/using-apikeys.html">Using API keys</a>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateKeyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_key::builders::CreateKeyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::create_key::CreateKeyOutput, crate::operation::create_key::CreateKeyError>
    for CreateKeyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::create_key::CreateKeyOutput, crate::operation::create_key::CreateKeyError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateKeyFluentBuilder {
    /// Creates a new `CreateKeyFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateKey as a reference.
    pub fn as_input(&self) -> &crate::operation::create_key::builders::CreateKeyInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_key::CreateKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_key::CreateKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_key::CreateKey::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_key::CreateKey::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_key::CreateKeyOutput,
        crate::operation::create_key::CreateKeyError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>A custom name for the API key resource.</p>
    /// <p>Requirements:</p>
    /// <ul>
    /// <li>
    /// <p>Contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods (.), and underscores (_).</p></li>
    /// <li>
    /// <p>Must be a unique API key name.</p></li>
    /// <li>
    /// <p>No spaces allowed. For example, <code>ExampleAPIKey</code>.</p></li>
    /// </ul>
    pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_name(input.into());
        self
    }
    /// <p>A custom name for the API key resource.</p>
    /// <p>Requirements:</p>
    /// <ul>
    /// <li>
    /// <p>Contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods (.), and underscores (_).</p></li>
    /// <li>
    /// <p>Must be a unique API key name.</p></li>
    /// <li>
    /// <p>No spaces allowed. For example, <code>ExampleAPIKey</code>.</p></li>
    /// </ul>
    pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_name(input);
        self
    }
    /// <p>A custom name for the API key resource.</p>
    /// <p>Requirements:</p>
    /// <ul>
    /// <li>
    /// <p>Contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods (.), and underscores (_).</p></li>
    /// <li>
    /// <p>Must be a unique API key name.</p></li>
    /// <li>
    /// <p>No spaces allowed. For example, <code>ExampleAPIKey</code>.</p></li>
    /// </ul>
    pub fn get_key_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_name()
    }
    /// <p>The API key restrictions for the API key resource.</p>
    pub fn restrictions(mut self, input: crate::types::ApiKeyRestrictions) -> Self {
        self.inner = self.inner.restrictions(input);
        self
    }
    /// <p>The API key restrictions for the API key resource.</p>
    pub fn set_restrictions(mut self, input: ::std::option::Option<crate::types::ApiKeyRestrictions>) -> Self {
        self.inner = self.inner.set_restrictions(input);
        self
    }
    /// <p>The API key restrictions for the API key resource.</p>
    pub fn get_restrictions(&self) -> &::std::option::Option<crate::types::ApiKeyRestrictions> {
        self.inner.get_restrictions()
    }
    /// <p>An optional description for the API key resource.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional description for the API key resource.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>An optional description for the API key resource.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The optional timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. One of <code>NoExpiry</code> or <code>ExpireTime</code> must be set.</p>
    pub fn expire_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.expire_time(input);
        self
    }
    /// <p>The optional timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. One of <code>NoExpiry</code> or <code>ExpireTime</code> must be set.</p>
    pub fn set_expire_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_expire_time(input);
        self
    }
    /// <p>The optional timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. One of <code>NoExpiry</code> or <code>ExpireTime</code> must be set.</p>
    pub fn get_expire_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_expire_time()
    }
    /// <p>Optionally set to <code>true</code> to set no expiration time for the API key. One of <code>NoExpiry</code> or <code>ExpireTime</code> must be set.</p>
    pub fn no_expiry(mut self, input: bool) -> Self {
        self.inner = self.inner.no_expiry(input);
        self
    }
    /// <p>Optionally set to <code>true</code> to set no expiration time for the API key. One of <code>NoExpiry</code> or <code>ExpireTime</code> must be set.</p>
    pub fn set_no_expiry(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_no_expiry(input);
        self
    }
    /// <p>Optionally set to <code>true</code> to set no expiration time for the API key. One of <code>NoExpiry</code> or <code>ExpireTime</code> must be set.</p>
    pub fn get_no_expiry(&self) -> &::std::option::Option<bool> {
        self.inner.get_no_expiry()
    }
    ///
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Applies one or more tags to the map resource. A tag is a key-value pair that helps manage, identify, search, and filter your resources by labelling them.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    /// <p>Restrictions:</p>
    /// <ul>
    /// <li>
    /// <p>Maximum 50 tags per resource</p></li>
    /// <li>
    /// <p>Each resource tag must be unique with a maximum of one value.</p></li>
    /// <li>
    /// <p>Maximum key length: 128 Unicode characters in UTF-8</p></li>
    /// <li>
    /// <p>Maximum value length: 256 Unicode characters in UTF-8</p></li>
    /// <li>
    /// <p>Can use alphanumeric characters (A–Z, a–z, 0–9), and the following characters: + - = . _ : / @.</p></li>
    /// <li>
    /// <p>Cannot use "aws:" as a prefix for a key.</p></li>
    /// </ul>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Applies one or more tags to the map resource. A tag is a key-value pair that helps manage, identify, search, and filter your resources by labelling them.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    /// <p>Restrictions:</p>
    /// <ul>
    /// <li>
    /// <p>Maximum 50 tags per resource</p></li>
    /// <li>
    /// <p>Each resource tag must be unique with a maximum of one value.</p></li>
    /// <li>
    /// <p>Maximum key length: 128 Unicode characters in UTF-8</p></li>
    /// <li>
    /// <p>Maximum value length: 256 Unicode characters in UTF-8</p></li>
    /// <li>
    /// <p>Can use alphanumeric characters (A–Z, a–z, 0–9), and the following characters: + - = . _ : / @.</p></li>
    /// <li>
    /// <p>Cannot use "aws:" as a prefix for a key.</p></li>
    /// </ul>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Applies one or more tags to the map resource. A tag is a key-value pair that helps manage, identify, search, and filter your resources by labelling them.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    /// <p>Restrictions:</p>
    /// <ul>
    /// <li>
    /// <p>Maximum 50 tags per resource</p></li>
    /// <li>
    /// <p>Each resource tag must be unique with a maximum of one value.</p></li>
    /// <li>
    /// <p>Maximum key length: 128 Unicode characters in UTF-8</p></li>
    /// <li>
    /// <p>Maximum value length: 256 Unicode characters in UTF-8</p></li>
    /// <li>
    /// <p>Can use alphanumeric characters (A–Z, a–z, 0–9), and the following characters: + - = . _ : / @.</p></li>
    /// <li>
    /// <p>Cannot use "aws:" as a prefix for a key.</p></li>
    /// </ul>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
