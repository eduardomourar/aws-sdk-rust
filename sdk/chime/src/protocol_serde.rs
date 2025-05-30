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

pub(crate) mod shape_associate_phone_number_with_user;

pub(crate) mod shape_associate_signin_delegate_groups_with_account;

pub(crate) mod shape_batch_create_room_membership;

pub(crate) mod shape_batch_delete_phone_number;

pub(crate) mod shape_batch_suspend_user;

pub(crate) mod shape_batch_unsuspend_user;

pub(crate) mod shape_batch_update_phone_number;

pub(crate) mod shape_batch_update_user;

pub(crate) mod shape_create_account;

pub(crate) mod shape_create_bot;

pub(crate) mod shape_create_meeting_dial_out;

pub(crate) mod shape_create_phone_number_order;

pub(crate) mod shape_create_room;

pub(crate) mod shape_create_room_membership;

pub(crate) mod shape_create_user;

pub(crate) mod shape_delete_account;

pub(crate) mod shape_delete_events_configuration;

pub(crate) mod shape_delete_phone_number;

pub(crate) mod shape_delete_room;

pub(crate) mod shape_delete_room_membership;

pub(crate) mod shape_disassociate_phone_number_from_user;

pub(crate) mod shape_disassociate_signin_delegate_groups_from_account;

pub(crate) mod shape_get_account;

pub(crate) mod shape_get_account_settings;

pub(crate) mod shape_get_bot;

pub(crate) mod shape_get_events_configuration;

pub(crate) mod shape_get_global_settings;

pub(crate) mod shape_get_phone_number;

pub(crate) mod shape_get_phone_number_order;

pub(crate) mod shape_get_phone_number_settings;

pub(crate) mod shape_get_retention_settings;

pub(crate) mod shape_get_room;

pub(crate) mod shape_get_user;

pub(crate) mod shape_get_user_settings;

pub(crate) mod shape_invite_users;

pub(crate) mod shape_list_accounts;

pub(crate) mod shape_list_bots;

pub(crate) mod shape_list_phone_number_orders;

pub(crate) mod shape_list_phone_numbers;

pub(crate) mod shape_list_room_memberships;

pub(crate) mod shape_list_rooms;

pub(crate) mod shape_list_supported_phone_number_countries;

pub(crate) mod shape_list_users;

pub(crate) mod shape_logout_user;

pub(crate) mod shape_put_events_configuration;

pub(crate) mod shape_put_retention_settings;

pub(crate) mod shape_redact_conversation_message;

pub(crate) mod shape_redact_room_message;

pub(crate) mod shape_regenerate_security_token;

pub(crate) mod shape_reset_personal_pin;

pub(crate) mod shape_restore_phone_number;

pub(crate) mod shape_search_available_phone_numbers;

pub(crate) mod shape_update_account;

pub(crate) mod shape_update_account_settings;

pub(crate) mod shape_update_bot;

pub(crate) mod shape_update_global_settings;

pub(crate) mod shape_update_phone_number;

pub(crate) mod shape_update_phone_number_settings;

pub(crate) mod shape_update_room;

pub(crate) mod shape_update_room_membership;

pub(crate) mod shape_update_user;

pub(crate) mod shape_update_user_settings;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_associate_phone_number_with_user_input;

pub(crate) mod shape_associate_signin_delegate_groups_with_account_input;

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_batch_create_room_membership_input;

pub(crate) mod shape_batch_delete_phone_number_input;

pub(crate) mod shape_batch_suspend_user_input;

pub(crate) mod shape_batch_unsuspend_user_input;

pub(crate) mod shape_batch_update_phone_number_input;

pub(crate) mod shape_batch_update_user_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_account_input;

pub(crate) mod shape_create_bot_input;

pub(crate) mod shape_create_meeting_dial_out_input;

pub(crate) mod shape_create_phone_number_order_input;

pub(crate) mod shape_create_room_input;

pub(crate) mod shape_create_room_membership_input;

pub(crate) mod shape_create_user_input;

pub(crate) mod shape_disassociate_signin_delegate_groups_from_account_input;

pub(crate) mod shape_forbidden_exception;

pub(crate) mod shape_invite_users_input;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_put_events_configuration_input;

pub(crate) mod shape_put_retention_settings_input;

pub(crate) mod shape_resource_limit_exceeded_exception;

pub(crate) mod shape_service_failure_exception;

pub(crate) mod shape_service_unavailable_exception;

pub(crate) mod shape_throttled_client_exception;

pub(crate) mod shape_unauthorized_client_exception;

pub(crate) mod shape_unprocessable_entity_exception;

pub(crate) mod shape_update_account_input;

pub(crate) mod shape_update_account_settings_input;

pub(crate) mod shape_update_bot_input;

pub(crate) mod shape_update_global_settings_input;

pub(crate) mod shape_update_phone_number_input;

pub(crate) mod shape_update_phone_number_settings_input;

pub(crate) mod shape_update_room_input;

pub(crate) mod shape_update_room_membership_input;

pub(crate) mod shape_update_user_input;

pub(crate) mod shape_update_user_settings_input;

pub(crate) mod shape_account;

pub(crate) mod shape_account_list;

pub(crate) mod shape_account_settings;

pub(crate) mod shape_alexa_for_business_metadata;

pub(crate) mod shape_bot;

pub(crate) mod shape_bot_list;

pub(crate) mod shape_business_calling_settings;

pub(crate) mod shape_e164_phone_number_list;

pub(crate) mod shape_events_configuration;

pub(crate) mod shape_invite_list;

pub(crate) mod shape_member_error_list;

pub(crate) mod shape_membership_item;

pub(crate) mod shape_phone_number;

pub(crate) mod shape_phone_number_countries_list;

pub(crate) mod shape_phone_number_error_list;

pub(crate) mod shape_phone_number_list;

pub(crate) mod shape_phone_number_order;

pub(crate) mod shape_phone_number_order_list;

pub(crate) mod shape_retention_settings;

pub(crate) mod shape_room;

pub(crate) mod shape_room_list;

pub(crate) mod shape_room_membership;

pub(crate) mod shape_room_membership_list;

pub(crate) mod shape_signin_delegate_group;

pub(crate) mod shape_update_phone_number_request_item;

pub(crate) mod shape_update_user_request_item;

pub(crate) mod shape_user;

pub(crate) mod shape_user_error_list;

pub(crate) mod shape_user_list;

pub(crate) mod shape_user_settings;

pub(crate) mod shape_voice_connector_settings;

pub(crate) mod shape_conversation_retention_settings;

pub(crate) mod shape_invite;

pub(crate) mod shape_license_list;

pub(crate) mod shape_member;

pub(crate) mod shape_member_error;

pub(crate) mod shape_ordered_phone_number_list;

pub(crate) mod shape_phone_number_association_list;

pub(crate) mod shape_phone_number_capabilities;

pub(crate) mod shape_phone_number_country;

pub(crate) mod shape_phone_number_error;

pub(crate) mod shape_room_retention_settings;

pub(crate) mod shape_signin_delegate_group_list;

pub(crate) mod shape_telephony_settings;

pub(crate) mod shape_user_error;

pub(crate) mod shape_ordered_phone_number;

pub(crate) mod shape_phone_number_association;

pub(crate) mod shape_phone_number_type_list;
