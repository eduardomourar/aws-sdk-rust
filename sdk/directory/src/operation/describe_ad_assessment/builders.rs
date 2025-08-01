// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_ad_assessment::_describe_ad_assessment_output::DescribeAdAssessmentOutputBuilder;

pub use crate::operation::describe_ad_assessment::_describe_ad_assessment_input::DescribeAdAssessmentInputBuilder;

impl crate::operation::describe_ad_assessment::builders::DescribeAdAssessmentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_ad_assessment::DescribeAdAssessmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_ad_assessment::DescribeADAssessmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_ad_assessment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeADAssessment`.
///
/// <p>Retrieves detailed information about a directory assessment, including its current status, validation results, and configuration details. Use this operation to monitor assessment progress and review results.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeADAssessmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_ad_assessment::builders::DescribeAdAssessmentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_ad_assessment::DescribeAdAssessmentOutput,
        crate::operation::describe_ad_assessment::DescribeADAssessmentError,
    > for DescribeADAssessmentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_ad_assessment::DescribeAdAssessmentOutput,
            crate::operation::describe_ad_assessment::DescribeADAssessmentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeADAssessmentFluentBuilder {
    /// Creates a new `DescribeADAssessmentFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeADAssessment as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_ad_assessment::builders::DescribeAdAssessmentInputBuilder {
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
        crate::operation::describe_ad_assessment::DescribeAdAssessmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_ad_assessment::DescribeADAssessmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_ad_assessment::DescribeADAssessment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_ad_assessment::DescribeADAssessment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_ad_assessment::DescribeAdAssessmentOutput,
        crate::operation::describe_ad_assessment::DescribeADAssessmentError,
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
    /// <p>The identifier of the directory assessment to describe.</p>
    pub fn assessment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.assessment_id(input.into());
        self
    }
    /// <p>The identifier of the directory assessment to describe.</p>
    pub fn set_assessment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_assessment_id(input);
        self
    }
    /// <p>The identifier of the directory assessment to describe.</p>
    pub fn get_assessment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_assessment_id()
    }
}
