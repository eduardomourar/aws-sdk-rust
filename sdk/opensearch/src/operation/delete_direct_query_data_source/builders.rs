// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_direct_query_data_source::_delete_direct_query_data_source_output::DeleteDirectQueryDataSourceOutputBuilder;

pub use crate::operation::delete_direct_query_data_source::_delete_direct_query_data_source_input::DeleteDirectQueryDataSourceInputBuilder;

impl crate::operation::delete_direct_query_data_source::builders::DeleteDirectQueryDataSourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_direct_query_data_source();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDirectQueryDataSource`.
///
/// <p>Deletes a previously configured direct query data source from Amazon OpenSearch Service.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDirectQueryDataSourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_direct_query_data_source::builders::DeleteDirectQueryDataSourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceOutput,
        crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError,
    > for DeleteDirectQueryDataSourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceOutput,
            crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteDirectQueryDataSourceFluentBuilder {
    /// Creates a new `DeleteDirectQueryDataSourceFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDirectQueryDataSource as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_direct_query_data_source::builders::DeleteDirectQueryDataSourceInputBuilder {
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
        crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceOutput,
        crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError,
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
    /// <p>A unique, user-defined label to identify the data source within your OpenSearch Service environment.</p>
    pub fn data_source_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data_source_name(input.into());
        self
    }
    /// <p>A unique, user-defined label to identify the data source within your OpenSearch Service environment.</p>
    pub fn set_data_source_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_data_source_name(input);
        self
    }
    /// <p>A unique, user-defined label to identify the data source within your OpenSearch Service environment.</p>
    pub fn get_data_source_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_data_source_name()
    }
}
