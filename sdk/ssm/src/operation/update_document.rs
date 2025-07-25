// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `UpdateDocument`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateDocument;
impl UpdateDocument {
    /// Creates a new `UpdateDocument`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::update_document::UpdateDocumentInput,
    ) -> ::std::result::Result<
        crate::operation::update_document::UpdateDocumentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_document::UpdateDocumentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::update_document::UpdateDocumentError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::update_document::UpdateDocumentOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::update_document::UpdateDocumentInput,
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
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSM", "UpdateDocument", input, runtime_plugins, stop_point)
            // Create a parent span for the entire operation. Includes a random, internal-only,
            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
            .instrument(::tracing::debug_span!(
                "SSM.UpdateDocument",
                "rpc.service" = "SSM",
                "rpc.method" = "UpdateDocument",
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UpdateDocument {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("UpdateDocument");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            UpdateDocumentRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            UpdateDocumentResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            crate::config::auth::Params::builder()
                .operation_name("UpdateDocument")
                .build()
                .expect("required fields set"),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("UpdateDocument", "SSM"));
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
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateDocument")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(UpdateDocumentEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::update_document::UpdateDocumentError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::update_document::UpdateDocumentError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::update_document::UpdateDocumentError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct UpdateDocumentResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for UpdateDocumentResponseDeserializer {
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
            crate::protocol_serde::shape_update_document::de_update_document_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_update_document::de_update_document_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct UpdateDocumentRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for UpdateDocumentRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::update_document::UpdateDocumentInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::update_document::UpdateDocumentInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::update_document::UpdateDocumentInput,
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
                "AmazonSSM.UpdateDocument",
            );
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_document::ser_update_document_input(&input)?);
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct UpdateDocumentEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateDocumentEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "UpdateDocumentEndpointParamsInterceptor"
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
            .downcast_ref::<UpdateDocumentInput>()
            .ok_or("failed to downcast to UpdateDocumentInput")?;

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

