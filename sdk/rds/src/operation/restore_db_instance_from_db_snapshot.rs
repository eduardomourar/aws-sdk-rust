// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `RestoreDBInstanceFromDBSnapshot`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RestoreDBInstanceFromDBSnapshot;
impl RestoreDBInstanceFromDBSnapshot {
    /// Creates a new `RestoreDBInstanceFromDBSnapshot`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::restore_db_instance_from_db_snapshot::RestoreDbInstanceFromDbSnapshotInput,
    ) -> ::std::result::Result<
        crate::operation::restore_db_instance_from_db_snapshot::RestoreDbInstanceFromDbSnapshotOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restore_db_instance_from_db_snapshot::RestoreDBInstanceFromDBSnapshotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::restore_db_instance_from_db_snapshot::RestoreDBInstanceFromDBSnapshotError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::restore_db_instance_from_db_snapshot::RestoreDbInstanceFromDbSnapshotOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::restore_db_instance_from_db_snapshot::RestoreDbInstanceFromDbSnapshotInput,
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
            "RDS",
            "RestoreDBInstanceFromDBSnapshot",
            input,
            runtime_plugins,
            stop_point,
        )
        // Create a parent span for the entire operation. Includes a random, internal-only,
        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
        .instrument(::tracing::debug_span!(
            "RDS.RestoreDBInstanceFromDBSnapshot",
            "rpc.service" = "RDS",
            "rpc.method" = "RestoreDBInstanceFromDBSnapshot",
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for RestoreDBInstanceFromDBSnapshot {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("RestoreDBInstanceFromDBSnapshot");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            RestoreDBInstanceFromDBSnapshotRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            RestoreDBInstanceFromDBSnapshotResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            crate::config::auth::Params::builder()
                .operation_name("RestoreDBInstanceFromDBSnapshot")
                .build()
                .expect("required fields set"),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "RestoreDBInstanceFromDBSnapshot",
            "RDS",
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
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("RestoreDBInstanceFromDBSnapshot")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(RestoreDBInstanceFromDBSnapshotEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::restore_db_instance_from_db_snapshot::RestoreDBInstanceFromDBSnapshotError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::restore_db_instance_from_db_snapshot::RestoreDBInstanceFromDBSnapshotError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::restore_db_instance_from_db_snapshot::RestoreDBInstanceFromDBSnapshotError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct RestoreDBInstanceFromDBSnapshotResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for RestoreDBInstanceFromDBSnapshotResponseDeserializer {
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
            crate::protocol_serde::shape_restore_db_instance_from_db_snapshot::de_restore_db_instance_from_db_snapshot_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_restore_db_instance_from_db_snapshot::de_restore_db_instance_from_db_snapshot_http_response(
                status, headers, body,
            )
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct RestoreDBInstanceFromDBSnapshotRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for RestoreDBInstanceFromDBSnapshotRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::restore_db_instance_from_db_snapshot::RestoreDbInstanceFromDbSnapshotInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::restore_db_instance_from_db_snapshot::RestoreDbInstanceFromDbSnapshotInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::restore_db_instance_from_db_snapshot::RestoreDbInstanceFromDbSnapshotInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(
            crate::protocol_serde::shape_restore_db_instance_from_db_snapshot_input::ser_restore_db_instance_from_db_snapshot_input_input_input(
                &input,
            )?,
        );
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct RestoreDBInstanceFromDBSnapshotEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for RestoreDBInstanceFromDBSnapshotEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "RestoreDBInstanceFromDBSnapshotEndpointParamsInterceptor"
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
            .downcast_ref::<RestoreDbInstanceFromDbSnapshotInput>()
            .ok_or("failed to downcast to RestoreDbInstanceFromDbSnapshotInput")?;

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

/// Error type for the `RestoreDBInstanceFromDBSnapshotError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum RestoreDBInstanceFromDBSnapshotError {
    /// <p>The specified CIDR IP range or Amazon EC2 security group might not be authorized for the specified DB security group.</p>
    /// <p>Or, RDS might not be authorized to perform necessary actions using IAM on your behalf.</p>
    AuthorizationNotFoundFault(crate::types::error::AuthorizationNotFoundFault),
    #[allow(missing_docs)] // documentation missing in model
    #[deprecated(note = "Please avoid using this fault")]
    BackupPolicyNotFoundFault(crate::types::error::BackupPolicyNotFoundFault),
    /// <p><code>CertificateIdentifier</code> doesn't refer to an existing certificate.</p>
    CertificateNotFoundFault(crate::types::error::CertificateNotFoundFault),
    /// <p><code>DBClusterSnapshotIdentifier</code> doesn't refer to an existing DB cluster snapshot.</p>
    DbClusterSnapshotNotFoundFault(crate::types::error::DbClusterSnapshotNotFoundFault),
    /// <p>The user already has a DB instance with the given identifier.</p>
    DbInstanceAlreadyExistsFault(crate::types::error::DbInstanceAlreadyExistsFault),
    /// <p><code>DBParameterGroupName</code> doesn't refer to an existing DB parameter group.</p>
    DbParameterGroupNotFoundFault(crate::types::error::DbParameterGroupNotFoundFault),
    /// <p><code>DBSecurityGroupName</code> doesn't refer to an existing DB security group.</p>
    DbSecurityGroupNotFoundFault(crate::types::error::DbSecurityGroupNotFoundFault),
    /// <p><code>DBSnapshotIdentifier</code> doesn't refer to an existing DB snapshot.</p>
    DbSnapshotNotFoundFault(crate::types::error::DbSnapshotNotFoundFault),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DbSubnetGroupDoesNotCoverEnoughAZs(crate::types::error::DbSubnetGroupDoesNotCoverEnoughAZs),
    /// <p><code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group.</p>
    DbSubnetGroupNotFoundFault(crate::types::error::DbSubnetGroupNotFoundFault),
    /// <p><code>Domain</code> doesn't refer to an existing Active Directory domain.</p>
    DomainNotFoundFault(crate::types::error::DomainNotFoundFault),
    /// <p>The request would result in the user exceeding the allowed number of DB instances.</p>
    InstanceQuotaExceededFault(crate::types::error::InstanceQuotaExceededFault),
    /// <p>The specified DB instance class isn't available in the specified Availability Zone.</p>
    InsufficientDbInstanceCapacityFault(crate::types::error::InsufficientDbInstanceCapacityFault),
    /// <p>The state of the DB snapshot doesn't allow deletion.</p>
    InvalidDbSnapshotStateFault(crate::types::error::InvalidDbSnapshotStateFault),
    /// <p>Cannot restore from VPC backup to non-VPC DB instance.</p>
    InvalidRestoreFault(crate::types::error::InvalidRestoreFault),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(crate::types::error::InvalidSubnet),
    /// <p>The DB subnet group doesn't cover all Availability Zones after it's created because of users' change.</p>
    InvalidVpcNetworkStateFault(crate::types::error::InvalidVpcNetworkStateFault),
    /// <p>An error occurred accessing an Amazon Web Services KMS key.</p>
    KmsKeyNotAccessibleFault(crate::types::error::KmsKeyNotAccessibleFault),
    /// <p>The network type is invalid for the DB instance. Valid nework type values are <code>IPV4</code> and <code>DUAL</code>.</p>
    NetworkTypeNotSupported(crate::types::error::NetworkTypeNotSupported),
    /// <p>The specified option group could not be found.</p>
    OptionGroupNotFoundFault(crate::types::error::OptionGroupNotFoundFault),
    /// <p>Provisioned IOPS not available in the specified Availability Zone.</p>
    ProvisionedIopsNotAvailableInAzFault(crate::types::error::ProvisionedIopsNotAvailableInAzFault),
    /// <p>The request would result in the user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(crate::types::error::StorageQuotaExceededFault),
    /// <p>The specified <code>StorageType</code> can't be associated with the DB instance.</p>
    StorageTypeNotSupportedFault(crate::types::error::StorageTypeNotSupportedFault),
    /// <p>You attempted to create more tenant databases than are permitted in your Amazon Web Services account.</p>
    TenantDatabaseQuotaExceededFault(crate::types::error::TenantDatabaseQuotaExceededFault),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-RestoreDBInstanceFromDBSnapshotError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl RestoreDBInstanceFromDBSnapshotError {
    /// Creates the `RestoreDBInstanceFromDBSnapshotError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `RestoreDBInstanceFromDBSnapshotError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
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
            Self::AuthorizationNotFoundFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::BackupPolicyNotFoundFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::CertificateNotFoundFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DbClusterSnapshotNotFoundFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DbInstanceAlreadyExistsFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DbParameterGroupNotFoundFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DbSecurityGroupNotFoundFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DbSnapshotNotFoundFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DbSubnetGroupDoesNotCoverEnoughAZs(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DbSubnetGroupNotFoundFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DomainNotFoundFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InstanceQuotaExceededFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InsufficientDbInstanceCapacityFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidDbSnapshotStateFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidRestoreFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidSubnet(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidVpcNetworkStateFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::KmsKeyNotAccessibleFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::NetworkTypeNotSupported(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::OptionGroupNotFoundFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ProvisionedIopsNotAvailableInAzFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::StorageQuotaExceededFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::StorageTypeNotSupportedFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::TenantDatabaseQuotaExceededFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::AuthorizationNotFoundFault`.
    pub fn is_authorization_not_found_fault(&self) -> bool {
        matches!(self, Self::AuthorizationNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::BackupPolicyNotFoundFault`.
    pub fn is_backup_policy_not_found_fault(&self) -> bool {
        matches!(self, Self::BackupPolicyNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::CertificateNotFoundFault`.
    pub fn is_certificate_not_found_fault(&self) -> bool {
        matches!(self, Self::CertificateNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::DbClusterSnapshotNotFoundFault`.
    pub fn is_db_cluster_snapshot_not_found_fault(&self) -> bool {
        matches!(self, Self::DbClusterSnapshotNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::DbInstanceAlreadyExistsFault`.
    pub fn is_db_instance_already_exists_fault(&self) -> bool {
        matches!(self, Self::DbInstanceAlreadyExistsFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::DbParameterGroupNotFoundFault`.
    pub fn is_db_parameter_group_not_found_fault(&self) -> bool {
        matches!(self, Self::DbParameterGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::DbSecurityGroupNotFoundFault`.
    pub fn is_db_security_group_not_found_fault(&self) -> bool {
        matches!(self, Self::DbSecurityGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::DbSnapshotNotFoundFault`.
    pub fn is_db_snapshot_not_found_fault(&self) -> bool {
        matches!(self, Self::DbSnapshotNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::DbSubnetGroupDoesNotCoverEnoughAZs`.
    pub fn is_db_subnet_group_does_not_cover_enough_azs(&self) -> bool {
        matches!(self, Self::DbSubnetGroupDoesNotCoverEnoughAZs(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::DbSubnetGroupNotFoundFault`.
    pub fn is_db_subnet_group_not_found_fault(&self) -> bool {
        matches!(self, Self::DbSubnetGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::DomainNotFoundFault`.
    pub fn is_domain_not_found_fault(&self) -> bool {
        matches!(self, Self::DomainNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::InstanceQuotaExceededFault`.
    pub fn is_instance_quota_exceeded_fault(&self) -> bool {
        matches!(self, Self::InstanceQuotaExceededFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::InsufficientDbInstanceCapacityFault`.
    pub fn is_insufficient_db_instance_capacity_fault(&self) -> bool {
        matches!(self, Self::InsufficientDbInstanceCapacityFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::InvalidDbSnapshotStateFault`.
    pub fn is_invalid_db_snapshot_state_fault(&self) -> bool {
        matches!(self, Self::InvalidDbSnapshotStateFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::InvalidRestoreFault`.
    pub fn is_invalid_restore_fault(&self) -> bool {
        matches!(self, Self::InvalidRestoreFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::InvalidSubnet`.
    pub fn is_invalid_subnet(&self) -> bool {
        matches!(self, Self::InvalidSubnet(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::InvalidVpcNetworkStateFault`.
    pub fn is_invalid_vpc_network_state_fault(&self) -> bool {
        matches!(self, Self::InvalidVpcNetworkStateFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::KmsKeyNotAccessibleFault`.
    pub fn is_kms_key_not_accessible_fault(&self) -> bool {
        matches!(self, Self::KmsKeyNotAccessibleFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::NetworkTypeNotSupported`.
    pub fn is_network_type_not_supported(&self) -> bool {
        matches!(self, Self::NetworkTypeNotSupported(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::OptionGroupNotFoundFault`.
    pub fn is_option_group_not_found_fault(&self) -> bool {
        matches!(self, Self::OptionGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::ProvisionedIopsNotAvailableInAzFault`.
    pub fn is_provisioned_iops_not_available_in_az_fault(&self) -> bool {
        matches!(self, Self::ProvisionedIopsNotAvailableInAzFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::StorageQuotaExceededFault`.
    pub fn is_storage_quota_exceeded_fault(&self) -> bool {
        matches!(self, Self::StorageQuotaExceededFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::StorageTypeNotSupportedFault`.
    pub fn is_storage_type_not_supported_fault(&self) -> bool {
        matches!(self, Self::StorageTypeNotSupportedFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBInstanceFromDBSnapshotError::TenantDatabaseQuotaExceededFault`.
    pub fn is_tenant_database_quota_exceeded_fault(&self) -> bool {
        matches!(self, Self::TenantDatabaseQuotaExceededFault(_))
    }
}
impl ::std::error::Error for RestoreDBInstanceFromDBSnapshotError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::AuthorizationNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::BackupPolicyNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::CertificateNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbClusterSnapshotNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbInstanceAlreadyExistsFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbParameterGroupNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbSecurityGroupNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbSnapshotNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbSubnetGroupDoesNotCoverEnoughAZs(_inner) => ::std::option::Option::Some(_inner),
            Self::DbSubnetGroupNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DomainNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InstanceQuotaExceededFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InsufficientDbInstanceCapacityFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidDbSnapshotStateFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidRestoreFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidSubnet(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidVpcNetworkStateFault(_inner) => ::std::option::Option::Some(_inner),
            Self::KmsKeyNotAccessibleFault(_inner) => ::std::option::Option::Some(_inner),
            Self::NetworkTypeNotSupported(_inner) => ::std::option::Option::Some(_inner),
            Self::OptionGroupNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::ProvisionedIopsNotAvailableInAzFault(_inner) => ::std::option::Option::Some(_inner),
            Self::StorageQuotaExceededFault(_inner) => ::std::option::Option::Some(_inner),
            Self::StorageTypeNotSupportedFault(_inner) => ::std::option::Option::Some(_inner),
            Self::TenantDatabaseQuotaExceededFault(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for RestoreDBInstanceFromDBSnapshotError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::AuthorizationNotFoundFault(_inner) => _inner.fmt(f),
            Self::BackupPolicyNotFoundFault(_inner) => _inner.fmt(f),
            Self::CertificateNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbClusterSnapshotNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbInstanceAlreadyExistsFault(_inner) => _inner.fmt(f),
            Self::DbParameterGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbSecurityGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbSnapshotNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbSubnetGroupDoesNotCoverEnoughAZs(_inner) => _inner.fmt(f),
            Self::DbSubnetGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::DomainNotFoundFault(_inner) => _inner.fmt(f),
            Self::InstanceQuotaExceededFault(_inner) => _inner.fmt(f),
            Self::InsufficientDbInstanceCapacityFault(_inner) => _inner.fmt(f),
            Self::InvalidDbSnapshotStateFault(_inner) => _inner.fmt(f),
            Self::InvalidRestoreFault(_inner) => _inner.fmt(f),
            Self::InvalidSubnet(_inner) => _inner.fmt(f),
            Self::InvalidVpcNetworkStateFault(_inner) => _inner.fmt(f),
            Self::KmsKeyNotAccessibleFault(_inner) => _inner.fmt(f),
            Self::NetworkTypeNotSupported(_inner) => _inner.fmt(f),
            Self::OptionGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::ProvisionedIopsNotAvailableInAzFault(_inner) => _inner.fmt(f),
            Self::StorageQuotaExceededFault(_inner) => _inner.fmt(f),
            Self::StorageTypeNotSupportedFault(_inner) => _inner.fmt(f),
            Self::TenantDatabaseQuotaExceededFault(_inner) => _inner.fmt(f),
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
impl ::aws_smithy_types::retry::ProvideErrorKind for RestoreDBInstanceFromDBSnapshotError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for RestoreDBInstanceFromDBSnapshotError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AuthorizationNotFoundFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::BackupPolicyNotFoundFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::CertificateNotFoundFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DbClusterSnapshotNotFoundFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DbInstanceAlreadyExistsFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DbParameterGroupNotFoundFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DbSecurityGroupNotFoundFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DbSnapshotNotFoundFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DbSubnetGroupDoesNotCoverEnoughAZs(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DbSubnetGroupNotFoundFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DomainNotFoundFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InstanceQuotaExceededFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InsufficientDbInstanceCapacityFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidDbSnapshotStateFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidRestoreFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidSubnet(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidVpcNetworkStateFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::KmsKeyNotAccessibleFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::NetworkTypeNotSupported(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::OptionGroupNotFoundFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ProvisionedIopsNotAvailableInAzFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::StorageQuotaExceededFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::StorageTypeNotSupportedFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::TenantDatabaseQuotaExceededFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for RestoreDBInstanceFromDBSnapshotError {
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
impl ::aws_types::request_id::RequestId for crate::operation::restore_db_instance_from_db_snapshot::RestoreDBInstanceFromDBSnapshotError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::restore_db_instance_from_db_snapshot::_restore_db_instance_from_db_snapshot_output::RestoreDbInstanceFromDbSnapshotOutput;

pub use crate::operation::restore_db_instance_from_db_snapshot::_restore_db_instance_from_db_snapshot_input::RestoreDbInstanceFromDbSnapshotInput;

mod _restore_db_instance_from_db_snapshot_input;

mod _restore_db_instance_from_db_snapshot_output;

/// Builders
pub mod builders;
