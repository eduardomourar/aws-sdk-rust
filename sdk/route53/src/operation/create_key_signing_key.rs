// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `CreateKeySigningKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateKeySigningKey;
impl CreateKeySigningKey {
    /// Creates a new `CreateKeySigningKey`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::create_key_signing_key::CreateKeySigningKeyInput,
    ) -> ::std::result::Result<
        crate::operation::create_key_signing_key::CreateKeySigningKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_key_signing_key::CreateKeySigningKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::create_key_signing_key::CreateKeySigningKeyError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::create_key_signing_key::CreateKeySigningKeyOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::create_key_signing_key::CreateKeySigningKeyInput,
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
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("Route 53", "CreateKeySigningKey", input, runtime_plugins, stop_point)
            // Create a parent span for the entire operation. Includes a random, internal-only,
            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
            .instrument(::tracing::debug_span!(
                "Route 53.CreateKeySigningKey",
                "rpc.service" = "Route 53",
                "rpc.method" = "CreateKeySigningKey",
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateKeySigningKey {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("CreateKeySigningKey");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            CreateKeySigningKeyRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            CreateKeySigningKeyResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            crate::config::auth::Params::builder()
                .operation_name("CreateKeySigningKey")
                .build()
                .expect("required fields set"),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "CreateKeySigningKey",
            "Route 53",
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
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateKeySigningKey")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(CreateKeySigningKeyEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::create_key_signing_key::CreateKeySigningKeyError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::create_key_signing_key::CreateKeySigningKeyError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::create_key_signing_key::CreateKeySigningKeyError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct CreateKeySigningKeyResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for CreateKeySigningKeyResponseDeserializer {
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
        let parse_result = if !success && status != 201 || force_error {
            crate::protocol_serde::shape_create_key_signing_key::de_create_key_signing_key_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_create_key_signing_key::de_create_key_signing_key_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct CreateKeySigningKeyRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for CreateKeySigningKeyRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::create_key_signing_key::CreateKeySigningKeyInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::create_key_signing_key::CreateKeySigningKeyInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/2013-04-01/keysigningkey").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_key_signing_key::CreateKeySigningKeyInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/xml");
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_key_signing_key::ser_create_key_signing_key_op_input(
            &input,
        )?);
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct CreateKeySigningKeyEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateKeySigningKeyEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "CreateKeySigningKeyEndpointParamsInterceptor"
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
            .downcast_ref::<CreateKeySigningKeyInput>()
            .ok_or("failed to downcast to CreateKeySigningKeyInput")?;

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

/// Error type for the `CreateKeySigningKeyError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum CreateKeySigningKeyError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request.</p>
    ConcurrentModification(crate::types::error::ConcurrentModification),
    /// <p>Parameter name is not valid.</p>
    InvalidArgument(crate::types::error::InvalidArgument),
    /// <p>The input is not valid.</p>
    InvalidInput(crate::types::error::InvalidInput),
    /// <p>The key-signing key (KSK) name that you specified isn't a valid name.</p>
    InvalidKeySigningKeyName(crate::types::error::InvalidKeySigningKeyName),
    /// <p>The key-signing key (KSK) status isn't valid or another KSK has the status <code>INTERNAL_FAILURE</code>.</p>
    InvalidKeySigningKeyStatus(crate::types::error::InvalidKeySigningKeyStatus),
    /// <p>The KeyManagementServiceArn that you specified isn't valid to use with DNSSEC signing.</p>
    InvalidKmsArn(crate::types::error::InvalidKmsArn),
    /// <p>Your hosted zone status isn't valid for this operation. In the hosted zone, change the status to enable <code>DNSSEC</code> or disable <code>DNSSEC</code>.</p>
    InvalidSigningStatus(crate::types::error::InvalidSigningStatus),
    /// <p>You've already created a key-signing key (KSK) with this name or with the same customer managed key ARN.</p>
    KeySigningKeyAlreadyExists(crate::types::error::KeySigningKeyAlreadyExists),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(crate::types::error::NoSuchHostedZone),
    /// <p>You've reached the limit for the number of key-signing keys (KSKs). Remove at least one KSK, and then try again.</p>
    TooManyKeySigningKeys(crate::types::error::TooManyKeySigningKeys),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateKeySigningKeyError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl CreateKeySigningKeyError {
    /// Creates the `CreateKeySigningKeyError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `CreateKeySigningKeyError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
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
            Self::ConcurrentModification(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidArgument(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidInput(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidKeySigningKeyName(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidKeySigningKeyStatus(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidKmsArn(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidSigningStatus(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::KeySigningKeyAlreadyExists(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::NoSuchHostedZone(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::TooManyKeySigningKeys(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `CreateKeySigningKeyError::ConcurrentModification`.
    pub fn is_concurrent_modification(&self) -> bool {
        matches!(self, Self::ConcurrentModification(_))
    }
    /// Returns `true` if the error kind is `CreateKeySigningKeyError::InvalidArgument`.
    pub fn is_invalid_argument(&self) -> bool {
        matches!(self, Self::InvalidArgument(_))
    }
    /// Returns `true` if the error kind is `CreateKeySigningKeyError::InvalidInput`.
    pub fn is_invalid_input(&self) -> bool {
        matches!(self, Self::InvalidInput(_))
    }
    /// Returns `true` if the error kind is `CreateKeySigningKeyError::InvalidKeySigningKeyName`.
    pub fn is_invalid_key_signing_key_name(&self) -> bool {
        matches!(self, Self::InvalidKeySigningKeyName(_))
    }
    /// Returns `true` if the error kind is `CreateKeySigningKeyError::InvalidKeySigningKeyStatus`.
    pub fn is_invalid_key_signing_key_status(&self) -> bool {
        matches!(self, Self::InvalidKeySigningKeyStatus(_))
    }
    /// Returns `true` if the error kind is `CreateKeySigningKeyError::InvalidKmsArn`.
    pub fn is_invalid_kms_arn(&self) -> bool {
        matches!(self, Self::InvalidKmsArn(_))
    }
    /// Returns `true` if the error kind is `CreateKeySigningKeyError::InvalidSigningStatus`.
    pub fn is_invalid_signing_status(&self) -> bool {
        matches!(self, Self::InvalidSigningStatus(_))
    }
    /// Returns `true` if the error kind is `CreateKeySigningKeyError::KeySigningKeyAlreadyExists`.
    pub fn is_key_signing_key_already_exists(&self) -> bool {
        matches!(self, Self::KeySigningKeyAlreadyExists(_))
    }
    /// Returns `true` if the error kind is `CreateKeySigningKeyError::NoSuchHostedZone`.
    pub fn is_no_such_hosted_zone(&self) -> bool {
        matches!(self, Self::NoSuchHostedZone(_))
    }
    /// Returns `true` if the error kind is `CreateKeySigningKeyError::TooManyKeySigningKeys`.
    pub fn is_too_many_key_signing_keys(&self) -> bool {
        matches!(self, Self::TooManyKeySigningKeys(_))
    }
}
impl ::std::error::Error for CreateKeySigningKeyError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::ConcurrentModification(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidArgument(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidInput(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidKeySigningKeyName(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidKeySigningKeyStatus(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidKmsArn(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidSigningStatus(_inner) => ::std::option::Option::Some(_inner),
            Self::KeySigningKeyAlreadyExists(_inner) => ::std::option::Option::Some(_inner),
            Self::NoSuchHostedZone(_inner) => ::std::option::Option::Some(_inner),
            Self::TooManyKeySigningKeys(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for CreateKeySigningKeyError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::ConcurrentModification(_inner) => _inner.fmt(f),
            Self::InvalidArgument(_inner) => _inner.fmt(f),
            Self::InvalidInput(_inner) => _inner.fmt(f),
            Self::InvalidKeySigningKeyName(_inner) => _inner.fmt(f),
            Self::InvalidKeySigningKeyStatus(_inner) => _inner.fmt(f),
            Self::InvalidKmsArn(_inner) => _inner.fmt(f),
            Self::InvalidSigningStatus(_inner) => _inner.fmt(f),
            Self::KeySigningKeyAlreadyExists(_inner) => _inner.fmt(f),
            Self::NoSuchHostedZone(_inner) => _inner.fmt(f),
            Self::TooManyKeySigningKeys(_inner) => _inner.fmt(f),
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
impl ::aws_smithy_types::retry::ProvideErrorKind for CreateKeySigningKeyError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateKeySigningKeyError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ConcurrentModification(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidArgument(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidInput(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidKeySigningKeyName(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidKeySigningKeyStatus(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidKmsArn(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidSigningStatus(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::KeySigningKeyAlreadyExists(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::NoSuchHostedZone(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::TooManyKeySigningKeys(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for CreateKeySigningKeyError {
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
impl ::aws_types::request_id::RequestId for crate::operation::create_key_signing_key::CreateKeySigningKeyError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::create_key_signing_key::_create_key_signing_key_output::CreateKeySigningKeyOutput;

pub use crate::operation::create_key_signing_key::_create_key_signing_key_input::CreateKeySigningKeyInput;

mod _create_key_signing_key_input;

mod _create_key_signing_key_output;

/// Builders
pub mod builders;
