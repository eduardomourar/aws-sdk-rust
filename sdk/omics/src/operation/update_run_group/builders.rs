// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_run_group::_update_run_group_output::UpdateRunGroupOutputBuilder;

pub use crate::operation::update_run_group::_update_run_group_input::UpdateRunGroupInputBuilder;

impl crate::operation::update_run_group::builders::UpdateRunGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_run_group::UpdateRunGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_run_group::UpdateRunGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_run_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateRunGroup`.
///
/// <p>Updates the settings of a run group and returns a response with no body if the operation is successful.</p>
/// <p>You can update the following settings with <code>UpdateRunGroup</code>:</p>
/// <ul>
/// <li>
/// <p>Maximum number of CPUs</p></li>
/// <li>
/// <p>Run time (measured in minutes)</p></li>
/// <li>
/// <p>Number of GPUs</p></li>
/// <li>
/// <p>Number of concurrent runs</p></li>
/// <li>
/// <p>Group name</p></li>
/// </ul>
/// <p>To confirm that the settings have been successfully updated, use the <code>ListRunGroups</code> or <code>GetRunGroup</code> API operations to verify that the desired changes have been made.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRunGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_run_group::builders::UpdateRunGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_run_group::UpdateRunGroupOutput,
        crate::operation::update_run_group::UpdateRunGroupError,
    > for UpdateRunGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_run_group::UpdateRunGroupOutput,
            crate::operation::update_run_group::UpdateRunGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateRunGroupFluentBuilder {
    /// Creates a new `UpdateRunGroupFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateRunGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::update_run_group::builders::UpdateRunGroupInputBuilder {
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
        crate::operation::update_run_group::UpdateRunGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_run_group::UpdateRunGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_run_group::UpdateRunGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_run_group::UpdateRunGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_run_group::UpdateRunGroupOutput,
        crate::operation::update_run_group::UpdateRunGroupError,
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
    /// <p>The group's ID.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The group's ID.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The group's ID.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>A name for the group.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A name for the group.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A name for the group.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The maximum number of CPUs to use.</p>
    pub fn max_cpus(mut self, input: i32) -> Self {
        self.inner = self.inner.max_cpus(input);
        self
    }
    /// <p>The maximum number of CPUs to use.</p>
    pub fn set_max_cpus(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_cpus(input);
        self
    }
    /// <p>The maximum number of CPUs to use.</p>
    pub fn get_max_cpus(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_cpus()
    }
    /// <p>The maximum number of concurrent runs for the group.</p>
    pub fn max_runs(mut self, input: i32) -> Self {
        self.inner = self.inner.max_runs(input);
        self
    }
    /// <p>The maximum number of concurrent runs for the group.</p>
    pub fn set_max_runs(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_runs(input);
        self
    }
    /// <p>The maximum number of concurrent runs for the group.</p>
    pub fn get_max_runs(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_runs()
    }
    /// <p>A maximum run time for the group in minutes.</p>
    pub fn max_duration(mut self, input: i32) -> Self {
        self.inner = self.inner.max_duration(input);
        self
    }
    /// <p>A maximum run time for the group in minutes.</p>
    pub fn set_max_duration(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_duration(input);
        self
    }
    /// <p>A maximum run time for the group in minutes.</p>
    pub fn get_max_duration(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_duration()
    }
    /// <p>The maximum GPUs that can be used by a run group.</p>
    pub fn max_gpus(mut self, input: i32) -> Self {
        self.inner = self.inner.max_gpus(input);
        self
    }
    /// <p>The maximum GPUs that can be used by a run group.</p>
    pub fn set_max_gpus(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_gpus(input);
        self
    }
    /// <p>The maximum GPUs that can be used by a run group.</p>
    pub fn get_max_gpus(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_gpus()
    }
}
