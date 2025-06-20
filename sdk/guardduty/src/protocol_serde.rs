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

pub(crate) mod shape_accept_administrator_invitation;

pub(crate) mod shape_accept_invitation;

pub(crate) mod shape_archive_findings;

pub(crate) mod shape_create_detector;

pub(crate) mod shape_create_filter;

pub(crate) mod shape_create_ip_set;

pub(crate) mod shape_create_malware_protection_plan;

pub(crate) mod shape_create_members;

pub(crate) mod shape_create_publishing_destination;

pub(crate) mod shape_create_sample_findings;

pub(crate) mod shape_create_threat_intel_set;

pub(crate) mod shape_decline_invitations;

pub(crate) mod shape_delete_detector;

pub(crate) mod shape_delete_filter;

pub(crate) mod shape_delete_invitations;

pub(crate) mod shape_delete_ip_set;

pub(crate) mod shape_delete_malware_protection_plan;

pub(crate) mod shape_delete_members;

pub(crate) mod shape_delete_publishing_destination;

pub(crate) mod shape_delete_threat_intel_set;

pub(crate) mod shape_describe_malware_scans;

pub(crate) mod shape_describe_organization_configuration;

pub(crate) mod shape_describe_publishing_destination;

pub(crate) mod shape_disable_organization_admin_account;

pub(crate) mod shape_disassociate_from_administrator_account;

pub(crate) mod shape_disassociate_from_master_account;

pub(crate) mod shape_disassociate_members;

pub(crate) mod shape_enable_organization_admin_account;

pub(crate) mod shape_get_administrator_account;

pub(crate) mod shape_get_coverage_statistics;

pub(crate) mod shape_get_detector;

pub(crate) mod shape_get_filter;

pub(crate) mod shape_get_findings;

pub(crate) mod shape_get_findings_statistics;

pub(crate) mod shape_get_invitations_count;

pub(crate) mod shape_get_ip_set;

pub(crate) mod shape_get_malware_protection_plan;

pub(crate) mod shape_get_malware_scan_settings;

pub(crate) mod shape_get_master_account;

pub(crate) mod shape_get_member_detectors;

pub(crate) mod shape_get_members;

pub(crate) mod shape_get_organization_statistics;

pub(crate) mod shape_get_remaining_free_trial_days;

pub(crate) mod shape_get_threat_intel_set;

pub(crate) mod shape_get_usage_statistics;

pub(crate) mod shape_invite_members;

pub(crate) mod shape_list_coverage;

pub(crate) mod shape_list_detectors;

pub(crate) mod shape_list_filters;

pub(crate) mod shape_list_findings;

pub(crate) mod shape_list_invitations;

pub(crate) mod shape_list_ip_sets;

pub(crate) mod shape_list_malware_protection_plans;

pub(crate) mod shape_list_members;

pub(crate) mod shape_list_organization_admin_accounts;

pub(crate) mod shape_list_publishing_destinations;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_threat_intel_sets;

pub(crate) mod shape_start_malware_scan;

pub(crate) mod shape_start_monitoring_members;

pub(crate) mod shape_stop_monitoring_members;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_unarchive_findings;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_detector;

pub(crate) mod shape_update_filter;

pub(crate) mod shape_update_findings_feedback;

pub(crate) mod shape_update_ip_set;

pub(crate) mod shape_update_malware_protection_plan;

pub(crate) mod shape_update_malware_scan_settings;

pub(crate) mod shape_update_member_detectors;

pub(crate) mod shape_update_organization_configuration;

pub(crate) mod shape_update_publishing_destination;

pub(crate) mod shape_update_threat_intel_set;

pub(crate) mod shape_accept_administrator_invitation_input;

pub(crate) mod shape_accept_invitation_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_archive_findings_input;

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_detector_input;

pub(crate) mod shape_create_filter_input;

pub(crate) mod shape_create_ip_set_input;

pub(crate) mod shape_create_malware_protection_plan_input;

pub(crate) mod shape_create_members_input;

pub(crate) mod shape_create_publishing_destination_input;

pub(crate) mod shape_create_sample_findings_input;

pub(crate) mod shape_create_threat_intel_set_input;

pub(crate) mod shape_decline_invitations_input;

pub(crate) mod shape_delete_invitations_input;

pub(crate) mod shape_delete_members_input;

pub(crate) mod shape_describe_malware_scans_input;

pub(crate) mod shape_disable_organization_admin_account_input;

