// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_resource::_register_resource_output::RegisterResourceOutputBuilder;

pub use crate::operation::register_resource::_register_resource_input::RegisterResourceInputBuilder;

impl crate::operation::register_resource::builders::RegisterResourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::register_resource::RegisterResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_resource::RegisterResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.register_resource();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RegisterResource`.
///
/// <p>Registers the resource as managed by the Data Catalog.</p>
/// <p>To add or update data, Lake Formation needs read/write access to the chosen data location. Choose a role that you know has permission to do this, or choose the AWSServiceRoleForLakeFormationDataAccess service-linked role. When you register the first Amazon S3 path, the service-linked role and a new inline policy are created on your behalf. Lake Formation adds the first path to the inline policy and attaches it to the service-linked role. When you register subsequent paths, Lake Formation adds the path to the existing policy.</p>
/// <p>The following request registers a new location and gives Lake Formation permission to use the service-linked role to access that location.</p>
/// <p><code>ResourceArn = arn:aws:s3:::my-bucket/ UseServiceLinkedRole = true</code></p>
/// <p>If <code>UseServiceLinkedRole</code> is not set to true, you must provide or set the <code>RoleArn</code>:</p>
/// <p><code>arn:aws:iam::12345:role/my-data-access-role</code></p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterResourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_resource::builders::RegisterResourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::register_resource::RegisterResourceOutput,
        crate::operation::register_resource::RegisterResourceError,
    > for RegisterResourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::register_resource::RegisterResourceOutput,
            crate::operation::register_resource::RegisterResourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RegisterResourceFluentBuilder {
    /// Creates a new `RegisterResourceFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RegisterResource as a reference.
    pub fn as_input(&self) -> &crate::operation::register_resource::builders::RegisterResourceInputBuilder {
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
        crate::operation::register_resource::RegisterResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_resource::RegisterResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::register_resource::RegisterResource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::register_resource::RegisterResource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::register_resource::RegisterResourceOutput,
        crate::operation::register_resource::RegisterResourceError,
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
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to register.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to register.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to register.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_arn()
    }
    /// <p>Designates an Identity and Access Management (IAM) service-linked role by registering this role with the Data Catalog. A service-linked role is a unique type of IAM role that is linked directly to Lake Formation.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/lake-formation/latest/dg/service-linked-roles.html">Using Service-Linked Roles for Lake Formation</a>.</p>
    pub fn use_service_linked_role(mut self, input: bool) -> Self {
        self.inner = self.inner.use_service_linked_role(input);
        self
    }
    /// <p>Designates an Identity and Access Management (IAM) service-linked role by registering this role with the Data Catalog. A service-linked role is a unique type of IAM role that is linked directly to Lake Formation.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/lake-formation/latest/dg/service-linked-roles.html">Using Service-Linked Roles for Lake Formation</a>.</p>
    pub fn set_use_service_linked_role(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_use_service_linked_role(input);
        self
    }
    /// <p>Designates an Identity and Access Management (IAM) service-linked role by registering this role with the Data Catalog. A service-linked role is a unique type of IAM role that is linked directly to Lake Formation.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/lake-formation/latest/dg/service-linked-roles.html">Using Service-Linked Roles for Lake Formation</a>.</p>
    pub fn get_use_service_linked_role(&self) -> &::std::option::Option<bool> {
        self.inner.get_use_service_linked_role()
    }
    /// <p>The identifier for the role that registers the resource.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The identifier for the role that registers the resource.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The identifier for the role that registers the resource.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// <p>Whether or not the resource is a federated resource.</p>
    pub fn with_federation(mut self, input: bool) -> Self {
        self.inner = self.inner.with_federation(input);
        self
    }
    /// <p>Whether or not the resource is a federated resource.</p>
    pub fn set_with_federation(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_with_federation(input);
        self
    }
    /// <p>Whether or not the resource is a federated resource.</p>
    pub fn get_with_federation(&self) -> &::std::option::Option<bool> {
        self.inner.get_with_federation()
    }
    /// <p>Specifies whether the data access of tables pointing to the location can be managed by both Lake Formation permissions as well as Amazon S3 bucket policies.</p>
    pub fn hybrid_access_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.hybrid_access_enabled(input);
        self
    }
    /// <p>Specifies whether the data access of tables pointing to the location can be managed by both Lake Formation permissions as well as Amazon S3 bucket policies.</p>
    pub fn set_hybrid_access_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_hybrid_access_enabled(input);
        self
    }
    /// <p>Specifies whether the data access of tables pointing to the location can be managed by both Lake Formation permissions as well as Amazon S3 bucket policies.</p>
    pub fn get_hybrid_access_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_hybrid_access_enabled()
    }
    /// <p>Grants the calling principal the permissions to perform all supported Lake Formation operations on the registered data location.</p>
    pub fn with_privileged_access(mut self, input: bool) -> Self {
        self.inner = self.inner.with_privileged_access(input);
        self
    }
    /// <p>Grants the calling principal the permissions to perform all supported Lake Formation operations on the registered data location.</p>
    pub fn set_with_privileged_access(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_with_privileged_access(input);
        self
    }
    /// <p>Grants the calling principal the permissions to perform all supported Lake Formation operations on the registered data location.</p>
    pub fn get_with_privileged_access(&self) -> &::std::option::Option<bool> {
        self.inner.get_with_privileged_access()
    }
}
