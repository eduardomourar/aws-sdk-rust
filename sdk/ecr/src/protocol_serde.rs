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

pub(crate) mod shape_batch_check_layer_availability;

pub(crate) mod shape_batch_delete_image;

pub(crate) mod shape_batch_get_image;

pub(crate) mod shape_batch_get_repository_scanning_configuration;

pub(crate) mod shape_complete_layer_upload;

pub(crate) mod shape_create_pull_through_cache_rule;

pub(crate) mod shape_create_repository;

pub(crate) mod shape_create_repository_creation_template;

pub(crate) mod shape_delete_lifecycle_policy;

pub(crate) mod shape_delete_pull_through_cache_rule;

pub(crate) mod shape_delete_registry_policy;

pub(crate) mod shape_delete_repository;

pub(crate) mod shape_delete_repository_creation_template;

pub(crate) mod shape_delete_repository_policy;

pub(crate) mod shape_describe_image_replication_status;

pub(crate) mod shape_describe_image_scan_findings;

pub(crate) mod shape_describe_images;

pub(crate) mod shape_describe_pull_through_cache_rules;

pub(crate) mod shape_describe_registry;

pub(crate) mod shape_describe_repositories;

pub(crate) mod shape_describe_repository_creation_templates;

pub(crate) mod shape_get_account_setting;

pub(crate) mod shape_get_authorization_token;

pub(crate) mod shape_get_download_url_for_layer;

pub(crate) mod shape_get_lifecycle_policy;

pub(crate) mod shape_get_lifecycle_policy_preview;

pub(crate) mod shape_get_registry_policy;

pub(crate) mod shape_get_registry_scanning_configuration;

pub(crate) mod shape_get_repository_policy;

pub(crate) mod shape_initiate_layer_upload;

pub(crate) mod shape_list_images;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_put_account_setting;

pub(crate) mod shape_put_image;

pub(crate) mod shape_put_image_scanning_configuration;

pub(crate) mod shape_put_image_tag_mutability;

pub(crate) mod shape_put_lifecycle_policy;

pub(crate) mod shape_put_registry_policy;

pub(crate) mod shape_put_registry_scanning_configuration;

pub(crate) mod shape_put_replication_configuration;

pub(crate) mod shape_set_repository_policy;

pub(crate) mod shape_start_image_scan;

pub(crate) mod shape_start_lifecycle_policy_preview;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_pull_through_cache_rule;

pub(crate) mod shape_update_repository_creation_template;

pub(crate) mod shape_upload_layer_part;

pub(crate) mod shape_validate_pull_through_cache_rule;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_batch_check_layer_availability_input;

pub(crate) mod shape_batch_delete_image_input;

pub(crate) mod shape_batch_get_image_input;

pub(crate) mod shape_batch_get_repository_scanning_configuration_input;

pub(crate) mod shape_complete_layer_upload_input;

pub(crate) mod shape_create_pull_through_cache_rule_input;

pub(crate) mod shape_create_repository_creation_template_input;

pub(crate) mod shape_create_repository_input;

pub(crate) mod shape_delete_lifecycle_policy_input;

pub(crate) mod shape_delete_pull_through_cache_rule_input;

pub(crate) mod shape_delete_repository_creation_template_input;

pub(crate) mod shape_delete_repository_input;

pub(crate) mod shape_delete_repository_policy_input;

pub(crate) mod shape_describe_image_replication_status_input;

pub(crate) mod shape_describe_image_scan_findings_input;

pub(crate) mod shape_describe_images_input;

pub(crate) mod shape_describe_pull_through_cache_rules_input;

pub(crate) mod shape_describe_repositories_input;

pub(crate) mod shape_describe_repository_creation_templates_input;

pub(crate) mod shape_empty_upload_exception;

pub(crate) mod shape_get_account_setting_input;

pub(crate) mod shape_get_authorization_token_input;

pub(crate) mod shape_get_download_url_for_layer_input;

pub(crate) mod shape_get_lifecycle_policy_input;

pub(crate) mod shape_get_lifecycle_policy_preview_input;

pub(crate) mod shape_get_repository_policy_input;

pub(crate) mod shape_image_already_exists_exception;

pub(crate) mod shape_image_digest_does_not_match_exception;

pub(crate) mod shape_image_not_found_exception;

pub(crate) mod shape_image_tag_already_exists_exception;

pub(crate) mod shape_initiate_layer_upload_input;

pub(crate) mod shape_invalid_layer_exception;

pub(crate) mod shape_invalid_layer_part_exception;

pub(crate) mod shape_invalid_parameter_exception;

pub(crate) mod shape_invalid_tag_parameter_exception;

pub(crate) mod shape_kms_exception;

pub(crate) mod shape_layer_already_exists_exception;

pub(crate) mod shape_layer_inaccessible_exception;

pub(crate) mod shape_layer_part_too_small_exception;

pub(crate) mod shape_layers_not_found_exception;

pub(crate) mod shape_lifecycle_policy_not_found_exception;

pub(crate) mod shape_lifecycle_policy_preview_in_progress_exception;

pub(crate) mod shape_lifecycle_policy_preview_not_found_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_list_images_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_pull_through_cache_rule_already_exists_exception;

pub(crate) mod shape_pull_through_cache_rule_not_found_exception;

pub(crate) mod shape_put_account_setting_input;

pub(crate) mod shape_put_image_input;

pub(crate) mod shape_put_image_scanning_configuration_input;

pub(crate) mod shape_put_image_tag_mutability_input;

pub(crate) mod shape_put_lifecycle_policy_input;

pub(crate) mod shape_put_registry_policy_input;

pub(crate) mod shape_put_registry_scanning_configuration_input;