pub(crate) mod shape_disassociate_members_input;

pub(crate) mod shape_enable_organization_admin_account_input;

pub(crate) mod shape_get_coverage_statistics_input;

pub(crate) mod shape_get_findings_input;

pub(crate) mod shape_get_findings_statistics_input;

pub(crate) mod shape_get_member_detectors_input;

pub(crate) mod shape_get_members_input;

pub(crate) mod shape_get_remaining_free_trial_days_input;

pub(crate) mod shape_get_usage_statistics_input;

pub(crate) mod shape_internal_server_error_exception;

pub(crate) mod shape_invite_members_input;

pub(crate) mod shape_list_coverage_input;

pub(crate) mod shape_list_findings_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_start_malware_scan_input;

pub(crate) mod shape_start_monitoring_members_input;

pub(crate) mod shape_stop_monitoring_members_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_unarchive_findings_input;

pub(crate) mod shape_update_detector_input;

pub(crate) mod shape_update_filter_input;

pub(crate) mod shape_update_findings_feedback_input;

pub(crate) mod shape_update_ip_set_input;

pub(crate) mod shape_update_malware_protection_plan_input;

pub(crate) mod shape_update_malware_scan_settings_input;

pub(crate) mod shape_update_member_detectors_input;

pub(crate) mod shape_update_organization_configuration_input;

pub(crate) mod shape_update_publishing_destination_input;

pub(crate) mod shape_update_threat_intel_set_input;

pub(crate) mod shape_account_detail;

pub(crate) mod shape_account_free_trial_infos;

pub(crate) mod shape_admin_accounts;

pub(crate) mod shape_administrator;

pub(crate) mod shape_coverage_filter_criteria;

pub(crate) mod shape_coverage_resources;

pub(crate) mod shape_coverage_sort_criteria;

pub(crate) mod shape_coverage_statistics;

pub(crate) mod shape_create_protected_resource;

pub(crate) mod shape_data_source_configurations;

pub(crate) mod shape_data_source_configurations_result;

pub(crate) mod shape_destination_properties;

pub(crate) mod shape_destinations;

pub(crate) mod shape_detector_feature_configuration;

pub(crate) mod shape_detector_feature_configurations_results;

pub(crate) mod shape_detector_ids;

pub(crate) mod shape_filter_criteria;

pub(crate) mod shape_filter_names;

pub(crate) mod shape_finding_criteria;

pub(crate) mod shape_finding_ids;

pub(crate) mod shape_finding_statistics;

pub(crate) mod shape_findings;

pub(crate) mod shape_invitations;

pub(crate) mod shape_ip_set_ids;

pub(crate) mod shape_malware_protection_plan_actions;

pub(crate) mod shape_malware_protection_plan_status_reasons_list;

pub(crate) mod shape_malware_protection_plans_summary;

pub(crate) mod shape_master;

pub(crate) mod shape_member_data_source_configurations;

pub(crate) mod shape_member_features_configuration;

pub(crate) mod shape_members;

pub(crate) mod shape_organization_data_source_configurations;

pub(crate) mod shape_organization_data_source_configurations_result;

pub(crate) mod shape_organization_details;

pub(crate) mod shape_organization_feature_configuration;

pub(crate) mod shape_organization_features_configurations_results;

pub(crate) mod shape_scan_resource_criteria;

pub(crate) mod shape_scans;

pub(crate) mod shape_sort_criteria;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_threat_intel_set_ids;

pub(crate) mod shape_unprocessed_accounts;

pub(crate) mod shape_unprocessed_data_sources_result;

pub(crate) mod shape_update_protected_resource;

pub(crate) mod shape_usage_criteria;

pub(crate) mod shape_usage_statistics;

pub(crate) mod shape_account_free_trial_info;

pub(crate) mod shape_admin_account;

pub(crate) mod shape_cloud_trail_configuration_result;

pub(crate) mod shape_condition;

pub(crate) mod shape_count_by_coverage_status;

pub(crate) mod shape_count_by_resource_type;

pub(crate) mod shape_count_by_severity;

pub(crate) mod shape_coverage_filter_criterion;

pub(crate) mod shape_coverage_resource;

pub(crate) mod shape_create_s3_bucket_resource;

pub(crate) mod shape_criterion;

pub(crate) mod shape_destination;

pub(crate) mod shape_detector_additional_configuration;

pub(crate) mod shape_detector_feature_configuration_result;

