// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `RequestCertificate`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RequestCertificate;
impl RequestCertificate {
    /// Creates a new `RequestCertificate`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::request_certificate::RequestCertificateInput,
    ) -> ::std::result::Result<
        crate::operation::request_certificate::RequestCertificateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::request_certificate::RequestCertificateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::request_certificate::RequestCertificateError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::request_certificate::RequestCertificateOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::request_certificate::RequestCertificateInput,
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
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("ACM", "RequestCertificate", input, runtime_plugins, stop_point)
            // Create a parent span for the entire operation. Includes a random, internal-only,
            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
            .instrument(::tracing::debug_span!(
                "ACM.RequestCertificate",
                "rpc.service" = "ACM",
                "rpc.method" = "RequestCertificate",
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for RequestCertificate {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("RequestCertificate");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            RequestCertificateRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            RequestCertificateResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            crate::config::auth::Params::builder()
                .operation_name("RequestCertificate")
                .build()
                .expect("required fields set"),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("RequestCertificate", "ACM"));
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
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("RequestCertificate")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(RequestCertificateEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::request_certificate::RequestCertificateError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::request_certificate::RequestCertificateError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::request_certificate::RequestCertificateError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct RequestCertificateResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for RequestCertificateResponseDeserializer {
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
            crate::protocol_serde::shape_request_certificate::de_request_certificate_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_request_certificate::de_request_certificate_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct RequestCertificateRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for RequestCertificateRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::request_certificate::RequestCertificateInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::request_certificate::RequestCertificateInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::request_certificate::RequestCertificateInput,
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
                "CertificateManager.RequestCertificate",
            );
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_request_certificate::ser_request_certificate_input(&input)?);
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct RequestCertificateEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for RequestCertificateEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "RequestCertificateEndpointParamsInterceptor"
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
            .downcast_ref::<RequestCertificateInput>()
            .ok_or("failed to downcast to RequestCertificateInput")?;

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

/// Error type for the `RequestCertificateError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum RequestCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArnException(crate::types::error::InvalidArnException),
    /// <p>One or more values in the <code>DomainValidationOption</code> structure is incorrect.</p>
    InvalidDomainValidationOptionsException(crate::types::error::InvalidDomainValidationOptionsException),
    /// <p>An input parameter was invalid.</p>
    InvalidParameterException(crate::types::error::InvalidParameterException),
    /// <p>One or both of the values that make up the key-value pair is not valid. For example, you cannot specify a tag value that begins with <code>aws:</code>.</p>
    InvalidTagException(crate::types::error::InvalidTagException),
    /// <p>An ACM quota has been exceeded.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// <p>A specified tag did not comply with an existing tag policy and was rejected.</p>
    TagPolicyException(crate::types::error::TagPolicyException),
    /// <p>The request contains too many tags. Try the request again with fewer tags.</p>
    TooManyTagsException(crate::types::error::TooManyTagsException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-RequestCertificateError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl RequestCertificateError {
    /// Creates the `RequestCertificateError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `RequestCertificateError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
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
            Self::InvalidArnException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidDomainValidationOptionsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidParameterException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidTagException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::LimitExceededException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::TagPolicyException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::TooManyTagsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `RequestCertificateError::InvalidArnException`.
    pub fn is_invalid_arn_exception(&self) -> bool {
        matches!(self, Self::InvalidArnException(_))
    }
    /// Returns `true` if the error kind is `RequestCertificateError::InvalidDomainValidationOptionsException`.
    pub fn is_invalid_domain_validation_options_exception(&self) -> bool {
        matches!(self, Self::InvalidDomainValidationOptionsException(_))
    }
    /// Returns `true` if the error kind is `RequestCertificateError::InvalidParameterException`.
    pub fn is_invalid_parameter_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterException(_))
    }
    /// Returns `true` if the error kind is `RequestCertificateError::InvalidTagException`.
    pub fn is_invalid_tag_exception(&self) -> bool {
        matches!(self, Self::InvalidTagException(_))
    }
    /// Returns `true` if the error kind is `RequestCertificateError::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::LimitExceededException(_))
    }
    /// Returns `true` if the error kind is `RequestCertificateError::TagPolicyException`.
    pub fn is_tag_policy_exception(&self) -> bool {
        matches!(self, Self::TagPolicyException(_))
    }
    /// Returns `true` if the error kind is `RequestCertificateError::TooManyTagsException`.
    pub fn is_too_many_tags_exception(&self) -> bool {
        matches!(self, Self::TooManyTagsException(_))
    }
}
impl ::std::error::Error for RequestCertificateError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InvalidArnException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidDomainValidationOptionsException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidParameterException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidTagException(_inner) => ::std::option::Option::Some(_inner),
            Self::LimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::TagPolicyException(_inner) => ::std::option::Option::Some(_inner),
            Self::TooManyTagsException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for RequestCertificateError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InvalidArnException(_inner) => _inner.fmt(f),
            Self::InvalidDomainValidationOptionsException(_inner) => _inner.fmt(f),
            Self::InvalidParameterException(_inner) => _inner.fmt(f),
            Self::InvalidTagException(_inner) => _inner.fmt(f),
            Self::LimitExceededException(_inner) => _inner.fmt(f),
            Self::TagPolicyException(_inner) => _inner.fmt(f),
            Self::TooManyTagsException(_inner) => _inner.fmt(f),
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
impl ::aws_smithy_types::retry::ProvideErrorKind for RequestCertificateError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for RequestCertificateError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InvalidArnException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidDomainValidationOptionsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidParameterException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidTagException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::LimitExceededException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::TagPolicyException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::TooManyTagsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for RequestCertificateError {
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
impl ::aws_types::request_id::RequestId for crate::operation::request_certificate::RequestCertificateError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::request_certificate::_request_certificate_output::RequestCertificateOutput;

pub use crate::operation::request_certificate::_request_certificate_input::RequestCertificateInput;

mod _request_certificate_input;

mod _request_certificate_output;

/// Builders
pub mod builders;
