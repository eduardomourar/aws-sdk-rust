// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_disk::_delete_disk_output::DeleteDiskOutputBuilder;

pub use crate::operation::delete_disk::_delete_disk_input::DeleteDiskInputBuilder;

impl crate::operation::delete_disk::builders::DeleteDiskInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_disk::DeleteDiskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_disk::DeleteDiskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_disk();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDisk`.
///
/// <p>Deletes the specified block storage disk. The disk must be in the <code>available</code> state (not attached to a Lightsail instance).</p><note>
/// <p>The disk may remain in the <code>deleting</code> state for several minutes.</p>
/// </note>
/// <p>The <code>delete disk</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>disk name</code>. For more information, see the <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-controlling-access-using-tags">Amazon Lightsail Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDiskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_disk::builders::DeleteDiskInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_disk::DeleteDiskOutput,
        crate::operation::delete_disk::DeleteDiskError,
    > for DeleteDiskFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_disk::DeleteDiskOutput,
            crate::operation::delete_disk::DeleteDiskError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteDiskFluentBuilder {
    /// Creates a new `DeleteDiskFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDisk as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_disk::builders::DeleteDiskInputBuilder {
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
        crate::operation::delete_disk::DeleteDiskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_disk::DeleteDiskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_disk::DeleteDisk::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_disk::DeleteDisk::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_disk::DeleteDiskOutput,
        crate::operation::delete_disk::DeleteDiskError,
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
    /// <p>The unique name of the disk you want to delete (<code>my-disk</code>).</p>
    pub fn disk_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.disk_name(input.into());
        self
    }
    /// <p>The unique name of the disk you want to delete (<code>my-disk</code>).</p>
    pub fn set_disk_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_disk_name(input);
        self
    }
    /// <p>The unique name of the disk you want to delete (<code>my-disk</code>).</p>
    pub fn get_disk_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_disk_name()
    }
    /// <p>A Boolean value to indicate whether to delete all add-ons for the disk.</p>
    pub fn force_delete_add_ons(mut self, input: bool) -> Self {
        self.inner = self.inner.force_delete_add_ons(input);
        self
    }
    /// <p>A Boolean value to indicate whether to delete all add-ons for the disk.</p>
    pub fn set_force_delete_add_ons(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force_delete_add_ons(input);
        self
    }
    /// <p>A Boolean value to indicate whether to delete all add-ons for the disk.</p>
    pub fn get_force_delete_add_ons(&self) -> &::std::option::Option<bool> {
        self.inner.get_force_delete_add_ons()
    }
}
