// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `DeleteClusterPolicy`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteClusterPolicy;
impl DeleteClusterPolicy {
    /// Creates a new `DeleteClusterPolicy`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::delete_cluster_policy::DeleteClusterPolicyInput,
    ) -> ::std::result::Result<
        crate::operation::delete_cluster_policy::DeleteClusterPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_cluster_policy::DeleteClusterPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::delete_cluster_policy::DeleteClusterPolicyError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::delete_cluster_policy::DeleteClusterPolicyOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::delete_cluster_policy::DeleteClusterPolicyInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        use ::tracing::Instrument;
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("Kafka", "DeleteClusterPolicy", input, runtime_plugins, stop_point)
            // Create a parent span for the entire operation. Includes a random, internal-only,
            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
            .instrument(::tracing::debug_span!(
                "Kafka.DeleteClusterPolicy",
                "rpc.service" = "Kafka",
                "rpc.method" = "DeleteClusterPolicy",
                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
                "rpc.system" = "aws-api",
            ))
            .await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());

        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteClusterPolicy {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DeleteClusterPolicy");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            DeleteClusterPolicyRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            DeleteClusterPolicyResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            crate::config::auth::Params::builder()
                .operation_name("DeleteClusterPolicy")
                .build()
                .expect("required fields set"),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "DeleteClusterPolicy",
            "Kafka",
        ));
        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
            signing_options,
            ..::std::default::Default::default()
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(
        &self,
        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        #[allow(unused_mut)]
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteClusterPolicy")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(DeleteClusterPolicyEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::delete_cluster_policy::DeleteClusterPolicyError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::delete_cluster_policy::DeleteClusterPolicyError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::delete_cluster_policy::DeleteClusterPolicyError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct DeleteClusterPolicyResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for DeleteClusterPolicyResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        #[allow(unused_mut)]
        let mut force_error = false;
        ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 || force_error {
            crate::protocol_serde::shape_delete_cluster_policy::de_delete_cluster_policy_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_delete_cluster_policy::de_delete_cluster_policy_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct DeleteClusterPolicyRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for DeleteClusterPolicyRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::delete_cluster_policy::DeleteClusterPolicyInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::delete_cluster_policy::DeleteClusterPolicyInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                let input_1 = &_input.cluster_arn;
                let input_1 = input_1
                    .as_ref()
                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("cluster_arn", "cannot be empty or unset"))?;
                let cluster_arn = ::aws_smithy_http::label::fmt_string(input_1, ::aws_smithy_http::label::EncodingStrategy::Default);
                if cluster_arn.is_empty() {
                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                        "cluster_arn",
                        "cannot be empty or unset",
                    ));
                }
                ::std::write!(output, "/v1/clusters/{ClusterArn}/policy", ClusterArn = cluster_arn).expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::delete_cluster_policy::DeleteClusterPolicyInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from("");

        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct DeleteClusterPolicyEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteClusterPolicyEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "DeleteClusterPolicyEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context
            .input()
            .downcast_ref::<DeleteClusterPolicyInput>()
            .ok_or("failed to downcast to DeleteClusterPolicyInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}

// The get_* functions below are generated from JMESPath expressions in the
// operationContextParams trait. They target the operation's input shape.

/// Error type for the `DeleteClusterPolicyError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum DeleteClusterPolicyError {
    /// <p>Returns information about an error.</p>
    BadRequestException(crate::types::error::BadRequestException),
    /// <p>Returns information about an error.</p>
    ForbiddenException(crate::types::error::ForbiddenException),
    /// <p>Returns information about an error.</p>
    InternalServerErrorException(crate::types::error::InternalServerErrorException),
    /// <p>Returns information about an error.</p>
    NotFoundException(crate::types::error::NotFoundException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-DeleteClusterPolicyError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl DeleteClusterPolicyError {
    /// Creates the `DeleteClusterPolicyError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `DeleteClusterPolicyError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.clone().into(),
            meta: err,
        })
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::BadRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ForbiddenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InternalServerErrorException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::NotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `DeleteClusterPolicyError::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(self, Self::BadRequestException(_))
    }
    /// Returns `true` if the error kind is `DeleteClusterPolicyError::ForbiddenException`.
    pub fn is_forbidden_exception(&self) -> bool {
        matches!(self, Self::ForbiddenException(_))
    }
    /// Returns `true` if the error kind is `DeleteClusterPolicyError::InternalServerErrorException`.
    pub fn is_internal_server_error_exception(&self) -> bool {
        matches!(self, Self::InternalServerErrorException(_))
    }
    /// Returns `true` if the error kind is `DeleteClusterPolicyError::NotFoundException`.
    pub fn is_not_found_exception(&self) -> bool {
        matches!(self, Self::NotFoundException(_))
    }
}
impl ::std::error::Error for DeleteClusterPolicyError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::BadRequestException(_inner) => ::std::option::Option::Some(_inner),
            Self::ForbiddenException(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalServerErrorException(_inner) => ::std::option::Option::Some(_inner),
            Self::NotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for DeleteClusterPolicyError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::BadRequestException(_inner) => _inner.fmt(f),
            Self::ForbiddenException(_inner) => _inner.fmt(f),
            Self::InternalServerErrorException(_inner) => _inner.fmt(f),
            Self::NotFoundException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for DeleteClusterPolicyError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for DeleteClusterPolicyError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::BadRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ForbiddenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InternalServerErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::NotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for DeleteClusterPolicyError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source,
            meta: meta.unwrap_or_default(),
        })
    }
}
impl ::aws_types::request_id::RequestId for crate::operation::delete_cluster_policy::DeleteClusterPolicyError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::delete_cluster_policy::_delete_cluster_policy_output::DeleteClusterPolicyOutput;

pub use crate::operation::delete_cluster_policy::_delete_cluster_policy_input::DeleteClusterPolicyInput;

mod _delete_cluster_policy_input;

mod _delete_cluster_policy_output;

/// Builders
pub mod builders;
