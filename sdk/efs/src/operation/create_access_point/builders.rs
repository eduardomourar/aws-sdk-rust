// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_access_point::_create_access_point_output::CreateAccessPointOutputBuilder;

pub use crate::operation::create_access_point::_create_access_point_input::CreateAccessPointInputBuilder;

impl crate::operation::create_access_point::builders::CreateAccessPointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_access_point::CreateAccessPointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_access_point::CreateAccessPointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_access_point();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAccessPoint`.
///
/// <p>Creates an EFS access point. An access point is an application-specific view into an EFS file system that applies an operating system user and group, and a file system path, to any file system request made through the access point. The operating system user and group override any identity information provided by the NFS client. The file system path is exposed as the access point's root directory. Applications using the access point can only access data in the application's own directory and any subdirectories. A file system can have a maximum of 10,000 access points unless you request an increase. To learn more, see <a href="https://docs.aws.amazon.com/efs/latest/ug/efs-access-points.html">Mounting a file system using EFS access points</a>.</p><note>
/// <p>If multiple requests to create access points on the same file system are sent in quick succession, and the file system is near the limit of access points, you may experience a throttling response for these requests. This is to ensure that the file system does not exceed the stated access point limit.</p>
/// </note>
/// <p>This operation requires permissions for the <code>elasticfilesystem:CreateAccessPoint</code> action.</p>
/// <p>Access points can be tagged on creation. If tags are specified in the creation action, IAM performs additional authorization on the <code>elasticfilesystem:TagResource</code> action to verify if users have permissions to create tags. Therefore, you must grant explicit permissions to use the <code>elasticfilesystem:TagResource</code> action. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/using-tags-efs.html#supported-iam-actions-tagging.html">Granting permissions to tag resources during creation</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAccessPointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_access_point::builders::CreateAccessPointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_access_point::CreateAccessPointOutput,
        crate::operation::create_access_point::CreateAccessPointError,
    > for CreateAccessPointFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_access_point::CreateAccessPointOutput,
            crate::operation::create_access_point::CreateAccessPointError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateAccessPointFluentBuilder {
    /// Creates a new `CreateAccessPointFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateAccessPoint as a reference.
    pub fn as_input(&self) -> &crate::operation::create_access_point::builders::CreateAccessPointInputBuilder {
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
        crate::operation::create_access_point::CreateAccessPointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_access_point::CreateAccessPointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_access_point::CreateAccessPoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_access_point::CreateAccessPoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_access_point::CreateAccessPointOutput,
        crate::operation::create_access_point::CreateAccessPointError,
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
    /// <p>A string of up to 64 ASCII characters that Amazon EFS uses to ensure idempotent creation.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A string of up to 64 ASCII characters that Amazon EFS uses to ensure idempotent creation.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A string of up to 64 ASCII characters that Amazon EFS uses to ensure idempotent creation.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Creates tags associated with the access point. Each tag is a key-value pair, each key must be unique. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Creates tags associated with the access point. Each tag is a key-value pair, each key must be unique. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Creates tags associated with the access point. Each tag is a key-value pair, each key must be unique. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>The ID of the EFS file system that the access point provides access to.</p>
    pub fn file_system_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.file_system_id(input.into());
        self
    }
    /// <p>The ID of the EFS file system that the access point provides access to.</p>
    pub fn set_file_system_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_file_system_id(input);
        self
    }
    /// <p>The ID of the EFS file system that the access point provides access to.</p>
    pub fn get_file_system_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_file_system_id()
    }
    /// <p>The operating system user and group applied to all file system requests made using the access point.</p>
    pub fn posix_user(mut self, input: crate::types::PosixUser) -> Self {
        self.inner = self.inner.posix_user(input);
        self
    }
    /// <p>The operating system user and group applied to all file system requests made using the access point.</p>
    pub fn set_posix_user(mut self, input: ::std::option::Option<crate::types::PosixUser>) -> Self {
        self.inner = self.inner.set_posix_user(input);
        self
    }
    /// <p>The operating system user and group applied to all file system requests made using the access point.</p>
    pub fn get_posix_user(&self) -> &::std::option::Option<crate::types::PosixUser> {
        self.inner.get_posix_user()
    }
    /// <p>Specifies the directory on the EFS file system that the access point exposes as the root directory of your file system to NFS clients using the access point. The clients using the access point can only access the root directory and below. If the <code>RootDirectory</code> &gt; <code>Path</code> specified does not exist, Amazon EFS creates it and applies the <code>CreationInfo</code> settings when a client connects to an access point. When specifying a <code>RootDirectory</code>, you must provide the <code>Path</code>, and the <code>CreationInfo</code>.</p>
    /// <p>Amazon EFS creates a root directory only if you have provided the CreationInfo: OwnUid, OwnGID, and permissions for the directory. If you do not provide this information, Amazon EFS does not create the root directory. If the root directory does not exist, attempts to mount using the access point will fail.</p>
    pub fn root_directory(mut self, input: crate::types::RootDirectory) -> Self {
        self.inner = self.inner.root_directory(input);
        self
    }
    /// <p>Specifies the directory on the EFS file system that the access point exposes as the root directory of your file system to NFS clients using the access point. The clients using the access point can only access the root directory and below. If the <code>RootDirectory</code> &gt; <code>Path</code> specified does not exist, Amazon EFS creates it and applies the <code>CreationInfo</code> settings when a client connects to an access point. When specifying a <code>RootDirectory</code>, you must provide the <code>Path</code>, and the <code>CreationInfo</code>.</p>
    /// <p>Amazon EFS creates a root directory only if you have provided the CreationInfo: OwnUid, OwnGID, and permissions for the directory. If you do not provide this information, Amazon EFS does not create the root directory. If the root directory does not exist, attempts to mount using the access point will fail.</p>
    pub fn set_root_directory(mut self, input: ::std::option::Option<crate::types::RootDirectory>) -> Self {
        self.inner = self.inner.set_root_directory(input);
        self
    }
    /// <p>Specifies the directory on the EFS file system that the access point exposes as the root directory of your file system to NFS clients using the access point. The clients using the access point can only access the root directory and below. If the <code>RootDirectory</code> &gt; <code>Path</code> specified does not exist, Amazon EFS creates it and applies the <code>CreationInfo</code> settings when a client connects to an access point. When specifying a <code>RootDirectory</code>, you must provide the <code>Path</code>, and the <code>CreationInfo</code>.</p>
    /// <p>Amazon EFS creates a root directory only if you have provided the CreationInfo: OwnUid, OwnGID, and permissions for the directory. If you do not provide this information, Amazon EFS does not create the root directory. If the root directory does not exist, attempts to mount using the access point will fail.</p>
    pub fn get_root_directory(&self) -> &::std::option::Option<crate::types::RootDirectory> {
        self.inner.get_root_directory()
    }
}