pub(crate) mod shape_dns_logs_configuration_result;

pub(crate) mod shape_filter_criterion;

pub(crate) mod shape_finding;

pub(crate) mod shape_flow_logs_configuration_result;

pub(crate) mod shape_grouped_by_account;

pub(crate) mod shape_grouped_by_date;

pub(crate) mod shape_grouped_by_finding_type;

pub(crate) mod shape_grouped_by_resource;

pub(crate) mod shape_grouped_by_severity;

pub(crate) mod shape_invitation;

pub(crate) mod shape_kubernetes_configuration;

pub(crate) mod shape_kubernetes_configuration_result;

pub(crate) mod shape_malware_protection_configuration;

pub(crate) mod shape_malware_protection_configuration_result;

pub(crate) mod shape_malware_protection_plan_status_reason;

pub(crate) mod shape_malware_protection_plan_summary;

pub(crate) mod shape_malware_protection_plan_tagging_action;

pub(crate) mod shape_member;

pub(crate) mod shape_member_additional_configuration;

pub(crate) mod shape_member_data_source_configuration;

pub(crate) mod shape_organization_additional_configuration;

pub(crate) mod shape_organization_feature_configuration_result;

pub(crate) mod shape_organization_kubernetes_configuration;

pub(crate) mod shape_organization_kubernetes_configuration_result;

pub(crate) mod shape_organization_malware_protection_configuration;

pub(crate) mod shape_organization_malware_protection_configuration_result;

pub(crate) mod shape_organization_s3_logs_configuration;

pub(crate) mod shape_organization_s3_logs_configuration_result;

pub(crate) mod shape_organization_statistics;

pub(crate) mod shape_s3_logs_configuration;

pub(crate) mod shape_s3_logs_configuration_result;

pub(crate) mod shape_scan;

pub(crate) mod shape_scan_condition;

pub(crate) mod shape_scan_criterion;

pub(crate) mod shape_unprocessed_account;

pub(crate) mod shape_update_s3_bucket_resource;

pub(crate) mod shape_usage_account_result_list;

pub(crate) mod shape_usage_data_source_result_list;

pub(crate) mod shape_usage_feature_result_list;

pub(crate) mod shape_usage_resource_result_list;

pub(crate) mod shape_usage_top_accounts_result_list;

pub(crate) mod shape_account_statistics;

pub(crate) mod shape_coverage_filter_condition;

pub(crate) mod shape_coverage_resource_details;

pub(crate) mod shape_data_sources_free_trial;

pub(crate) mod shape_date_statistics;

pub(crate) mod shape_detector_additional_configuration_results;

pub(crate) mod shape_filter_condition;

pub(crate) mod shape_finding_type_statistics;

pub(crate) mod shape_free_trial_feature_configurations_results;

pub(crate) mod shape_kubernetes_audit_logs_configuration;

pub(crate) mod shape_kubernetes_audit_logs_configuration_result;

pub(crate) mod shape_malware_protection_plan_object_prefixes_list;

pub(crate) mod shape_member_features_configurations_results;

pub(crate) mod shape_organization_additional_configuration_results;

pub(crate) mod shape_organization_feature_statistics_results;

pub(crate) mod shape_organization_kubernetes_audit_logs_configuration;

pub(crate) mod shape_organization_kubernetes_audit_logs_configuration_result;

pub(crate) mod shape_organization_scan_ec2_instance_with_findings;

pub(crate) mod shape_organization_scan_ec2_instance_with_findings_result;

pub(crate) mod shape_resource;

pub(crate) mod shape_resource_details;

pub(crate) mod shape_resource_statistics;

pub(crate) mod shape_scan_condition_pair;

pub(crate) mod shape_scan_ec2_instance_with_findings;

pub(crate) mod shape_scan_ec2_instance_with_findings_result;

pub(crate) mod shape_scan_result_details;

pub(crate) mod shape_service;

pub(crate) mod shape_severity_statistics;

pub(crate) mod shape_trigger_details;

pub(crate) mod shape_usage_account_result;

pub(crate) mod shape_usage_data_source_result;

pub(crate) mod shape_usage_feature_result;

pub(crate) mod shape_usage_resource_result;

pub(crate) mod shape_usage_top_accounts_result;

pub(crate) mod shape_volume_details;

pub(crate) mod shape_access_key_details;

pub(crate) mod shape_action;

pub(crate) mod shape_container;

