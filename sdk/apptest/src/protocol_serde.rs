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

pub(crate) mod shape_create_test_case;

pub(crate) mod shape_create_test_configuration;

pub(crate) mod shape_create_test_suite;

pub(crate) mod shape_delete_test_case;

pub(crate) mod shape_delete_test_configuration;

pub(crate) mod shape_delete_test_run;

pub(crate) mod shape_delete_test_suite;

pub(crate) mod shape_get_test_case;

pub(crate) mod shape_get_test_configuration;

pub(crate) mod shape_get_test_run_step;

pub(crate) mod shape_get_test_suite;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_test_cases;

pub(crate) mod shape_list_test_configurations;

pub(crate) mod shape_list_test_run_steps;

pub(crate) mod shape_list_test_run_test_cases;

pub(crate) mod shape_list_test_runs;

pub(crate) mod shape_list_test_suites;

pub(crate) mod shape_start_test_run;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_test_case;

pub(crate) mod shape_update_test_configuration;

pub(crate) mod shape_update_test_suite;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_test_case_input;

pub(crate) mod shape_create_test_configuration_input;

pub(crate) mod shape_create_test_suite_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_test_run_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_test_case_input;

pub(crate) mod shape_update_test_configuration_input;

pub(crate) mod shape_update_test_suite_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_properties;

pub(crate) mod shape_resource;

pub(crate) mod shape_resource_list;

pub(crate) mod shape_service_settings;

pub(crate) mod shape_step;

pub(crate) mod shape_step_list;

pub(crate) mod shape_step_run_summary;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_test_case_latest_version;

pub(crate) mod shape_test_case_run_summary_list;

pub(crate) mod shape_test_case_summary_list;

pub(crate) mod shape_test_cases;

pub(crate) mod shape_test_configuration_latest_version;

pub(crate) mod shape_test_configuration_list;

pub(crate) mod shape_test_run_step_summary_list;

pub(crate) mod shape_test_run_summary_list;

pub(crate) mod shape_test_suite_latest_version;

pub(crate) mod shape_test_suite_list;

pub(crate) mod shape_validation_exception_field_list;

pub(crate) mod shape_compare_action_summary;

pub(crate) mod shape_mainframe_action_summary;

pub(crate) mod shape_resource_action_summary;

pub(crate) mod shape_resource_type;

pub(crate) mod shape_step_action;

pub(crate) mod shape_test_case_list;

pub(crate) mod shape_test_case_run_summary;

pub(crate) mod shape_test_case_summary;

pub(crate) mod shape_test_configuration_summary;

pub(crate) mod shape_test_run_step_summary;

pub(crate) mod shape_test_run_summary;

pub(crate) mod shape_test_suite_summary;

pub(crate) mod shape_validation_exception_field;

pub(crate) mod shape_batch_summary;

pub(crate) mod shape_cloud_formation;

pub(crate) mod shape_cloud_formation_step_summary;

pub(crate) mod shape_compare_action;

pub(crate) mod shape_file;

pub(crate) mod shape_m2_managed_application;

pub(crate) mod shape_m2_managed_application_step_summary;

pub(crate) mod shape_m2_non_managed_application;

pub(crate) mod shape_m2_non_managed_application_step_summary;

pub(crate) mod shape_mainframe_action;

pub(crate) mod shape_resource_action;

pub(crate) mod shape_tn3270_summary;

pub(crate) mod shape_batch_step_input;

pub(crate) mod shape_batch_step_output;

pub(crate) mod shape_cloud_formation_action;

pub(crate) mod shape_compare_file_type;

pub(crate) mod shape_create_cloud_formation_summary;

pub(crate) mod shape_delete_cloud_formation_summary;

pub(crate) mod shape_input;

pub(crate) mod shape_m2_managed_application_action;

pub(crate) mod shape_m2_managed_application_step_input;

pub(crate) mod shape_m2_managed_application_step_output;

pub(crate) mod shape_m2_non_managed_application_action;

pub(crate) mod shape_m2_non_managed_application_step_input;

pub(crate) mod shape_m2_non_managed_application_step_output;

pub(crate) mod shape_mainframe_action_properties;

pub(crate) mod shape_mainframe_action_type;

pub(crate) mod shape_output;

pub(crate) mod shape_tn3270_step_input;

pub(crate) mod shape_tn3270_step_output;

pub(crate) mod shape_batch;

pub(crate) mod shape_batch_job_parameters;

pub(crate) mod shape_compare_data_sets_summary;

pub(crate) mod shape_compare_database_cdc_summary;

pub(crate) mod shape_create_cloud_formation_step_input;

pub(crate) mod shape_create_cloud_formation_step_output;

pub(crate) mod shape_data_set_list;

pub(crate) mod shape_delete_cloud_formation_step_input;

pub(crate) mod shape_delete_cloud_formation_step_output;

pub(crate) mod shape_export_data_set_names;

pub(crate) mod shape_input_file;

pub(crate) mod shape_m2_managed_action_properties;

pub(crate) mod shape_mainframe_resource_summary;

pub(crate) mod shape_output_file;

pub(crate) mod shape_script_summary;

pub(crate) mod shape_tn3270;

pub(crate) mod shape_compare_data_sets_step_input;

pub(crate) mod shape_compare_data_sets_step_output;

pub(crate) mod shape_compare_database_cdc_step_input;

pub(crate) mod shape_compare_database_cdc_step_output;

pub(crate) mod shape_data_set;

pub(crate) mod shape_file_metadata;

pub(crate) mod shape_m2_managed_application_summary;

pub(crate) mod shape_m2_non_managed_application_summary;

pub(crate) mod shape_script;

pub(crate) mod shape_database_cdc;

pub(crate) mod shape_source_database_metadata;

pub(crate) mod shape_target_database_metadata;
