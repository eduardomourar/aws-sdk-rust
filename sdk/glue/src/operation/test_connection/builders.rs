// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::test_connection::_test_connection_output::TestConnectionOutputBuilder;

pub use crate::operation::test_connection::_test_connection_input::TestConnectionInputBuilder;

impl crate::operation::test_connection::builders::TestConnectionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::test_connection::TestConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::test_connection::TestConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.test_connection();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TestConnection`.
///
/// <p>Tests a connection to a service to validate the service credentials that you provide.</p>
/// <p>You can either provide an existing connection name or a <code>TestConnectionInput</code> for testing a non-existing connection input. Providing both at the same time will cause an error.</p>
/// <p>If the action is successful, the service sends back an HTTP 200 response.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TestConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::test_connection::builders::TestConnectionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::test_connection::TestConnectionOutput,
        crate::operation::test_connection::TestConnectionError,
    > for TestConnectionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::test_connection::TestConnectionOutput,
            crate::operation::test_connection::TestConnectionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl TestConnectionFluentBuilder {
    /// Creates a new `TestConnectionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the TestConnection as a reference.
    pub fn as_input(&self) -> &crate::operation::test_connection::builders::TestConnectionInputBuilder {
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
        crate::operation::test_connection::TestConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::test_connection::TestConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::test_connection::TestConnection::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::test_connection::TestConnection::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::test_connection::TestConnectionOutput,
        crate::operation::test_connection::TestConnectionError,
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
    /// <p>Optional. The name of the connection to test. If only name is provided, the operation will get the connection and use that for testing.</p>
    pub fn connection_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_name(input.into());
        self
    }
    /// <p>Optional. The name of the connection to test. If only name is provided, the operation will get the connection and use that for testing.</p>
    pub fn set_connection_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connection_name(input);
        self
    }
    /// <p>Optional. The name of the connection to test. If only name is provided, the operation will get the connection and use that for testing.</p>
    pub fn get_connection_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connection_name()
    }
    /// <p>The catalog ID where the connection resides.</p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog_id(input.into());
        self
    }
    /// <p>The catalog ID where the connection resides.</p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog_id(input);
        self
    }
    /// <p>The catalog ID where the connection resides.</p>
    pub fn get_catalog_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_catalog_id()
    }
    /// <p>A structure that is used to specify testing a connection to a service.</p>
    pub fn test_connection_input(mut self, input: crate::types::TestConnectionInput) -> Self {
        self.inner = self.inner.test_connection_input(input);
        self
    }
    /// <p>A structure that is used to specify testing a connection to a service.</p>
    pub fn set_test_connection_input(mut self, input: ::std::option::Option<crate::types::TestConnectionInput>) -> Self {
        self.inner = self.inner.set_test_connection_input(input);
        self
    }
    /// <p>A structure that is used to specify testing a connection to a service.</p>
    pub fn get_test_connection_input(&self) -> &::std::option::Option<crate::types::TestConnectionInput> {
        self.inner.get_test_connection_input()
    }
}
