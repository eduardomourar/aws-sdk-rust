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

pub(crate) mod shape_create_cell;

pub(crate) mod shape_create_cross_account_authorization;

pub(crate) mod shape_create_readiness_check;

pub(crate) mod shape_create_recovery_group;

pub(crate) mod shape_create_resource_set;

pub(crate) mod shape_delete_cell;

pub(crate) mod shape_delete_cross_account_authorization;

pub(crate) mod shape_delete_readiness_check;

pub(crate) mod shape_delete_recovery_group;

pub(crate) mod shape_delete_resource_set;

pub(crate) mod shape_get_architecture_recommendations;

pub(crate) mod shape_get_cell;

pub(crate) mod shape_get_cell_readiness_summary;

pub(crate) mod shape_get_readiness_check;

pub(crate) mod shape_get_readiness_check_resource_status;

pub(crate) mod shape_get_readiness_check_status;

pub(crate) mod shape_get_recovery_group;

pub(crate) mod shape_get_recovery_group_readiness_summary;

pub(crate) mod shape_get_resource_set;

pub(crate) mod shape_list_cells;

pub(crate) mod shape_list_cross_account_authorizations;

pub(crate) mod shape_list_readiness_checks;

pub(crate) mod shape_list_recovery_groups;

pub(crate) mod shape_list_resource_sets;

pub(crate) mod shape_list_rules;

pub(crate) mod shape_list_tags_for_resources;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_cell;

pub(crate) mod shape_update_readiness_check;

pub(crate) mod shape_update_recovery_group;

pub(crate) mod shape_update_resource_set;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_cell_input;

pub(crate) mod shape_create_cross_account_authorization_input;

pub(crate) mod shape_create_readiness_check_input;

pub(crate) mod shape_create_recovery_group_input;

pub(crate) mod shape_create_resource_set_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_cell_input;

pub(crate) mod shape_update_readiness_check_input;

pub(crate) mod shape_update_recovery_group_input;

pub(crate) mod shape_update_resource_set_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_list_of_cell_output;

pub(crate) mod shape_list_of_cross_account_authorization;

pub(crate) mod shape_list_of_list_rules_output;

pub(crate) mod shape_list_of_message;

pub(crate) mod shape_list_of_readiness_check_output;

pub(crate) mod shape_list_of_readiness_check_summary;

pub(crate) mod shape_list_of_recommendation;

pub(crate) mod shape_list_of_recovery_group_output;

pub(crate) mod shape_list_of_resource;

pub(crate) mod shape_list_of_resource_result;

pub(crate) mod shape_list_of_resource_set_output;

pub(crate) mod shape_list_of_rule_result;

pub(crate) mod shape_list_of_string;

pub(crate) mod shape_resource;

pub(crate) mod shape_tags;

pub(crate) mod shape_cell_output;

pub(crate) mod shape_dns_target_resource;

pub(crate) mod shape_list_rules_output;

pub(crate) mod shape_message;

pub(crate) mod shape_readiness_check_output;

pub(crate) mod shape_readiness_check_summary;

pub(crate) mod shape_recommendation;

pub(crate) mod shape_recovery_group_output;

pub(crate) mod shape_resource_result;

pub(crate) mod shape_resource_set_output;

pub(crate) mod shape_rule_result;

pub(crate) mod shape_target_resource;

pub(crate) mod shape_nlb_resource;

pub(crate) mod shape_r53_resource_record;