pub(crate) mod shape_coverage_ec2_instance_details;

pub(crate) mod shape_coverage_ecs_cluster_details;

pub(crate) mod shape_coverage_eks_cluster_details;

pub(crate) mod shape_data_source_free_trial;

pub(crate) mod shape_detection;

pub(crate) mod shape_detector_additional_configuration_result;

pub(crate) mod shape_ebs_volume_details;

pub(crate) mod shape_ebs_volume_scan_details;

pub(crate) mod shape_ebs_volumes_result;

pub(crate) mod shape_ecs_cluster_details;

pub(crate) mod shape_eks_cluster_details;

pub(crate) mod shape_eq;

pub(crate) mod shape_equals;

pub(crate) mod shape_evidence;

pub(crate) mod shape_free_trial_feature_configuration_result;

pub(crate) mod shape_instance_details;

pub(crate) mod shape_kubernetes_data_source_free_trial;

pub(crate) mod shape_kubernetes_details;

pub(crate) mod shape_lambda_details;

pub(crate) mod shape_malware_protection_data_source_free_trial;

pub(crate) mod shape_malware_scan_details;

pub(crate) mod shape_map_equals;

pub(crate) mod shape_member_features_configuration_result;

pub(crate) mod shape_neq;

pub(crate) mod shape_not_equals;

pub(crate) mod shape_organization_additional_configuration_result;

pub(crate) mod shape_organization_ebs_volumes;

pub(crate) mod shape_organization_ebs_volumes_result;

pub(crate) mod shape_organization_feature_statistics;

pub(crate) mod shape_rds_db_instance_details;

pub(crate) mod shape_rds_db_user_details;

pub(crate) mod shape_rds_limitless_db_details;

pub(crate) mod shape_runtime_details;

pub(crate) mod shape_s3_bucket_details;

pub(crate) mod shape_service_additional_info;

pub(crate) mod shape_total;

pub(crate) mod shape_usage_top_accounts_by_feature_list;

pub(crate) mod shape_volume_detail;

pub(crate) mod shape_addon_details;

pub(crate) mod shape_agent_details;

pub(crate) mod shape_anomaly;

pub(crate) mod shape_aws_api_call_action;

pub(crate) mod shape_container_instance_details;

pub(crate) mod shape_dns_request_action;

pub(crate) mod shape_ecs_task_details;

pub(crate) mod shape_fargate_details;

pub(crate) mod shape_iam_instance_profile;

pub(crate) mod shape_kubernetes_api_call_action;

pub(crate) mod shape_kubernetes_permission_checked_details;

pub(crate) mod shape_kubernetes_role_binding_details;

pub(crate) mod shape_kubernetes_role_details;

pub(crate) mod shape_kubernetes_user_details;

pub(crate) mod shape_kubernetes_workload_details;

pub(crate) mod shape_member_additional_configuration_results;

pub(crate) mod shape_network_connection_action;

pub(crate) mod shape_network_interfaces;

pub(crate) mod shape_organization_feature_statistics_additional_configurations;

pub(crate) mod shape_port_probe_action;

pub(crate) mod shape_process_details;

pub(crate) mod shape_product_codes;

pub(crate) mod shape_rds_login_attempt_action;

pub(crate) mod shape_runtime_context;

pub(crate) mod shape_s3_bucket_detail;

pub(crate) mod shape_scan_detections;

pub(crate) mod shape_security_context;

pub(crate) mod shape_sequence;

pub(crate) mod shape_sources;

pub(crate) mod shape_tags;

pub(crate) mod shape_threat_intelligence_details;

pub(crate) mod shape_threats;

pub(crate) mod shape_usage_top_account_result;

pub(crate) mod shape_volume_mounts;

pub(crate) mod shape_vpc_config;

pub(crate) mod shape_actors;

pub(crate) mod shape_additional_sequence_types;

pub(crate) mod shape_affected_resources;

pub(crate) mod shape_anomaly_profiles;

pub(crate) mod shape_anomaly_unusual;

pub(crate) mod shape_containers;

pub(crate) mod shape_default_server_side_encryption;

pub(crate) mod shape_domain_details;

pub(crate) mod shape_flags_list;

pub(crate) mod shape_groups;

pub(crate) mod shape_highest_severity_threat_details;

pub(crate) mod shape_impersonated_user;

pub(crate) mod shape_indicators;

pub(crate) mod shape_issues;