/// Error type for the `UpdateDocumentError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum UpdateDocumentError {
    /// <p>The document has too many versions. Delete one or more document versions and try again.</p>
    DocumentVersionLimitExceeded(crate::types::error::DocumentVersionLimitExceeded),
    /// <p>The content of the association document matches another document. Change the content of the document and try again.</p>
    DuplicateDocumentContent(crate::types::error::DuplicateDocumentContent),
    /// <p>The version name has already been used in this document. Specify a different version name, and then try again.</p>
    DuplicateDocumentVersionName(crate::types::error::DuplicateDocumentVersionName),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::types::error::InternalServerError),
    /// <p>The specified SSM document doesn't exist.</p>
    InvalidDocument(crate::types::error::InvalidDocument),
    /// <p>The content for the document isn't valid.</p>
    InvalidDocumentContent(crate::types::error::InvalidDocumentContent),
    /// <p>You attempted to delete a document while it is still shared. You must stop sharing the document before you can delete it.</p>
    InvalidDocumentOperation(crate::types::error::InvalidDocumentOperation),
    /// <p>The version of the document schema isn't supported.</p>
    InvalidDocumentSchemaVersion(crate::types::error::InvalidDocumentSchemaVersion),
    /// <p>The document version isn't valid or doesn't exist.</p>
    InvalidDocumentVersion(crate::types::error::InvalidDocumentVersion),
    /// <p>The size limit of a document is 64 KB.</p>
    MaxDocumentSizeExceeded(crate::types::error::MaxDocumentSizeExceeded),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-UpdateDocumentError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl UpdateDocumentError {
    /// Creates the `UpdateDocumentError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `UpdateDocumentError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
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
            Self::DocumentVersionLimitExceeded(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DuplicateDocumentContent(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DuplicateDocumentVersionName(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InternalServerError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidDocument(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidDocumentContent(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidDocumentOperation(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidDocumentSchemaVersion(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidDocumentVersion(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::MaxDocumentSizeExceeded(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `UpdateDocumentError::DocumentVersionLimitExceeded`.
    pub fn is_document_version_limit_exceeded(&self) -> bool {
        matches!(self, Self::DocumentVersionLimitExceeded(_))
    }
    /// Returns `true` if the error kind is `UpdateDocumentError::DuplicateDocumentContent`.
    pub fn is_duplicate_document_content(&self) -> bool {
        matches!(self, Self::DuplicateDocumentContent(_))
    }
    /// Returns `true` if the error kind is `UpdateDocumentError::DuplicateDocumentVersionName`.
    pub fn is_duplicate_document_version_name(&self) -> bool {
        matches!(self, Self::DuplicateDocumentVersionName(_))
    }
    /// Returns `true` if the error kind is `UpdateDocumentError::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(self, Self::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `UpdateDocumentError::InvalidDocument`.
    pub fn is_invalid_document(&self) -> bool {
        matches!(self, Self::InvalidDocument(_))
    }
    /// Returns `true` if the error kind is `UpdateDocumentError::InvalidDocumentContent`.
    pub fn is_invalid_document_content(&self) -> bool {
        matches!(self, Self::InvalidDocumentContent(_))
    }
    /// Returns `true` if the error kind is `UpdateDocumentError::InvalidDocumentOperation`.
    pub fn is_invalid_document_operation(&self) -> bool {
        matches!(self, Self::InvalidDocumentOperation(_))
    }
    /// Returns `true` if the error kind is `UpdateDocumentError::InvalidDocumentSchemaVersion`.
    pub fn is_invalid_document_schema_version(&self) -> bool {
        matches!(self, Self::InvalidDocumentSchemaVersion(_))
    }
    /// Returns `true` if the error kind is `UpdateDocumentError::InvalidDocumentVersion`.
    pub fn is_invalid_document_version(&self) -> bool {
        matches!(self, Self::InvalidDocumentVersion(_))
    }
    /// Returns `true` if the error kind is `UpdateDocumentError::MaxDocumentSizeExceeded`.
    pub fn is_max_document_size_exceeded(&self) -> bool {
        matches!(self, Self::MaxDocumentSizeExceeded(_))
    }
}
impl ::std::error::Error for UpdateDocumentError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::DocumentVersionLimitExceeded(_inner) => ::std::option::Option::Some(_inner),
            Self::DuplicateDocumentContent(_inner) => ::std::option::Option::Some(_inner),
            Self::DuplicateDocumentVersionName(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalServerError(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidDocument(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidDocumentContent(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidDocumentOperation(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidDocumentSchemaVersion(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidDocumentVersion(_inner) => ::std::option::Option::Some(_inner),
            Self::MaxDocumentSizeExceeded(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for UpdateDocumentError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::DocumentVersionLimitExceeded(_inner) => _inner.fmt(f),
            Self::DuplicateDocumentContent(_inner) => _inner.fmt(f),
            Self::DuplicateDocumentVersionName(_inner) => _inner.fmt(f),
            Self::InternalServerError(_inner) => _inner.fmt(f),
            Self::InvalidDocument(_inner) => _inner.fmt(f),
            Self::InvalidDocumentContent(_inner) => _inner.fmt(f),
            Self::InvalidDocumentOperation(_inner) => _inner.fmt(f),
            Self::InvalidDocumentSchemaVersion(_inner) => _inner.fmt(f),
            Self::InvalidDocumentVersion(_inner) => _inner.fmt(f),
            Self::MaxDocumentSizeExceeded(_inner) => _inner.fmt(f),
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
impl ::aws_smithy_types::retry::ProvideErrorKind for UpdateDocumentError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for UpdateDocumentError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::DocumentVersionLimitExceeded(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DuplicateDocumentContent(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DuplicateDocumentVersionName(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InternalServerError(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidDocument(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidDocumentContent(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidDocumentOperation(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidDocumentSchemaVersion(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidDocumentVersion(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::MaxDocumentSizeExceeded(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for UpdateDocumentError {
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
impl ::aws_types::request_id::RequestId for crate::operation::update_document::UpdateDocumentError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::update_document::_update_document_output::UpdateDocumentOutput;

pub use crate::operation::update_document::_update_document_input::UpdateDocumentInput;

mod _update_document_input;

mod _update_document_output;

/// Builders
pub mod builders;
