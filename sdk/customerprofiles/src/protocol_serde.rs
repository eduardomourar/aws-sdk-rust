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

pub(crate) mod shape_add_profile_key;

pub(crate) mod shape_batch_get_calculated_attribute_for_profile;

pub(crate) mod shape_batch_get_profile;

pub(crate) mod shape_create_calculated_attribute_definition;

pub(crate) mod shape_create_domain;

pub(crate) mod shape_create_domain_layout;

pub(crate) mod shape_create_event_stream;

pub(crate) mod shape_create_event_trigger;

pub(crate) mod shape_create_integration_workflow;

pub(crate) mod shape_create_profile;

pub(crate) mod shape_create_segment_definition;

pub(crate) mod shape_create_segment_estimate;

pub(crate) mod shape_create_segment_snapshot;

pub(crate) mod shape_create_upload_job;

pub(crate) mod shape_delete_calculated_attribute_definition;

pub(crate) mod shape_delete_domain;

pub(crate) mod shape_delete_domain_layout;

pub(crate) mod shape_delete_event_stream;

pub(crate) mod shape_delete_event_trigger;

pub(crate) mod shape_delete_integration;

pub(crate) mod shape_delete_profile;

pub(crate) mod shape_delete_profile_key;

pub(crate) mod shape_delete_profile_object;

pub(crate) mod shape_delete_profile_object_type;

pub(crate) mod shape_delete_segment_definition;

pub(crate) mod shape_delete_workflow;

pub(crate) mod shape_detect_profile_object_type;

pub(crate) mod shape_get_auto_merging_preview;

pub(crate) mod shape_get_calculated_attribute_definition;

pub(crate) mod shape_get_calculated_attribute_for_profile;

pub(crate) mod shape_get_domain;

pub(crate) mod shape_get_domain_layout;

pub(crate) mod shape_get_event_stream;

pub(crate) mod shape_get_event_trigger;

pub(crate) mod shape_get_identity_resolution_job;

pub(crate) mod shape_get_integration;

pub(crate) mod shape_get_matches;

pub(crate) mod shape_get_profile_object_type;

pub(crate) mod shape_get_profile_object_type_template;

pub(crate) mod shape_get_segment_definition;

pub(crate) mod shape_get_segment_estimate;

pub(crate) mod shape_get_segment_membership;

pub(crate) mod shape_get_segment_snapshot;

pub(crate) mod shape_get_similar_profiles;

pub(crate) mod shape_get_upload_job;

pub(crate) mod shape_get_upload_job_path;

pub(crate) mod shape_get_workflow;

pub(crate) mod shape_get_workflow_steps;

pub(crate) mod shape_list_account_integrations;

pub(crate) mod shape_list_calculated_attribute_definitions;

pub(crate) mod shape_list_calculated_attributes_for_profile;

pub(crate) mod shape_list_domain_layouts;

pub(crate) mod shape_list_domains;

pub(crate) mod shape_list_event_streams;

pub(crate) mod shape_list_event_triggers;

pub(crate) mod shape_list_identity_resolution_jobs;

pub(crate) mod shape_list_integrations;

pub(crate) mod shape_list_object_type_attributes;

pub(crate) mod shape_list_profile_attribute_values;

pub(crate) mod shape_list_profile_object_type_templates;

pub(crate) mod shape_list_profile_object_types;

pub(crate) mod shape_list_profile_objects;

pub(crate) mod shape_list_rule_based_matches;

pub(crate) mod shape_list_segment_definitions;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_upload_jobs;

pub(crate) mod shape_list_workflows;

pub(crate) mod shape_merge_profiles;

pub(crate) mod shape_put_integration;

pub(crate) mod shape_put_profile_object;

pub(crate) mod shape_put_profile_object_type;

pub(crate) mod shape_search_profiles;

pub(crate) mod shape_start_upload_job;

pub(crate) mod shape_stop_upload_job;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_calculated_attribute_definition;

pub(crate) mod shape_update_domain;

pub(crate) mod shape_update_domain_layout;

pub(crate) mod shape_update_event_trigger;

pub(crate) mod shape_update_profile;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_add_profile_key_input;

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_batch_get_calculated_attribute_for_profile_input;

pub(crate) mod shape_batch_get_profile_input;

pub(crate) mod shape_create_calculated_attribute_definition_input;

pub(crate) mod shape_create_domain_input;

pub(crate) mod shape_create_domain_layout_input;

pub(crate) mod shape_create_event_stream_input;

pub(crate) mod shape_create_event_trigger_input;

pub(crate) mod shape_create_integration_workflow_input;

pub(crate) mod shape_create_profile_input;

pub(crate) mod shape_create_segment_definition_input;

pub(crate) mod shape_create_segment_estimate_input;

pub(crate) mod shape_create_segment_snapshot_input;

