// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `StartADAssessment`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartADAssessment;
impl StartADAssessment {
    /// Creates a new `StartADAssessment`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::start_ad_assessment::StartAdAssessmentInput,
    ) -> ::std::result::Result<
        crate::operation::start_ad_assessment::StartAdAssessmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_ad_assessment::StartADAssessmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::start_ad_assessment::StartADAssessmentError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::start_ad_assessment::StartAdAssessmentOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::start_ad_assessment::StartAdAssessmentInput,
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
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
            "Directory Service",
            "StartADAssessment",
            input,
            runtime_plugins,
            stop_point,
        )
        // Create a parent span for the entire operation. Includes a random, internal-only,
        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
        .instrument(::tracing::debug_span!(
            "Directory Service.StartADAssessment",
            "rpc.service" = "Directory Service",
            "rpc.method" = "StartADAssessment",
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for StartADAssessment {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("StartADAssessment");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            StartADAssessmentRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            StartADAssessmentResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            crate::config::auth::Params::builder()
                .operation_name("StartADAssessment")
                .build()
                .expect("required fields set"),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "StartADAssessment",
            "Directory Service",
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
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("StartADAssessment")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(StartADAssessmentEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::start_ad_assessment::StartADAssessmentError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::start_ad_assessment::StartADAssessmentError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::start_ad_assessment::StartADAssessmentError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct StartADAssessmentResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for StartADAssessmentResponseDeserializer {
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
            crate::protocol_serde::shape_start_ad_assessment::de_start_ad_assessment_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_start_ad_assessment::de_start_ad_assessment_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct StartADAssessmentRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for StartADAssessmentRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::start_ad_assessment::StartAdAssessmentInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::start_ad_assessment::StartAdAssessmentInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::start_ad_assessment::StartAdAssessmentInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/x-amz-json-1.1");
            builder = _header_serialization_settings.set_default_header(
                builder,
                ::http::header::HeaderName::from_static("x-amz-target"),
                "DirectoryService_20150416.StartADAssessment",
            );
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_start_ad_assessment::ser_start_ad_assessment_input(&input)?);
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct StartADAssessmentEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for StartADAssessmentEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "StartADAssessmentEndpointParamsInterceptor"
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
            .downcast_ref::<StartAdAssessmentInput>()
            .ok_or("failed to downcast to StartAdAssessmentInput")?;

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

/// Error type for the `StartADAssessmentError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum StartADAssessmentError {
    /// <p>A directory assessment is automatically created when you create a hybrid directory. There are two types of assessments: <code>CUSTOMER</code> and <code>SYSTEM</code>. Your Amazon Web Services account has a limit of 100 <code>CUSTOMER</code> directory assessments.</p>
    /// <p>If you attempt to create a hybrid directory; and you already have 100 <code>CUSTOMER</code> directory assessments;, you will encounter an error. Delete assessments to free up capacity before trying again.</p>
    /// <p>You can request an increase to your <code>CUSTOMER</code> directory assessment quota by contacting customer support or delete existing CUSTOMER directory assessments; to free up capacity.</p>
    AdAssessmentLimitExceededException(crate::types::error::AdAssessmentLimitExceededException),
    /// <p>A client exception has occurred.</p>
    ClientException(crate::types::error::ClientException),
    /// <p>The specified directory does not exist in the system.</p>
    DirectoryDoesNotExistException(crate::types::error::DirectoryDoesNotExistException),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameterException(crate::types::error::InvalidParameterException),
    /// <p>An exception has occurred in Directory Service.</p>
    ServiceException(crate::types::error::ServiceException),
    /// <p>The operation is not supported.</p>
    UnsupportedOperationException(crate::types::error::UnsupportedOperationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-StartADAssessmentError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl StartADAssessmentError {
    /// Creates the `StartADAssessmentError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `StartADAssessmentError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
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
            Self::AdAssessmentLimitExceededException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DirectoryDoesNotExistException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidParameterException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ServiceException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::UnsupportedOperationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `StartADAssessmentError::AdAssessmentLimitExceededException`.
    pub fn is_ad_assessment_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::AdAssessmentLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `StartADAssessmentError::ClientException`.
    pub fn is_client_exception(&self) -> bool {
        matches!(self, Self::ClientException(_))
    }
    /// Returns `true` if the error kind is `StartADAssessmentError::DirectoryDoesNotExistException`.
    pub fn is_directory_does_not_exist_exception(&self) -> bool {
        matches!(self, Self::DirectoryDoesNotExistException(_))
    }
    /// Returns `true` if the error kind is `StartADAssessmentError::InvalidParameterException`.
    pub fn is_invalid_parameter_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterException(_))
    }
    /// Returns `true` if the error kind is `StartADAssessmentError::ServiceException`.
    pub fn is_service_exception(&self) -> bool {
        matches!(self, Self::ServiceException(_))
    }
    /// Returns `true` if the error kind is `StartADAssessmentError::UnsupportedOperationException`.
    pub fn is_unsupported_operation_exception(&self) -> bool {
        matches!(self, Self::UnsupportedOperationException(_))
    }
}
impl ::std::error::Error for StartADAssessmentError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::AdAssessmentLimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::ClientException(_inner) => ::std::option::Option::Some(_inner),
            Self::DirectoryDoesNotExistException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidParameterException(_inner) => ::std::option::Option::Some(_inner),
            Self::ServiceException(_inner) => ::std::option::Option::Some(_inner),
            Self::UnsupportedOperationException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for StartADAssessmentError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::AdAssessmentLimitExceededException(_inner) => _inner.fmt(f),
            Self::ClientException(_inner) => _inner.fmt(f),
            Self::DirectoryDoesNotExistException(_inner) => _inner.fmt(f),
            Self::InvalidParameterException(_inner) => _inner.fmt(f),
            Self::ServiceException(_inner) => _inner.fmt(f),
            Self::UnsupportedOperationException(_inner) => _inner.fmt(f),
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
impl ::aws_smithy_types::retry::ProvideErrorKind for StartADAssessmentError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for StartADAssessmentError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AdAssessmentLimitExceededException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DirectoryDoesNotExistException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidParameterException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ServiceException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::UnsupportedOperationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for StartADAssessmentError {
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
impl ::aws_types::request_id::RequestId for crate::operation::start_ad_assessment::StartADAssessmentError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::start_ad_assessment::_start_ad_assessment_output::StartAdAssessmentOutput;

pub use crate::operation::start_ad_assessment::_start_ad_assessment_input::StartAdAssessmentInput;

mod _start_ad_assessment_input;

mod _start_ad_assessment_output;

/// Builders
pub mod builders;
