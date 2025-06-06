// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_remote_move::_start_remote_move_output::StartRemoteMoveOutputBuilder;

pub use crate::operation::start_remote_move::_start_remote_move_input::StartRemoteMoveInputBuilder;

impl crate::operation::start_remote_move::builders::StartRemoteMoveInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_remote_move::StartRemoteMoveOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_remote_move::StartRemoteMoveError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_remote_move();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartRemoteMove`.
///
/// <p>Moves or renames a file or directory on the remote SFTP server.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartRemoteMoveFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_remote_move::builders::StartRemoteMoveInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_remote_move::StartRemoteMoveOutput,
        crate::operation::start_remote_move::StartRemoteMoveError,
    > for StartRemoteMoveFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_remote_move::StartRemoteMoveOutput,
            crate::operation::start_remote_move::StartRemoteMoveError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartRemoteMoveFluentBuilder {
    /// Creates a new `StartRemoteMoveFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartRemoteMove as a reference.
    pub fn as_input(&self) -> &crate::operation::start_remote_move::builders::StartRemoteMoveInputBuilder {
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
        crate::operation::start_remote_move::StartRemoteMoveOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_remote_move::StartRemoteMoveError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_remote_move::StartRemoteMove::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_remote_move::StartRemoteMove::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_remote_move::StartRemoteMoveOutput,
        crate::operation::start_remote_move::StartRemoteMoveError,
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
    /// <p>The unique identifier for the connector.</p>
    pub fn connector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connector_id(input.into());
        self
    }
    /// <p>The unique identifier for the connector.</p>
    pub fn set_connector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connector_id(input);
        self
    }
    /// <p>The unique identifier for the connector.</p>
    pub fn get_connector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connector_id()
    }
    /// <p>The absolute path of the file or directory to move or rename. You can only specify one path per call to this operation.</p>
    pub fn source_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_path(input.into());
        self
    }
    /// <p>The absolute path of the file or directory to move or rename. You can only specify one path per call to this operation.</p>
    pub fn set_source_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_path(input);
        self
    }
    /// <p>The absolute path of the file or directory to move or rename. You can only specify one path per call to this operation.</p>
    pub fn get_source_path(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_path()
    }
    /// <p>The absolute path for the target of the move/rename operation.</p>
    pub fn target_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_path(input.into());
        self
    }
    /// <p>The absolute path for the target of the move/rename operation.</p>
    pub fn set_target_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_path(input);
        self
    }
    /// <p>The absolute path for the target of the move/rename operation.</p>
    pub fn get_target_path(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_path()
    }
}
