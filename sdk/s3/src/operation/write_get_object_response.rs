// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `WriteGetObjectResponse`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct WriteGetObjectResponse;
impl WriteGetObjectResponse {
    /// Creates a new `WriteGetObjectResponse`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::write_get_object_response::WriteGetObjectResponseInput,
    ) -> ::std::result::Result<
        crate::operation::write_get_object_response::WriteGetObjectResponseOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::write_get_object_response::WriteGetObjectResponseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::write_get_object_response::WriteGetObjectResponseError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::write_get_object_response::WriteGetObjectResponseOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::write_get_object_response::WriteGetObjectResponseInput,
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
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("S3", "WriteGetObjectResponse", input, runtime_plugins, stop_point)
            // Create a parent span for the entire operation. Includes a random, internal-only,
            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
            .instrument(::tracing::debug_span!(
                "S3.WriteGetObjectResponse",
                "rpc.service" = "S3",
                "rpc.method" = "WriteGetObjectResponse",
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for WriteGetObjectResponse {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("WriteGetObjectResponse");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            WriteGetObjectResponseRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            WriteGetObjectResponseResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            crate::config::auth::Params::builder()
                .operation_name("WriteGetObjectResponse")
                .build()
                .expect("required fields set"),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "WriteGetObjectResponse",
            "S3",
        ));
        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = true;
        signing_options.normalize_uri_path = false;
        signing_options.payload_override = Some(::aws_sigv4::http_request::SignableBody::UnsignedPayload);

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
        let mut rcb =
            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("WriteGetObjectResponse")
                .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
                .with_interceptor(WriteGetObjectResponseEndpointParamsInterceptor)
                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                    crate::operation::write_get_object_response::WriteGetObjectResponseError,
                >::new())
                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                    crate::operation::write_get_object_response::WriteGetObjectResponseError,
                >::new())
                .with_retry_classifier(
                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                        crate::operation::write_get_object_response::WriteGetObjectResponseError,
                    >::builder()
                    .transient_errors({
                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
                        transient_errors.push("InternalError");
                        ::std::borrow::Cow::Owned(transient_errors)
                    })
                    .build(),
                );

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct WriteGetObjectResponseResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for WriteGetObjectResponseResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        #[allow(unused_mut)]
        let mut force_error = false;
        ::tracing::debug!(extended_request_id = ?crate::s3_request_id::RequestIdExt::extended_request_id(response));
        if matches!(crate::rest_xml_unwrapped_errors::body_is_error(body), Ok(true)) {
            force_error = true;
        }
        ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 || force_error {
            crate::protocol_serde::shape_write_get_object_response::de_write_get_object_response_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_write_get_object_response::de_write_get_object_response_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct WriteGetObjectResponseRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for WriteGetObjectResponseRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::write_get_object_response::WriteGetObjectResponseInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::write_get_object_response::WriteGetObjectResponseInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/WriteGetObjectResponse").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::write_get_object_response::WriteGetObjectResponseInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                let builder = crate::protocol_serde::shape_write_get_object_response::ser_write_get_object_response_headers(input, builder)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/octet-stream");
            builder
        };
        let body = crate::protocol_serde::shape_write_get_object_response_input::ser_body_http_payload(input.body)?.into_inner();
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct WriteGetObjectResponseEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for WriteGetObjectResponseEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "WriteGetObjectResponseEndpointParamsInterceptor"
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
            .downcast_ref::<WriteGetObjectResponseInput>()
            .ok_or("failed to downcast to WriteGetObjectResponseInput")?;

        let endpoint_prefix = {
            let request_route = _input.request_route.as_deref().unwrap_or_default();
            if request_route.is_empty() {
                return Err(
                    ::aws_smithy_runtime_api::client::endpoint::error::InvalidEndpointError::failed_to_construct_uri(
                        "request_route was unset or empty but must be set as part of the endpoint prefix",
                    )
                    .into(),
                );
            }
            ::aws_smithy_runtime_api::client::endpoint::EndpointPrefix::new(format!("{RequestRoute}.", RequestRoute = request_route))
        }
        .map_err(|err| ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint prefix could not be built", err))?;
        cfg.interceptor_state().store_put(endpoint_prefix);

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .set_force_path_style(cfg.load::<crate::config::ForcePathStyle>().map(|ty| ty.0))
            .set_use_arn_region(cfg.load::<crate::config::UseArnRegion>().map(|ty| ty.0))
            .set_disable_multi_region_access_points(cfg.load::<crate::config::DisableMultiRegionAccessPoints>().map(|ty| ty.0))
            .set_accelerate(cfg.load::<crate::config::Accelerate>().map(|ty| ty.0))
            .set_disable_s3_express_session_auth(cfg.load::<crate::config::DisableS3ExpressSessionAuth>().map(|ty| ty.0))
            .set_use_object_lambda_endpoint(Some(true))
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

/// Error type for the `WriteGetObjectResponseError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum WriteGetObjectResponseError {
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-WriteGetObjectResponseError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl WriteGetObjectResponseError {
    /// Creates the `WriteGetObjectResponseError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `WriteGetObjectResponseError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
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
            Self::Unhandled(e) => &e.meta,
        }
    }
}
impl ::std::error::Error for WriteGetObjectResponseError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for WriteGetObjectResponseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
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
impl ::aws_smithy_types::retry::ProvideErrorKind for WriteGetObjectResponseError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for WriteGetObjectResponseError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for WriteGetObjectResponseError {
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
impl crate::s3_request_id::RequestIdExt for crate::operation::write_get_object_response::WriteGetObjectResponseError {
    fn extended_request_id(&self) -> Option<&str> {
        self.meta().extended_request_id()
    }
}
impl ::aws_types::request_id::RequestId for crate::operation::write_get_object_response::WriteGetObjectResponseError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::write_get_object_response::_write_get_object_response_output::WriteGetObjectResponseOutput;

pub use crate::operation::write_get_object_response::_write_get_object_response_input::WriteGetObjectResponseInput;

mod _write_get_object_response_input;

mod _write_get_object_response_output;

/// Builders
pub mod builders;
