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

pub(crate) mod shape_batch_get_user_access_tasks;

pub(crate) mod shape_connect_app_authorization;

pub(crate) mod shape_create_app_authorization;

pub(crate) mod shape_create_app_bundle;

pub(crate) mod shape_create_ingestion;

pub(crate) mod shape_create_ingestion_destination;

pub(crate) mod shape_delete_app_authorization;

pub(crate) mod shape_delete_app_bundle;

pub(crate) mod shape_delete_ingestion;

pub(crate) mod shape_delete_ingestion_destination;

pub(crate) mod shape_get_app_authorization;

pub(crate) mod shape_get_app_bundle;

pub(crate) mod shape_get_ingestion;

pub(crate) mod shape_get_ingestion_destination;

pub(crate) mod shape_list_app_authorizations;

pub(crate) mod shape_list_app_bundles;

pub(crate) mod shape_list_ingestion_destinations;

pub(crate) mod shape_list_ingestions;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_start_ingestion;

pub(crate) mod shape_start_user_access_tasks;

pub(crate) mod shape_stop_ingestion;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_app_authorization;

pub(crate) mod shape_update_ingestion_destination;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_batch_get_user_access_tasks_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_connect_app_authorization_input;

pub(crate) mod shape_create_app_authorization_input;

pub(crate) mod shape_create_app_bundle_input;

pub(crate) mod shape_create_ingestion_destination_input;

pub(crate) mod shape_create_ingestion_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_user_access_tasks_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_app_authorization_input;

pub(crate) mod shape_update_ingestion_destination_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_app_authorization;

pub(crate) mod shape_app_authorization_summary;

pub(crate) mod shape_app_authorization_summary_list;

pub(crate) mod shape_app_bundle;

pub(crate) mod shape_app_bundle_summary_list;

pub(crate) mod shape_auth_request;

pub(crate) mod shape_credential;

pub(crate) mod shape_destination_configuration;

pub(crate) mod shape_ingestion;

pub(crate) mod shape_ingestion_destination;

pub(crate) mod shape_ingestion_destination_list;

pub(crate) mod shape_ingestion_list;

pub(crate) mod shape_processing_configuration;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_tenant;

pub(crate) mod shape_user_access_results_list;

pub(crate) mod shape_user_access_tasks_list;

pub(crate) mod shape_validation_exception_field_list;

pub(crate) mod shape_api_key_credential;

pub(crate) mod shape_app_bundle_summary;

pub(crate) mod shape_audit_log_destination_configuration;

pub(crate) mod shape_audit_log_processing_configuration;

pub(crate) mod shape_ingestion_destination_summary;

pub(crate) mod shape_ingestion_summary;

pub(crate) mod shape_oauth2_credential;

pub(crate) mod shape_user_access_result_item;

pub(crate) mod shape_user_access_task_item;

pub(crate) mod shape_validation_exception_field;

pub(crate) mod shape_destination;

pub(crate) mod shape_task_error;

pub(crate) mod shape_firehose_stream;

pub(crate) mod shape_s3_bucket;
