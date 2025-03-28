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

pub(crate) mod shape_cancel_task;

pub(crate) mod shape_create_task;

pub(crate) mod shape_describe_device;

pub(crate) mod shape_describe_device_ec2_instances;

pub(crate) mod shape_describe_execution;

pub(crate) mod shape_describe_task;

pub(crate) mod shape_list_device_resources;

pub(crate) mod shape_list_devices;

pub(crate) mod shape_list_executions;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_tasks;

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

pub(crate) mod shape_create_task_input;

pub(crate) mod shape_describe_device_ec2_instances_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_capacity_list;

pub(crate) mod shape_command;

pub(crate) mod shape_device_summary_list;

pub(crate) mod shape_execution_summary_list;

pub(crate) mod shape_instance_summary_list;

pub(crate) mod shape_physical_network_interface_list;

pub(crate) mod shape_resource_summary_list;

pub(crate) mod shape_software_information;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_target_list;

pub(crate) mod shape_task_summary_list;

pub(crate) mod shape_capacity;

pub(crate) mod shape_device_summary;

pub(crate) mod shape_execution_summary;

pub(crate) mod shape_instance_summary;

pub(crate) mod shape_physical_network_interface;

pub(crate) mod shape_reboot;

pub(crate) mod shape_resource_summary;

pub(crate) mod shape_task_summary;

pub(crate) mod shape_unlock;

pub(crate) mod shape_instance;

pub(crate) mod shape_cpu_options;

pub(crate) mod shape_instance_block_device_mapping_list;

pub(crate) mod shape_instance_state;

pub(crate) mod shape_security_group_identifier_list;

pub(crate) mod shape_instance_block_device_mapping;

pub(crate) mod shape_security_group_identifier;

pub(crate) mod shape_ebs_instance_block_device;
