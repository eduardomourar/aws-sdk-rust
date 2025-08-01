// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_tags_for_resource::_list_tags_for_resource_output::ListTagsForResourceOutputBuilder;

pub use crate::operation::list_tags_for_resource::_list_tags_for_resource_input::ListTagsForResourceInputBuilder;

impl crate::operation::list_tags_for_resource::builders::ListTagsForResourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_tags_for_resource::ListTagsForResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_tags_for_resource();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListTagsForResource`.
///
/// <p>Displays the tags associated with a CloudWatch investigations resource. Currently, investigation groups support tagging.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTagsForResourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_tags_for_resource::builders::ListTagsForResourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
        crate::operation::list_tags_for_resource::ListTagsForResourceError,
    > for ListTagsForResourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
            crate::operation::list_tags_for_resource::ListTagsForResourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListTagsForResourceFluentBuilder {
    /// Creates a new `ListTagsForResourceFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListTagsForResource as a reference.
    pub fn as_input(&self) -> &crate::operation::list_tags_for_resource::builders::ListTagsForResourceInputBuilder {
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
        crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_tags_for_resource::ListTagsForResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_tags_for_resource::ListTagsForResource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_tags_for_resource::ListTagsForResource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
        crate::operation::list_tags_for_resource::ListTagsForResourceError,
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
    /// <p>The ARN of the CloudWatch investigations resource that you want to view tags for. You can use the <code>ListInvestigationGroups</code> operation to find the ARNs of investigation groups.</p>
    /// <p>The ARN format for an investigation group is <code>arn:aws:aiops:<i>Region</i>:<i>account-id</i>:investigation-group:<i>investigation-group-id</i> </code>.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The ARN of the CloudWatch investigations resource that you want to view tags for. You can use the <code>ListInvestigationGroups</code> operation to find the ARNs of investigation groups.</p>
    /// <p>The ARN format for an investigation group is <code>arn:aws:aiops:<i>Region</i>:<i>account-id</i>:investigation-group:<i>investigation-group-id</i> </code>.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>The ARN of the CloudWatch investigations resource that you want to view tags for. You can use the <code>ListInvestigationGroups</code> operation to find the ARNs of investigation groups.</p>
    /// <p>The ARN format for an investigation group is <code>arn:aws:aiops:<i>Region</i>:<i>account-id</i>:investigation-group:<i>investigation-group-id</i> </code>.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_arn()
    }
}
