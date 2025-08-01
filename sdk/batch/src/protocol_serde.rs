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

pub(crate) mod shape_cancel_job;

pub(crate) mod shape_create_compute_environment;

pub(crate) mod shape_create_consumable_resource;

pub(crate) mod shape_create_job_queue;

pub(crate) mod shape_create_scheduling_policy;

pub(crate) mod shape_create_service_environment;

pub(crate) mod shape_delete_compute_environment;

pub(crate) mod shape_delete_consumable_resource;

pub(crate) mod shape_delete_job_queue;

pub(crate) mod shape_delete_scheduling_policy;

pub(crate) mod shape_delete_service_environment;

pub(crate) mod shape_deregister_job_definition;

pub(crate) mod shape_describe_compute_environments;

pub(crate) mod shape_describe_consumable_resource;

pub(crate) mod shape_describe_job_definitions;

pub(crate) mod shape_describe_job_queues;

pub(crate) mod shape_describe_jobs;

pub(crate) mod shape_describe_scheduling_policies;

pub(crate) mod shape_describe_service_environments;

pub(crate) mod shape_describe_service_job;

pub(crate) mod shape_get_job_queue_snapshot;

pub(crate) mod shape_list_consumable_resources;

pub(crate) mod shape_list_jobs;

pub(crate) mod shape_list_jobs_by_consumable_resource;

pub(crate) mod shape_list_scheduling_policies;

pub(crate) mod shape_list_service_jobs;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_register_job_definition;

pub(crate) mod shape_submit_job;

pub(crate) mod shape_submit_service_job;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_terminate_job;

pub(crate) mod shape_terminate_service_job;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_compute_environment;

pub(crate) mod shape_update_consumable_resource;

pub(crate) mod shape_update_job_queue;

pub(crate) mod shape_update_scheduling_policy;

pub(crate) mod shape_update_service_environment;

pub(crate) mod shape_cancel_job_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_client_exception;

pub(crate) mod shape_create_compute_environment_input;

pub(crate) mod shape_create_consumable_resource_input;

pub(crate) mod shape_create_job_queue_input;

pub(crate) mod shape_create_scheduling_policy_input;

pub(crate) mod shape_create_service_environment_input;

pub(crate) mod shape_delete_compute_environment_input;

pub(crate) mod shape_delete_consumable_resource_input;

pub(crate) mod shape_delete_job_queue_input;

pub(crate) mod shape_delete_scheduling_policy_input;

pub(crate) mod shape_delete_service_environment_input;

pub(crate) mod shape_deregister_job_definition_input;

pub(crate) mod shape_describe_compute_environments_input;

pub(crate) mod shape_describe_consumable_resource_input;

pub(crate) mod shape_describe_job_definitions_input;

pub(crate) mod shape_describe_job_queues_input;

pub(crate) mod shape_describe_jobs_input;

pub(crate) mod shape_describe_scheduling_policies_input;

pub(crate) mod shape_describe_service_environments_input;

pub(crate) mod shape_describe_service_job_input;

pub(crate) mod shape_get_job_queue_snapshot_input;

pub(crate) mod shape_list_consumable_resources_input;

pub(crate) mod shape_list_jobs_by_consumable_resource_input;

pub(crate) mod shape_list_jobs_input;

pub(crate) mod shape_list_scheduling_policies_input;

pub(crate) mod shape_list_service_jobs_input;

pub(crate) mod shape_register_job_definition_input;

pub(crate) mod shape_server_exception;

pub(crate) mod shape_submit_job_input;

pub(crate) mod shape_submit_service_job_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_terminate_job_input;

pub(crate) mod shape_terminate_service_job_input;

pub(crate) mod shape_update_compute_environment_input;

pub(crate) mod shape_update_consumable_resource_input;

pub(crate) mod shape_update_job_queue_input;

pub(crate) mod shape_update_scheduling_policy_input;

pub(crate) mod shape_update_service_environment_input;

pub(crate) mod shape_array_properties;

pub(crate) mod shape_capacity_limit;

pub(crate) mod shape_compute_environment_detail_list;

pub(crate) mod shape_compute_environment_order;

pub(crate) mod shape_compute_resource;

pub(crate) mod shape_compute_resource_update;

