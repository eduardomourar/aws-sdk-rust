// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_api::_create_api_output::CreateApiOutputBuilder;

pub use crate::operation::create_api::_create_api_input::CreateApiInputBuilder;

impl crate::operation::create_api::builders::CreateApiInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_api::CreateApiOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_api::CreateApiError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_api();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateApi`.
///
/// <p>Creates an Api resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateApiFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_api::builders::CreateApiInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::create_api::CreateApiOutput, crate::operation::create_api::CreateApiError>
    for CreateApiFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::create_api::CreateApiOutput, crate::operation::create_api::CreateApiError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateApiFluentBuilder {
    /// Creates a new `CreateApiFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateApi as a reference.
    pub fn as_input(&self) -> &crate::operation::create_api::builders::CreateApiInputBuilder {
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
        crate::operation::create_api::CreateApiOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_api::CreateApiError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_api::CreateApi::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_api::CreateApi::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_api::CreateApiOutput,
        crate::operation::create_api::CreateApiError,
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
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    pub fn api_key_selection_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.api_key_selection_expression(input.into());
        self
    }
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    pub fn set_api_key_selection_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_api_key_selection_expression(input);
        self
    }
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    pub fn get_api_key_selection_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_api_key_selection_expression()
    }
    /// <p>A CORS configuration. Supported only for HTTP APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html">Configuring CORS</a> for more information.</p>
    pub fn cors_configuration(mut self, input: crate::types::Cors) -> Self {
        self.inner = self.inner.cors_configuration(input);
        self
    }
    /// <p>A CORS configuration. Supported only for HTTP APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html">Configuring CORS</a> for more information.</p>
    pub fn set_cors_configuration(mut self, input: ::std::option::Option<crate::types::Cors>) -> Self {
        self.inner = self.inner.set_cors_configuration(input);
        self
    }
    /// <p>A CORS configuration. Supported only for HTTP APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html">Configuring CORS</a> for more information.</p>
    pub fn get_cors_configuration(&self) -> &::std::option::Option<crate::types::Cors> {
        self.inner.get_cors_configuration()
    }
    /// <p>This property is part of quick create. It specifies the credentials required for the integration, if any. For a Lambda integration, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null. Currently, this property is not used for HTTP integrations. Supported only for HTTP APIs.</p>
    pub fn credentials_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.credentials_arn(input.into());
        self
    }
    /// <p>This property is part of quick create. It specifies the credentials required for the integration, if any. For a Lambda integration, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null. Currently, this property is not used for HTTP integrations. Supported only for HTTP APIs.</p>
    pub fn set_credentials_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_credentials_arn(input);
        self
    }
    /// <p>This property is part of quick create. It specifies the credentials required for the integration, if any. For a Lambda integration, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null. Currently, this property is not used for HTTP integrations. Supported only for HTTP APIs.</p>
    pub fn get_credentials_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_credentials_arn()
    }
    /// <p>The description of the API.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the API.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the API.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    pub fn disable_schema_validation(mut self, input: bool) -> Self {
        self.inner = self.inner.disable_schema_validation(input);
        self
    }
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    pub fn set_disable_schema_validation(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_disable_schema_validation(input);
        self
    }
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    pub fn get_disable_schema_validation(&self) -> &::std::option::Option<bool> {
        self.inner.get_disable_schema_validation()
    }
    /// <p>Specifies whether clients can invoke your API by using the default execute-api endpoint. By default, clients can invoke your API with the default https://{api_id}.execute-api.{region}.amazonaws.com endpoint. To require that clients use a custom domain name to invoke your API, disable the default endpoint.</p>
    pub fn disable_execute_api_endpoint(mut self, input: bool) -> Self {
        self.inner = self.inner.disable_execute_api_endpoint(input);
        self
    }
    /// <p>Specifies whether clients can invoke your API by using the default execute-api endpoint. By default, clients can invoke your API with the default https://{api_id}.execute-api.{region}.amazonaws.com endpoint. To require that clients use a custom domain name to invoke your API, disable the default endpoint.</p>
    pub fn set_disable_execute_api_endpoint(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_disable_execute_api_endpoint(input);
        self
    }
    /// <p>Specifies whether clients can invoke your API by using the default execute-api endpoint. By default, clients can invoke your API with the default https://{api_id}.execute-api.{region}.amazonaws.com endpoint. To require that clients use a custom domain name to invoke your API, disable the default endpoint.</p>
    pub fn get_disable_execute_api_endpoint(&self) -> &::std::option::Option<bool> {
        self.inner.get_disable_execute_api_endpoint()
    }
    /// <p>The IP address types that can invoke the API.</p>
    pub fn ip_address_type(mut self, input: crate::types::IpAddressType) -> Self {
        self.inner = self.inner.ip_address_type(input);
        self
    }
    /// <p>The IP address types that can invoke the API.</p>
    pub fn set_ip_address_type(mut self, input: ::std::option::Option<crate::types::IpAddressType>) -> Self {
        self.inner = self.inner.set_ip_address_type(input);
        self
    }
    /// <p>The IP address types that can invoke the API.</p>
    pub fn get_ip_address_type(&self) -> &::std::option::Option<crate::types::IpAddressType> {
        self.inner.get_ip_address_type()
    }
    /// <p>The name of the API.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the API.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the API.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The API protocol.</p>
    pub fn protocol_type(mut self, input: crate::types::ProtocolType) -> Self {
        self.inner = self.inner.protocol_type(input);
        self
    }
    /// <p>The API protocol.</p>
    pub fn set_protocol_type(mut self, input: ::std::option::Option<crate::types::ProtocolType>) -> Self {
        self.inner = self.inner.set_protocol_type(input);
        self
    }
    /// <p>The API protocol.</p>
    pub fn get_protocol_type(&self) -> &::std::option::Option<crate::types::ProtocolType> {
        self.inner.get_protocol_type()
    }
    /// <p>This property is part of quick create. If you don't specify a routeKey, a default route of $default is created. The $default route acts as a catch-all for any request made to your API, for a particular stage. The $default route key can't be modified. You can add routes after creating the API, and you can update the route keys of additional routes. Supported only for HTTP APIs.</p>
    pub fn route_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_key(input.into());
        self
    }
    /// <p>This property is part of quick create. If you don't specify a routeKey, a default route of $default is created. The $default route acts as a catch-all for any request made to your API, for a particular stage. The $default route key can't be modified. You can add routes after creating the API, and you can update the route keys of additional routes. Supported only for HTTP APIs.</p>
    pub fn set_route_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_route_key(input);
        self
    }
    /// <p>This property is part of quick create. If you don't specify a routeKey, a default route of $default is created. The $default route acts as a catch-all for any request made to your API, for a particular stage. The $default route key can't be modified. You can add routes after creating the API, and you can update the route keys of additional routes. Supported only for HTTP APIs.</p>
    pub fn get_route_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_route_key()
    }
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    pub fn route_selection_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_selection_expression(input.into());
        self
    }
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    pub fn set_route_selection_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_route_selection_expression(input);
        self
    }
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    pub fn get_route_selection_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_route_selection_expression()
    }
    ///
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// <p>This property is part of quick create. Quick create produces an API with an integration, a default catch-all route, and a default stage which is configured to automatically deploy changes. For HTTP integrations, specify a fully qualified URL. For Lambda integrations, specify a function ARN. The type of the integration will be HTTP_PROXY or AWS_PROXY, respectively. Supported only for HTTP APIs.</p>
    pub fn target(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target(input.into());
        self
    }
    /// <p>This property is part of quick create. Quick create produces an API with an integration, a default catch-all route, and a default stage which is configured to automatically deploy changes. For HTTP integrations, specify a fully qualified URL. For Lambda integrations, specify a function ARN. The type of the integration will be HTTP_PROXY or AWS_PROXY, respectively. Supported only for HTTP APIs.</p>
    pub fn set_target(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target(input);
        self
    }
    /// <p>This property is part of quick create. Quick create produces an API with an integration, a default catch-all route, and a default stage which is configured to automatically deploy changes. For HTTP integrations, specify a fully qualified URL. For Lambda integrations, specify a function ARN. The type of the integration will be HTTP_PROXY or AWS_PROXY, respectively. Supported only for HTTP APIs.</p>
    pub fn get_target(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target()
    }
    /// <p>A version identifier for the API.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version(input.into());
        self
    }
    /// <p>A version identifier for the API.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version(input);
        self
    }
    /// <p>A version identifier for the API.</p>
    pub fn get_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version()
    }
}
