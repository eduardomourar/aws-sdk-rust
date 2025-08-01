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

pub(crate) mod shape_add_policy_statement;

pub(crate) mod shape_batch_delete_unique_id;

pub(crate) mod shape_create_id_mapping_workflow;

pub(crate) mod shape_create_id_namespace;

pub(crate) mod shape_create_matching_workflow;

pub(crate) mod shape_create_schema_mapping;

pub(crate) mod shape_delete_id_mapping_workflow;

pub(crate) mod shape_delete_id_namespace;

pub(crate) mod shape_delete_matching_workflow;

pub(crate) mod shape_delete_policy_statement;

pub(crate) mod shape_delete_schema_mapping;

pub(crate) mod shape_generate_match_id;

pub(crate) mod shape_get_id_mapping_job;

pub(crate) mod shape_get_id_mapping_workflow;

pub(crate) mod shape_get_id_namespace;

pub(crate) mod shape_get_match_id;

pub(crate) mod shape_get_matching_job;

pub(crate) mod shape_get_matching_workflow;

pub(crate) mod shape_get_policy;

pub(crate) mod shape_get_provider_service;

pub(crate) mod shape_get_schema_mapping;

pub(crate) mod shape_list_id_mapping_jobs;

pub(crate) mod shape_list_id_mapping_workflows;

pub(crate) mod shape_list_id_namespaces;

pub(crate) mod shape_list_matching_jobs;

pub(crate) mod shape_list_matching_workflows;

pub(crate) mod shape_list_provider_services;

pub(crate) mod shape_list_schema_mappings;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_put_policy;

pub(crate) mod shape_start_id_mapping_job;

pub(crate) mod shape_start_matching_job;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_id_mapping_workflow;

pub(crate) mod shape_update_id_namespace;

pub(crate) mod shape_update_matching_workflow;

pub(crate) mod shape_update_schema_mapping;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_add_policy_statement_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_id_mapping_workflow_input;

pub(crate) mod shape_create_id_namespace_input;

pub(crate) mod shape_create_matching_workflow_input;

pub(crate) mod shape_create_schema_mapping_input;

pub(crate) mod shape_exceeds_limit_exception;

pub(crate) mod shape_generate_match_id_input;

pub(crate) mod shape_get_match_id_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_put_policy_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_start_id_mapping_job_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_id_mapping_workflow_input;

pub(crate) mod shape_update_id_namespace_input;

pub(crate) mod shape_update_matching_workflow_input;

pub(crate) mod shape_update_schema_mapping_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_delete_unique_id_errors_list;

pub(crate) mod shape_deleted_unique_id_list;

pub(crate) mod shape_disconnected_unique_ids_list;

pub(crate) mod shape_error_details;

pub(crate) mod shape_failed_records_list;

pub(crate) mod shape_id_mapping_job_metrics;

pub(crate) mod shape_id_mapping_job_output_source;

pub(crate) mod shape_id_mapping_job_output_source_config;

pub(crate) mod shape_id_mapping_techniques;

pub(crate) mod shape_id_mapping_workflow_input_source;

pub(crate) mod shape_id_mapping_workflow_input_source_config;

pub(crate) mod shape_id_mapping_workflow_list;

pub(crate) mod shape_id_mapping_workflow_output_source;

pub(crate) mod shape_id_mapping_workflow_output_source_config;

pub(crate) mod shape_id_namespace_id_mapping_workflow_properties;

pub(crate) mod shape_id_namespace_id_mapping_workflow_properties_list;

pub(crate) mod shape_id_namespace_input_source;

pub(crate) mod shape_id_namespace_input_source_config;

pub(crate) mod shape_id_namespace_list;

pub(crate) mod shape_incremental_run_config;

pub(crate) mod shape_input_source;

pub(crate) mod shape_input_source_config;

pub(crate) mod shape_job_list;

pub(crate) mod shape_job_metrics;

pub(crate) mod shape_job_output_source_config;

pub(crate) mod shape_match_groups_list;

pub(crate) mod shape_matching_workflow_list;

pub(crate) mod shape_output_source;

pub(crate) mod shape_output_source_config;

pub(crate) mod shape_provider_component_schema;

pub(crate) mod shape_provider_endpoint_configuration;

pub(crate) mod shape_provider_id_name_space_configuration;

pub(crate) mod shape_provider_intermediate_data_access_configuration;

pub(crate) mod shape_provider_service_list;

pub(crate) mod shape_record;

pub(crate) mod shape_resolution_techniques;

pub(crate) mod shape_schema_input_attribute;

pub(crate) mod shape_schema_input_attributes;

pub(crate) mod shape_schema_mapping_list;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_aws_account_id_list;

pub(crate) mod shape_delete_unique_id_error;

pub(crate) mod shape_deleted_unique_id;

pub(crate) mod shape_failed_record;

pub(crate) mod shape_id_mapping_rule_based_properties;

pub(crate) mod shape_id_mapping_workflow_summary;

pub(crate) mod shape_id_namespace_summary;

pub(crate) mod shape_job_output_source;

pub(crate) mod shape_job_summary;

pub(crate) mod shape_match_group;

pub(crate) mod shape_matching_workflow_summary;

pub(crate) mod shape_namespace_provider_properties;

pub(crate) mod shape_namespace_rule_based_properties;

pub(crate) mod shape_output_attribute;

pub(crate) mod shape_provider_marketplace_configuration;

pub(crate) mod shape_provider_properties;

pub(crate) mod shape_provider_schema_attributes;

pub(crate) mod shape_provider_service_summary;

pub(crate) mod shape_required_bucket_actions_list;

pub(crate) mod shape_rule_based_properties;

pub(crate) mod shape_rule_condition_properties;

pub(crate) mod shape_schema_mapping_summary;

pub(crate) mod shape_schemas;

pub(crate) mod shape_id_namespace_id_mapping_workflow_metadata_list;

pub(crate) mod shape_intermediate_source_configuration;

pub(crate) mod shape_matched_records_list;

pub(crate) mod shape_output_attributes;

pub(crate) mod shape_provider_schema_attribute;

pub(crate) mod shape_rule;

pub(crate) mod shape_rule_condition;

pub(crate) mod shape_rule_condition_list;

pub(crate) mod shape_rule_list;

pub(crate) mod shape_schema_list;

pub(crate) mod shape_id_mapping_workflow_rule_definition_type_list;

pub(crate) mod shape_id_namespace_id_mapping_workflow_metadata;

pub(crate) mod shape_matched_record;

pub(crate) mod shape_record_matching_model_list;

pub(crate) mod shape_matching_keys;