pub(crate) mod shape_consumable_resource_properties;

pub(crate) mod shape_consumable_resource_summary_list;

pub(crate) mod shape_container_overrides;

pub(crate) mod shape_container_properties;

pub(crate) mod shape_ecs_properties;

pub(crate) mod shape_ecs_properties_override;

pub(crate) mod shape_eks_configuration;

pub(crate) mod shape_eks_properties;

pub(crate) mod shape_eks_properties_override;

pub(crate) mod shape_fairshare_policy;

pub(crate) mod shape_front_of_queue_detail;

pub(crate) mod shape_job_definition_list;

pub(crate) mod shape_job_dependency;

pub(crate) mod shape_job_detail_list;

pub(crate) mod shape_job_queue_detail_list;

pub(crate) mod shape_job_state_time_limit_action;

pub(crate) mod shape_job_summary_list;

pub(crate) mod shape_job_timeout;

pub(crate) mod shape_key_values_pair;

pub(crate) mod shape_latest_service_job_attempt;

pub(crate) mod shape_list_jobs_by_consumable_resource_summary_list;

pub(crate) mod shape_node_overrides;

pub(crate) mod shape_node_properties;

pub(crate) mod shape_retry_strategy;

pub(crate) mod shape_scheduling_policy_detail_list;

pub(crate) mod shape_scheduling_policy_listing_detail_list;

pub(crate) mod shape_service_environment_detail_list;

pub(crate) mod shape_service_environment_order;

pub(crate) mod shape_service_job_attempt_details;

pub(crate) mod shape_service_job_retry_strategy;

pub(crate) mod shape_service_job_summary_list;

pub(crate) mod shape_service_job_timeout;

pub(crate) mod shape_tagris_tags_map;

pub(crate) mod shape_update_policy;

pub(crate) mod shape_compute_environment_detail;

pub(crate) mod shape_consumable_resource_requirement;

pub(crate) mod shape_consumable_resource_summary;

pub(crate) mod shape_ec2_configuration;

pub(crate) mod shape_ecs_task_properties;

pub(crate) mod shape_eks_pod_properties;

pub(crate) mod shape_eks_pod_properties_override;

pub(crate) mod shape_ephemeral_storage;

pub(crate) mod shape_evaluate_on_exit;

pub(crate) mod shape_fargate_platform_configuration;

pub(crate) mod shape_front_of_queue_job_summary_list;

pub(crate) mod shape_job_definition;

pub(crate) mod shape_job_detail;

pub(crate) mod shape_job_queue_detail;

pub(crate) mod shape_job_summary;

pub(crate) mod shape_key_value_pair;

pub(crate) mod shape_launch_template_specification;

pub(crate) mod shape_linux_parameters;

pub(crate) mod shape_list_jobs_by_consumable_resource_summary;

pub(crate) mod shape_log_configuration;

pub(crate) mod shape_mount_point;

pub(crate) mod shape_network_configuration;

pub(crate) mod shape_node_property_override;

pub(crate) mod shape_node_range_property;

pub(crate) mod shape_repository_credentials;

pub(crate) mod shape_resource_requirement;

pub(crate) mod shape_runtime_platform;

pub(crate) mod shape_scheduling_policy_detail;

pub(crate) mod shape_scheduling_policy_listing_detail;

pub(crate) mod shape_secret;

pub(crate) mod shape_service_environment_detail;

pub(crate) mod shape_service_job_attempt_detail;

pub(crate) mod shape_service_job_evaluate_on_exit;

pub(crate) mod shape_service_job_evaluate_on_exit_list;

pub(crate) mod shape_service_job_summary;

pub(crate) mod shape_service_resource_id;

pub(crate) mod shape_share_attributes;

pub(crate) mod shape_task_properties_override;

pub(crate) mod shape_ulimit;

pub(crate) mod shape_volume;

pub(crate) mod shape_array_properties_detail;

pub(crate) mod shape_array_properties_summary;

pub(crate) mod shape_attempt_details;

pub(crate) mod shape_capacity_limits;

pub(crate) mod shape_compute_environment_orders;

pub(crate) mod shape_container_detail;

pub(crate) mod shape_container_summary;

pub(crate) mod shape_device;

pub(crate) mod shape_ecs_properties_detail;