pub(crate) mod shape_put_replication_configuration_input;

pub(crate) mod shape_referenced_images_not_found_exception;

pub(crate) mod shape_registry_policy_not_found_exception;

pub(crate) mod shape_repository_already_exists_exception;

pub(crate) mod shape_repository_not_empty_exception;

pub(crate) mod shape_repository_not_found_exception;

pub(crate) mod shape_repository_policy_not_found_exception;

pub(crate) mod shape_scan_not_found_exception;

pub(crate) mod shape_secret_not_found_exception;

pub(crate) mod shape_server_exception;

pub(crate) mod shape_set_repository_policy_input;

pub(crate) mod shape_start_image_scan_input;

pub(crate) mod shape_start_lifecycle_policy_preview_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_template_already_exists_exception;

pub(crate) mod shape_template_not_found_exception;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_unable_to_access_secret_exception;

pub(crate) mod shape_unable_to_decrypt_secret_value_exception;

pub(crate) mod shape_unable_to_get_upstream_image_exception;

pub(crate) mod shape_unable_to_get_upstream_layer_exception;

pub(crate) mod shape_unsupported_image_type_exception;

pub(crate) mod shape_unsupported_upstream_registry_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_pull_through_cache_rule_input;

pub(crate) mod shape_update_repository_creation_template_input;

pub(crate) mod shape_upload_layer_part_input;

pub(crate) mod shape_upload_not_found_exception;

pub(crate) mod shape_validate_pull_through_cache_rule_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_authorization_data_list;

pub(crate) mod shape_describe_images_filter;

pub(crate) mod shape_encryption_configuration;

pub(crate) mod shape_encryption_configuration_for_repository_creation_template;

pub(crate) mod shape_image;

pub(crate) mod shape_image_detail_list;

pub(crate) mod shape_image_failure_list;

pub(crate) mod shape_image_identifier;

pub(crate) mod shape_image_identifier_list;

pub(crate) mod shape_image_list;

pub(crate) mod shape_image_replication_status_list;

pub(crate) mod shape_image_scan_findings;

pub(crate) mod shape_image_scan_status;

pub(crate) mod shape_image_scanning_configuration;

pub(crate) mod shape_image_tag_mutability_exclusion_filter;

pub(crate) mod shape_image_tag_mutability_exclusion_filters;

pub(crate) mod shape_layer_failure_list;

pub(crate) mod shape_layer_list;

pub(crate) mod shape_lifecycle_policy_preview_filter;

pub(crate) mod shape_lifecycle_policy_preview_result_list;

pub(crate) mod shape_lifecycle_policy_preview_summary;

pub(crate) mod shape_list_images_filter;

pub(crate) mod shape_pull_through_cache_rule_list;

pub(crate) mod shape_registry_scanning_configuration;

pub(crate) mod shape_registry_scanning_rule;

pub(crate) mod shape_replication_configuration;

pub(crate) mod shape_repository;

pub(crate) mod shape_repository_creation_template;

pub(crate) mod shape_repository_creation_template_list;

pub(crate) mod shape_repository_list;

pub(crate) mod shape_repository_scanning_configuration_failure_list;

pub(crate) mod shape_repository_scanning_configuration_list;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_authorization_data;

pub(crate) mod shape_enhanced_image_scan_finding_list;

pub(crate) mod shape_finding_severity_counts;

pub(crate) mod shape_image_detail;

pub(crate) mod shape_image_failure;

pub(crate) mod shape_image_replication_status;

pub(crate) mod shape_image_scan_finding_list;

pub(crate) mod shape_layer;

pub(crate) mod shape_layer_failure;

pub(crate) mod shape_lifecycle_policy_preview_result;

pub(crate) mod shape_pull_through_cache_rule;

pub(crate) mod shape_rct_applied_for_list;

pub(crate) mod shape_registry_scanning_rule_list;

pub(crate) mod shape_replication_rule;

pub(crate) mod shape_replication_rule_list;

pub(crate) mod shape_repository_scanning_configuration;

pub(crate) mod shape_repository_scanning_configuration_failure;

pub(crate) mod shape_scanning_repository_filter;

pub(crate) mod shape_enhanced_image_scan_finding;

pub(crate) mod shape_image_scan_finding;

pub(crate) mod shape_image_scan_findings_summary;

pub(crate) mod shape_image_tag_list;

pub(crate) mod shape_lifecycle_policy_rule_action;

pub(crate) mod shape_replication_destination;

pub(crate) mod shape_repository_filter;

pub(crate) mod shape_scanning_repository_filter_list;

pub(crate) mod shape_attribute_list;

pub(crate) mod shape_package_vulnerability_details;

pub(crate) mod shape_remediation;

pub(crate) mod shape_replication_destination_list;

pub(crate) mod shape_repository_filter_list;

pub(crate) mod shape_resource_list;

pub(crate) mod shape_score_details;

pub(crate) mod shape_attribute;

pub(crate) mod shape_cvss_score_details;

pub(crate) mod shape_cvss_score_list;

pub(crate) mod shape_recommendation;

pub(crate) mod shape_reference_urls_list;

pub(crate) mod shape_related_vulnerabilities_list;

pub(crate) mod shape_resource;

pub(crate) mod shape_vulnerable_packages_list;

pub(crate) mod shape_cvss_score;

pub(crate) mod shape_cvss_score_adjustment_list;

pub(crate) mod shape_resource_details;

pub(crate) mod shape_tags;

pub(crate) mod shape_vulnerable_package;

pub(crate) mod shape_aws_ecr_container_image_details;

pub(crate) mod shape_cvss_score_adjustment;

pub(crate) mod shape_image_tags_list;
