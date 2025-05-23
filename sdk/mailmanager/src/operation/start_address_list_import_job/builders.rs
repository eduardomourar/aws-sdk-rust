// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_address_list_import_job::_start_address_list_import_job_output::StartAddressListImportJobOutputBuilder;

pub use crate::operation::start_address_list_import_job::_start_address_list_import_job_input::StartAddressListImportJobInputBuilder;

impl crate::operation::start_address_list_import_job::builders::StartAddressListImportJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_address_list_import_job::StartAddressListImportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_address_list_import_job::StartAddressListImportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_address_list_import_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartAddressListImportJob`.
///
/// <p>Starts an import job for an address list.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartAddressListImportJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_address_list_import_job::builders::StartAddressListImportJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_address_list_import_job::StartAddressListImportJobOutput,
        crate::operation::start_address_list_import_job::StartAddressListImportJobError,
    > for StartAddressListImportJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_address_list_import_job::StartAddressListImportJobOutput,
            crate::operation::start_address_list_import_job::StartAddressListImportJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartAddressListImportJobFluentBuilder {
    /// Creates a new `StartAddressListImportJobFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartAddressListImportJob as a reference.
    pub fn as_input(&self) -> &crate::operation::start_address_list_import_job::builders::StartAddressListImportJobInputBuilder {
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
        crate::operation::start_address_list_import_job::StartAddressListImportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_address_list_import_job::StartAddressListImportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_address_list_import_job::StartAddressListImportJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_address_list_import_job::StartAddressListImportJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_address_list_import_job::StartAddressListImportJobOutput,
        crate::operation::start_address_list_import_job::StartAddressListImportJobError,
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
    /// <p>The identifier of the import job that needs to be started.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The identifier of the import job that needs to be started.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
    /// <p>The identifier of the import job that needs to be started.</p>
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_job_id()
    }
}