pub(crate) mod shape_create_upload_job_input;

pub(crate) mod shape_delete_integration_input;

pub(crate) mod shape_delete_profile_input;

pub(crate) mod shape_delete_profile_key_input;

pub(crate) mod shape_delete_profile_object_input;

pub(crate) mod shape_detect_profile_object_type_input;

pub(crate) mod shape_get_auto_merging_preview_input;

pub(crate) mod shape_get_integration_input;

pub(crate) mod shape_get_segment_membership_input;

pub(crate) mod shape_get_similar_profiles_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_account_integrations_input;

pub(crate) mod shape_list_profile_objects_input;

pub(crate) mod shape_list_workflows_input;

pub(crate) mod shape_merge_profiles_input;

pub(crate) mod shape_put_integration_input;

pub(crate) mod shape_put_profile_object_input;

pub(crate) mod shape_put_profile_object_type_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_search_profiles_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_calculated_attribute_definition_input;

pub(crate) mod shape_update_domain_input;

pub(crate) mod shape_update_domain_layout_input;

pub(crate) mod shape_update_event_trigger_input;

pub(crate) mod shape_update_profile_input;

pub(crate) mod shape_additional_search_key;

pub(crate) mod shape_address;

pub(crate) mod shape_attribute_details;

pub(crate) mod shape_attribute_value_item_list;

pub(crate) mod shape_auto_merging;

pub(crate) mod shape_batch_get_calculated_attribute_for_profile_error_list;

pub(crate) mod shape_batch_get_profile_error_list;

pub(crate) mod shape_calculated_attribute_definitions_list;

pub(crate) mod shape_calculated_attribute_value_list;

pub(crate) mod shape_calculated_attributes_for_profile_list;

pub(crate) mod shape_condition_overrides;

pub(crate) mod shape_conditions;

pub(crate) mod shape_conflict_resolution;

pub(crate) mod shape_consolidation;

pub(crate) mod shape_detected_profile_object_types;

pub(crate) mod shape_domain_list;

pub(crate) mod shape_domain_stats;

pub(crate) mod shape_engagement_preferences;

pub(crate) mod shape_event_stream_destination_details;

pub(crate) mod shape_event_stream_summary_list;

pub(crate) mod shape_event_trigger_condition;

pub(crate) mod shape_event_trigger_conditions;

pub(crate) mod shape_event_trigger_limits;

pub(crate) mod shape_event_trigger_names;

pub(crate) mod shape_event_trigger_summary_list;

pub(crate) mod shape_exporting_location;

pub(crate) mod shape_failures;

pub(crate) mod shape_field_map;

pub(crate) mod shape_field_source_profile_ids;

pub(crate) mod shape_filter;

pub(crate) mod shape_flow_definition;

pub(crate) mod shape_identity_resolution_jobs_list;

pub(crate) mod shape_integration_config;

pub(crate) mod shape_integration_list;

pub(crate) mod shape_job_stats;

pub(crate) mod shape_key_map;

pub(crate) mod shape_layout_list;

pub(crate) mod shape_list_object_type_attributes_list;

pub(crate) mod shape_match_id_list;

pub(crate) mod shape_matches_list;

pub(crate) mod shape_matching_request;

pub(crate) mod shape_matching_response;

pub(crate) mod shape_object_filter;

pub(crate) mod shape_object_type_field;

pub(crate) mod shape_object_type_key;

pub(crate) mod shape_object_type_names;

pub(crate) mod shape_profile_id_list;

pub(crate) mod shape_profile_list;

pub(crate) mod shape_profile_object_list;

pub(crate) mod shape_profile_object_type_list;

pub(crate) mod shape_profile_object_type_template_list;

pub(crate) mod shape_profiles;

pub(crate) mod shape_readiness;

pub(crate) mod shape_request_value_list;

pub(crate) mod shape_results_summary;

pub(crate) mod shape_rule_based_matching_request;

pub(crate) mod shape_rule_based_matching_response;

pub(crate) mod shape_segment_definitions_list;

pub(crate) mod shape_segment_group;

pub(crate) mod shape_segment_group_structure;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_update_address;

pub(crate) mod shape_upload_jobs_list;

pub(crate) mod shape_workflow_attributes;

pub(crate) mod shape_workflow_list;

pub(crate) mod shape_workflow_metrics;

pub(crate) mod shape_workflow_steps_list;

pub(crate) mod shape_appflow_integration;

pub(crate) mod shape_appflow_integration_workflow_attributes;

pub(crate) mod shape_appflow_integration_workflow_metrics;

pub(crate) mod shape_attribute_item;

pub(crate) mod shape_attribute_list;

pub(crate) mod shape_attribute_types_selector;

pub(crate) mod shape_attribute_value_item;

pub(crate) mod shape_batch_get_calculated_attribute_for_profile_error;