pub(crate) mod shape_efs_volume_configuration;

pub(crate) mod shape_eks_attempt_details;

pub(crate) mod shape_eks_container;

pub(crate) mod shape_eks_container_override;

pub(crate) mod shape_eks_metadata;

pub(crate) mod shape_eks_properties_detail;

pub(crate) mod shape_eks_volume;

pub(crate) mod shape_front_of_queue_job_summary;

pub(crate) mod shape_host;

pub(crate) mod shape_image_pull_secret;

pub(crate) mod shape_job_dependency_list;

pub(crate) mod shape_job_state_time_limit_actions;

pub(crate) mod shape_launch_template_specification_override;

pub(crate) mod shape_node_details;

pub(crate) mod shape_node_properties_summary;

pub(crate) mod shape_parameters_map;

pub(crate) mod shape_platform_capability_list;

pub(crate) mod shape_service_environment_orders;

pub(crate) mod shape_task_container_overrides;

pub(crate) mod shape_task_container_properties;

pub(crate) mod shape_tmpfs;

pub(crate) mod shape_array_job_status_summary;

pub(crate) mod shape_attempt_detail;

pub(crate) mod shape_consumable_resource_list;

pub(crate) mod shape_ec2_configuration_list;

pub(crate) mod shape_efs_authorization_config;

pub(crate) mod shape_eks_attempt_detail;

pub(crate) mod shape_eks_container_environment_variable;

pub(crate) mod shape_eks_container_resource_requirements;

pub(crate) mod shape_eks_container_security_context;

pub(crate) mod shape_eks_container_volume_mount;

pub(crate) mod shape_eks_empty_dir;

pub(crate) mod shape_eks_host_path;

pub(crate) mod shape_eks_persistent_volume_claim;

pub(crate) mod shape_eks_pod_properties_detail;

pub(crate) mod shape_eks_secret;

pub(crate) mod shape_environment_variables;

pub(crate) mod shape_evaluate_on_exit_list;

pub(crate) mod shape_firelens_configuration;

pub(crate) mod shape_list_ecs_task_details;

pub(crate) mod shape_list_ecs_task_properties;

pub(crate) mod shape_mount_points;

pub(crate) mod shape_network_interface_list;

pub(crate) mod shape_node_range_properties;

pub(crate) mod shape_resource_requirements;

pub(crate) mod shape_secret_list;

pub(crate) mod shape_share_attributes_list;

pub(crate) mod shape_string_list;

pub(crate) mod shape_tags_map;

pub(crate) mod shape_task_container_dependency;

pub(crate) mod shape_ulimits;

pub(crate) mod shape_volumes;

pub(crate) mod shape_attempt_container_detail;

pub(crate) mod shape_devices_list;

pub(crate) mod shape_ecs_task_details;

pub(crate) mod shape_eks_attempt_container_details;

pub(crate) mod shape_eks_container_details;

pub(crate) mod shape_eks_containers;

pub(crate) mod shape_eks_volumes;

pub(crate) mod shape_image_pull_secrets;

pub(crate) mod shape_launch_template_specification_override_list;

pub(crate) mod shape_list_attempt_ecs_task_details;

pub(crate) mod shape_log_configuration_options_map;

pub(crate) mod shape_network_interface;

pub(crate) mod shape_tmpfs_list;

pub(crate) mod shape_attempt_ecs_task_details;

pub(crate) mod shape_eks_annotations_map;

pub(crate) mod shape_eks_attempt_container_detail;

pub(crate) mod shape_eks_container_detail;

pub(crate) mod shape_eks_labels_map;

pub(crate) mod shape_list_task_container_details;

pub(crate) mod shape_list_task_container_properties;

pub(crate) mod shape_device_cgroup_permissions;

pub(crate) mod shape_eks_container_environment_variables;

pub(crate) mod shape_eks_container_volume_mounts;

pub(crate) mod shape_list_attempt_task_container_details;

pub(crate) mod shape_task_container_details;

pub(crate) mod shape_attempt_task_container_details;

pub(crate) mod shape_eks_limits;

pub(crate) mod shape_eks_requests;

pub(crate) mod shape_task_container_dependency_list;

pub(crate) mod shape_firelens_configuration_options_map;
