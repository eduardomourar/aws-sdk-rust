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

pub(crate) mod shape_associate_resource;

pub(crate) mod shape_create_canary;

pub(crate) mod shape_create_group;

pub(crate) mod shape_delete_canary;

pub(crate) mod shape_delete_group;

pub(crate) mod shape_describe_canaries;

pub(crate) mod shape_describe_canaries_last_run;

pub(crate) mod shape_describe_runtime_versions;

pub(crate) mod shape_disassociate_resource;

pub(crate) mod shape_get_canary;

pub(crate) mod shape_get_canary_runs;

pub(crate) mod shape_get_group;

pub(crate) mod shape_list_associated_groups;

pub(crate) mod shape_list_group_resources;

pub(crate) mod shape_list_groups;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_start_canary;

pub(crate) mod shape_start_canary_dry_run;

pub(crate) mod shape_stop_canary;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_canary;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_associate_resource_input;

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_canary_input;

pub(crate) mod shape_create_group_input;

pub(crate) mod shape_describe_canaries_input;

pub(crate) mod shape_describe_canaries_last_run_input;

pub(crate) mod shape_describe_runtime_versions_input;

pub(crate) mod shape_disassociate_resource_input;

pub(crate) mod shape_get_canary_runs_input;

pub(crate) mod shape_internal_failure_exception;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_associated_groups_input;

pub(crate) mod shape_list_group_resources_input;

pub(crate) mod shape_list_groups_input;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_request_entity_too_large_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_canary_dry_run_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_update_canary_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_artifact_config_input;

pub(crate) mod shape_canaries;

pub(crate) mod shape_canaries_last_run;

pub(crate) mod shape_canary;

pub(crate) mod shape_canary_code_input;

pub(crate) mod shape_canary_run_config_input;

pub(crate) mod shape_canary_runs;

pub(crate) mod shape_canary_schedule_input;

pub(crate) mod shape_dry_run_config_output;

pub(crate) mod shape_group;

pub(crate) mod shape_group_summary_list;

pub(crate) mod shape_runtime_version_list;

pub(crate) mod shape_string_list;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_visual_reference_input;

pub(crate) mod shape_vpc_config_input;

pub(crate) mod shape_artifact_config_output;

pub(crate) mod shape_base_screenshot;

pub(crate) mod shape_canary_code_output;

pub(crate) mod shape_canary_last_run;

pub(crate) mod shape_canary_run;

pub(crate) mod shape_canary_run_config_output;

pub(crate) mod shape_canary_schedule_output;

pub(crate) mod shape_canary_status;

pub(crate) mod shape_canary_timeline;

pub(crate) mod shape_dependency;

pub(crate) mod shape_group_summary;

pub(crate) mod shape_retry_config_input;

pub(crate) mod shape_runtime_version;

pub(crate) mod shape_s3_encryption_config;

pub(crate) mod shape_visual_reference_output;

pub(crate) mod shape_vpc_config_output;

pub(crate) mod shape_base_screenshots;

pub(crate) mod shape_canary_dry_run_config_output;

pub(crate) mod shape_canary_run_status;

pub(crate) mod shape_canary_run_timeline;

pub(crate) mod shape_dependencies;

pub(crate) mod shape_retry_config_output;

pub(crate) mod shape_security_group_ids;

pub(crate) mod shape_subnet_ids;

pub(crate) mod shape_base_screenshot_ignore_coordinates;