pub(crate) mod shape_lineage;

pub(crate) mod shape_local_ip_details;

pub(crate) mod shape_local_port_details;

pub(crate) mod shape_login_attributes;

pub(crate) mod shape_member_additional_configuration_result;

pub(crate) mod shape_memory_regions_list;

pub(crate) mod shape_network_endpoints;

pub(crate) mod shape_network_interface;

pub(crate) mod shape_organization_feature_statistics_additional_configuration;

pub(crate) mod shape_owner;

pub(crate) mod shape_port_probe_details;

pub(crate) mod shape_product_code;

pub(crate) mod shape_public_access;

pub(crate) mod shape_remote_account_details;

pub(crate) mod shape_remote_ip_details;

pub(crate) mod shape_remote_port_details;

pub(crate) mod shape_resources;

pub(crate) mod shape_s3_object_details;

pub(crate) mod shape_scanned_item_count;

pub(crate) mod shape_security_groups;

pub(crate) mod shape_session_name_list;

pub(crate) mod shape_signals;

pub(crate) mod shape_source_ips;

pub(crate) mod shape_subnet_ids;

pub(crate) mod shape_tag;

pub(crate) mod shape_threat;

pub(crate) mod shape_threat_detected_by_name;

pub(crate) mod shape_threat_intelligence_detail;

pub(crate) mod shape_threats_detected_item_count;

pub(crate) mod shape_volume_mount;

pub(crate) mod shape_volumes;

pub(crate) mod shape_actor;

pub(crate) mod shape_anomaly_profile_features;

pub(crate) mod shape_behavior;

pub(crate) mod shape_city;

pub(crate) mod shape_country;

pub(crate) mod shape_geo_location;

pub(crate) mod shape_indicator;

pub(crate) mod shape_ipv6_addresses;

pub(crate) mod shape_item_paths;

pub(crate) mod shape_lineage_object;

pub(crate) mod shape_login_attribute;

pub(crate) mod shape_network_endpoint;

pub(crate) mod shape_organization;

pub(crate) mod shape_permission_configuration;

pub(crate) mod shape_port_probe_detail;

pub(crate) mod shape_private_ip_addresses;

pub(crate) mod shape_resource_v2;

pub(crate) mod shape_s3_object_detail;

pub(crate) mod shape_scan_threat_names;

pub(crate) mod shape_security_group;

pub(crate) mod shape_signal;

pub(crate) mod shape_threat_names;

pub(crate) mod shape_volume;

pub(crate) mod shape_account_level_permissions;

pub(crate) mod shape_actor_ids;

pub(crate) mod shape_actor_process;

pub(crate) mod shape_anomaly_profile_feature_objects;

pub(crate) mod shape_anomaly_unusual_behavior_feature;

pub(crate) mod shape_autonomous_system;

pub(crate) mod shape_bucket_level_permissions;

pub(crate) mod shape_endpoint_ids;

pub(crate) mod shape_host_path;

pub(crate) mod shape_indicator_values;

pub(crate) mod shape_item_path;

pub(crate) mod shape_network_connection;

pub(crate) mod shape_network_geo_location;

pub(crate) mod shape_private_ip_address_details;

pub(crate) mod shape_resource_data;

pub(crate) mod shape_resource_uids;

pub(crate) mod shape_scan_threat_name;

pub(crate) mod shape_session;

pub(crate) mod shape_user;

pub(crate) mod shape_access_control_list;

pub(crate) mod shape_access_key;

pub(crate) mod shape_account;

pub(crate) mod shape_anomaly_object;

pub(crate) mod shape_block_public_access;

pub(crate) mod shape_bucket_policy;

pub(crate) mod shape_container_finding_resource;

pub(crate) mod shape_ec2_instance;

pub(crate) mod shape_ec2_network_interface;

pub(crate) mod shape_eks_cluster;

pub(crate) mod shape_file_paths;

pub(crate) mod shape_kubernetes_workload;

pub(crate) mod shape_s3_bucket;

pub(crate) mod shape_s3_object;

pub(crate) mod shape_container_uids;

pub(crate) mod shape_ec2_instance_uids;

pub(crate) mod shape_ec2_network_interface_uids;

pub(crate) mod shape_observations;

pub(crate) mod shape_public_access_configuration;

pub(crate) mod shape_s3_object_uids;

pub(crate) mod shape_scan_file_path;

pub(crate) mod shape_observation_texts;