pub(crate) mod shape_batch_get_profile_error;

pub(crate) mod shape_calculated_attribute_value;

pub(crate) mod shape_contact_preference;

pub(crate) mod shape_detected_profile_object_type;

pub(crate) mod shape_event_stream_summary;

pub(crate) mod shape_event_trigger_dimension;

pub(crate) mod shape_event_trigger_summary_item;

pub(crate) mod shape_exporting_config;

pub(crate) mod shape_filter_group;

pub(crate) mod shape_group;

pub(crate) mod shape_group_list;

pub(crate) mod shape_identity_resolution_job;

pub(crate) mod shape_job_schedule;

pub(crate) mod shape_layout_item;

pub(crate) mod shape_list_calculated_attribute_definition_item;

pub(crate) mod shape_list_calculated_attribute_for_profile_item;

pub(crate) mod shape_list_domain_item;

pub(crate) mod shape_list_integration_item;

pub(crate) mod shape_list_object_type_attribute_item;

pub(crate) mod shape_list_profile_object_type_item;

pub(crate) mod shape_list_profile_object_type_template_item;

pub(crate) mod shape_list_profile_objects_item;

pub(crate) mod shape_list_workflows_item;

pub(crate) mod shape_match_item;

pub(crate) mod shape_matching_rule;

pub(crate) mod shape_matching_rules;

pub(crate) mod shape_object_type_key_list;

pub(crate) mod shape_period;

pub(crate) mod shape_periods;

pub(crate) mod shape_profile;

pub(crate) mod shape_profile_query_failures;

pub(crate) mod shape_profile_query_result;

pub(crate) mod shape_range;

pub(crate) mod shape_range_override;

pub(crate) mod shape_s3_exporting_location;

pub(crate) mod shape_segment_definition_item;

pub(crate) mod shape_segment_group_list;

pub(crate) mod shape_source_flow_config;

pub(crate) mod shape_task;

pub(crate) mod shape_threshold;

pub(crate) mod shape_trigger_config;

pub(crate) mod shape_upload_job_item;

pub(crate) mod shape_workflow_step_item;

pub(crate) mod shape_address_list;

pub(crate) mod shape_appflow_integration_workflow_step;

pub(crate) mod shape_attributes;

pub(crate) mod shape_batch;

pub(crate) mod shape_connector_operator;

pub(crate) mod shape_destination_summary;

pub(crate) mod shape_dimension;

pub(crate) mod shape_email_list;

pub(crate) mod shape_event_trigger_dimensions;

pub(crate) mod shape_filter_dimension;

pub(crate) mod shape_found_by_list;

pub(crate) mod shape_incremental_pull_config;

pub(crate) mod shape_matching_attributes_list;

pub(crate) mod shape_object_attribute;

pub(crate) mod shape_phone_number_list;

pub(crate) mod shape_s3_exporting_config;

pub(crate) mod shape_source_connector_properties;

pub(crate) mod shape_source_segment;

pub(crate) mod shape_trigger_properties;

pub(crate) mod shape_value_range;

pub(crate) mod shape_calculated_attribute_dimension;

pub(crate) mod shape_dimension_list;

pub(crate) mod shape_email_preference_list;

pub(crate) mod shape_field_name_list;

pub(crate) mod shape_filter_attribute_dimension;

pub(crate) mod shape_filter_dimension_list;

pub(crate) mod shape_found_by_key_value;

pub(crate) mod shape_marketo_source_properties;

pub(crate) mod shape_matching_attributes;

pub(crate) mod shape_matching_rule_attribute_list;

pub(crate) mod shape_phone_preference_list;

pub(crate) mod shape_profile_attributes;

pub(crate) mod shape_s3_source_properties;

pub(crate) mod shape_salesforce_source_properties;

pub(crate) mod shape_scheduled_trigger_properties;

pub(crate) mod shape_service_now_source_properties;

pub(crate) mod shape_source_segment_list;

pub(crate) mod shape_standard_identifier_list;

pub(crate) mod shape_zendesk_source_properties;

pub(crate) mod shape_address_dimension;

pub(crate) mod shape_attribute_dimension;

pub(crate) mod shape_date_dimension;

pub(crate) mod shape_extra_length_value_profile_dimension;

pub(crate) mod shape_object_attributes;

pub(crate) mod shape_profile_dimension;

pub(crate) mod shape_profile_type_dimension;

pub(crate) mod shape_attribute_map;

pub(crate) mod shape_calculated_custom_attributes;

pub(crate) mod shape_custom_attributes;

pub(crate) mod shape_event_trigger_values;

pub(crate) mod shape_date_values;

pub(crate) mod shape_extra_length_values;

pub(crate) mod shape_profile_type_values;

pub(crate) mod shape_value_list;

pub(crate) mod shape_values;
