// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_add_profile_permission;

pub(crate) mod shape_cancel_signing_profile;

pub(crate) mod shape_describe_signing_job;

pub(crate) mod shape_get_revocation_status;

pub(crate) mod shape_get_signing_platform;

pub(crate) mod shape_get_signing_profile;

pub(crate) mod shape_list_profile_permissions;

pub(crate) mod shape_list_signing_jobs;

pub(crate) mod shape_list_signing_platforms;

pub(crate) mod shape_list_signing_profiles;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_put_signing_profile;

pub(crate) mod shape_remove_profile_permission;

pub(crate) mod shape_revoke_signature;

pub(crate) mod shape_revoke_signing_profile;

pub(crate) mod shape_sign_payload;

pub(crate) mod shape_start_signing_job;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_add_profile_permission_input;

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_internal_service_error_exception;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_put_signing_profile_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_revoke_signature_input;

pub(crate) mod shape_revoke_signing_profile_input;

pub(crate) mod shape_service_limit_exceeded_exception;

pub(crate) mod shape_sign_payload_input;

pub(crate) mod shape_start_signing_job_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_destination;

pub(crate) mod shape_metadata;

pub(crate) mod shape_permissions;

pub(crate) mod shape_revoked_entities;

pub(crate) mod shape_signature_validity_period;

pub(crate) mod shape_signed_object;

pub(crate) mod shape_signing_configuration;

pub(crate) mod shape_signing_image_format;

pub(crate) mod shape_signing_job_revocation_record;

pub(crate) mod shape_signing_jobs;

pub(crate) mod shape_signing_material;

pub(crate) mod shape_signing_parameters;

pub(crate) mod shape_signing_platform_overrides;

pub(crate) mod shape_signing_platforms;

pub(crate) mod shape_signing_profile_revocation_record;

pub(crate) mod shape_signing_profiles;

pub(crate) mod shape_source;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_encryption_algorithm_options;

pub(crate) mod shape_hash_algorithm_options;

pub(crate) mod shape_image_formats;

pub(crate) mod shape_permission;

pub(crate) mod shape_s3_destination;

pub(crate) mod shape_s3_signed_object;

pub(crate) mod shape_s3_source;

pub(crate) mod shape_signing_configuration_overrides;

pub(crate) mod shape_signing_job;

pub(crate) mod shape_signing_platform;

pub(crate) mod shape_signing_profile;

pub(crate) mod shape_encryption_algorithms;

pub(crate) mod shape_hash_algorithms;
