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

pub(crate) mod shape_add_custom_attributes;

pub(crate) mod shape_admin_add_user_to_group;

pub(crate) mod shape_admin_confirm_sign_up;

pub(crate) mod shape_admin_create_user;

pub(crate) mod shape_admin_delete_user;

pub(crate) mod shape_admin_delete_user_attributes;

pub(crate) mod shape_admin_disable_provider_for_user;

pub(crate) mod shape_admin_disable_user;

pub(crate) mod shape_admin_enable_user;

pub(crate) mod shape_admin_forget_device;

pub(crate) mod shape_admin_get_device;

pub(crate) mod shape_admin_get_user;

pub(crate) mod shape_admin_initiate_auth;

pub(crate) mod shape_admin_link_provider_for_user;

pub(crate) mod shape_admin_list_devices;

pub(crate) mod shape_admin_list_groups_for_user;

pub(crate) mod shape_admin_list_user_auth_events;

pub(crate) mod shape_admin_remove_user_from_group;

pub(crate) mod shape_admin_reset_user_password;

pub(crate) mod shape_admin_respond_to_auth_challenge;

pub(crate) mod shape_admin_set_user_mfa_preference;

pub(crate) mod shape_admin_set_user_password;

pub(crate) mod shape_admin_set_user_settings;

pub(crate) mod shape_admin_update_auth_event_feedback;

pub(crate) mod shape_admin_update_device_status;

pub(crate) mod shape_admin_update_user_attributes;

pub(crate) mod shape_admin_user_global_sign_out;

pub(crate) mod shape_associate_software_token;

pub(crate) mod shape_change_password;

pub(crate) mod shape_complete_web_authn_registration;

pub(crate) mod shape_confirm_device;

pub(crate) mod shape_confirm_forgot_password;

pub(crate) mod shape_confirm_sign_up;

pub(crate) mod shape_create_group;

pub(crate) mod shape_create_identity_provider;

pub(crate) mod shape_create_managed_login_branding;

pub(crate) mod shape_create_resource_server;

pub(crate) mod shape_create_user_import_job;

pub(crate) mod shape_create_user_pool;

pub(crate) mod shape_create_user_pool_client;

pub(crate) mod shape_create_user_pool_domain;

pub(crate) mod shape_delete_group;

pub(crate) mod shape_delete_identity_provider;

pub(crate) mod shape_delete_managed_login_branding;

pub(crate) mod shape_delete_resource_server;

pub(crate) mod shape_delete_user;

pub(crate) mod shape_delete_user_attributes;

pub(crate) mod shape_delete_user_pool;

pub(crate) mod shape_delete_user_pool_client;

pub(crate) mod shape_delete_user_pool_domain;

pub(crate) mod shape_delete_web_authn_credential;

pub(crate) mod shape_describe_identity_provider;

pub(crate) mod shape_describe_managed_login_branding;

pub(crate) mod shape_describe_managed_login_branding_by_client;

pub(crate) mod shape_describe_resource_server;

pub(crate) mod shape_describe_risk_configuration;

pub(crate) mod shape_describe_user_import_job;

pub(crate) mod shape_describe_user_pool;

pub(crate) mod shape_describe_user_pool_client;

pub(crate) mod shape_describe_user_pool_domain;

pub(crate) mod shape_forget_device;

pub(crate) mod shape_forgot_password;

pub(crate) mod shape_get_csv_header;

pub(crate) mod shape_get_device;

pub(crate) mod shape_get_group;

pub(crate) mod shape_get_identity_provider_by_identifier;

pub(crate) mod shape_get_log_delivery_configuration;

pub(crate) mod shape_get_signing_certificate;

pub(crate) mod shape_get_tokens_from_refresh_token;

pub(crate) mod shape_get_ui_customization;

pub(crate) mod shape_get_user;

pub(crate) mod shape_get_user_attribute_verification_code;

pub(crate) mod shape_get_user_auth_factors;

pub(crate) mod shape_get_user_pool_mfa_config;

pub(crate) mod shape_global_sign_out;

pub(crate) mod shape_initiate_auth;

pub(crate) mod shape_list_devices;

pub(crate) mod shape_list_groups;

pub(crate) mod shape_list_identity_providers;

pub(crate) mod shape_list_resource_servers;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_user_import_jobs;

pub(crate) mod shape_list_user_pool_clients;

pub(crate) mod shape_list_user_pools;

pub(crate) mod shape_list_users;

pub(crate) mod shape_list_users_in_group;

pub(crate) mod shape_list_web_authn_credentials;

pub(crate) mod shape_resend_confirmation_code;

pub(crate) mod shape_respond_to_auth_challenge;

pub(crate) mod shape_revoke_token;

pub(crate) mod shape_set_log_delivery_configuration;

pub(crate) mod shape_set_risk_configuration;

pub(crate) mod shape_set_ui_customization;

pub(crate) mod shape_set_user_mfa_preference;

pub(crate) mod shape_set_user_pool_mfa_config;

pub(crate) mod shape_set_user_settings;

pub(crate) mod shape_sign_up;

pub(crate) mod shape_start_user_import_job;

pub(crate) mod shape_start_web_authn_registration;

pub(crate) mod shape_stop_user_import_job;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_auth_event_feedback;

pub(crate) mod shape_update_device_status;

pub(crate) mod shape_update_group;

pub(crate) mod shape_update_identity_provider;

pub(crate) mod shape_update_managed_login_branding;

pub(crate) mod shape_update_resource_server;

pub(crate) mod shape_update_user_attributes;

pub(crate) mod shape_update_user_pool;

pub(crate) mod shape_update_user_pool_client;

pub(crate) mod shape_update_user_pool_domain;

pub(crate) mod shape_verify_software_token;

pub(crate) mod shape_verify_user_attribute;

pub(crate) mod shape_add_custom_attributes_input;

pub(crate) mod shape_admin_add_user_to_group_input;

pub(crate) mod shape_admin_confirm_sign_up_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_admin_create_user_input;

pub(crate) mod shape_admin_delete_user_attributes_input;

pub(crate) mod shape_admin_delete_user_input;

pub(crate) mod shape_admin_disable_provider_for_user_input;

pub(crate) mod shape_admin_disable_user_input;

pub(crate) mod shape_admin_enable_user_input;

pub(crate) mod shape_admin_forget_device_input;

pub(crate) mod shape_admin_get_device_input;

pub(crate) mod shape_admin_get_user_input;

pub(crate) mod shape_admin_initiate_auth_input;

pub(crate) mod shape_admin_link_provider_for_user_input;

pub(crate) mod shape_admin_list_devices_input;

pub(crate) mod shape_admin_list_groups_for_user_input;

pub(crate) mod shape_admin_list_user_auth_events_input;

pub(crate) mod shape_admin_remove_user_from_group_input;

pub(crate) mod shape_admin_reset_user_password_input;

pub(crate) mod shape_admin_respond_to_auth_challenge_input;

pub(crate) mod shape_admin_set_user_mfa_preference_input;

pub(crate) mod shape_admin_set_user_password_input;

pub(crate) mod shape_admin_set_user_settings_input;

pub(crate) mod shape_admin_update_auth_event_feedback_input;

pub(crate) mod shape_admin_update_device_status_input;

pub(crate) mod shape_admin_update_user_attributes_input;

pub(crate) mod shape_admin_user_global_sign_out_input;

pub(crate) mod shape_alias_exists_exception;

pub(crate) mod shape_associate_software_token_input;

pub(crate) mod shape_change_password_input;

pub(crate) mod shape_code_delivery_failure_exception;

pub(crate) mod shape_code_mismatch_exception;

pub(crate) mod shape_complete_web_authn_registration_input;

pub(crate) mod shape_concurrent_modification_exception;

pub(crate) mod shape_confirm_device_input;

pub(crate) mod shape_confirm_forgot_password_input;

pub(crate) mod shape_confirm_sign_up_input;

pub(crate) mod shape_create_group_input;

pub(crate) mod shape_create_identity_provider_input;

pub(crate) mod shape_create_managed_login_branding_input;

pub(crate) mod shape_create_resource_server_input;

pub(crate) mod shape_create_user_import_job_input;

pub(crate) mod shape_create_user_pool_client_input;

pub(crate) mod shape_create_user_pool_domain_input;

pub(crate) mod shape_create_user_pool_input;

pub(crate) mod shape_delete_group_input;

pub(crate) mod shape_delete_identity_provider_input;

pub(crate) mod shape_delete_managed_login_branding_input;

pub(crate) mod shape_delete_resource_server_input;

pub(crate) mod shape_delete_user_attributes_input;

pub(crate) mod shape_delete_user_input;

pub(crate) mod shape_delete_user_pool_client_input;

pub(crate) mod shape_delete_user_pool_domain_input;

pub(crate) mod shape_delete_user_pool_input;

pub(crate) mod shape_delete_web_authn_credential_input;

pub(crate) mod shape_describe_identity_provider_input;

pub(crate) mod shape_describe_managed_login_branding_by_client_input;

pub(crate) mod shape_describe_managed_login_branding_input;

pub(crate) mod shape_describe_resource_server_input;

pub(crate) mod shape_describe_risk_configuration_input;

pub(crate) mod shape_describe_user_import_job_input;

pub(crate) mod shape_describe_user_pool_client_input;

pub(crate) mod shape_describe_user_pool_domain_input;

pub(crate) mod shape_describe_user_pool_input;

pub(crate) mod shape_device_key_exists_exception;

pub(crate) mod shape_duplicate_provider_exception;

pub(crate) mod shape_enable_software_token_mfa_exception;

pub(crate) mod shape_expired_code_exception;

pub(crate) mod shape_feature_unavailable_in_tier_exception;

pub(crate) mod shape_forbidden_exception;

pub(crate) mod shape_forget_device_input;

pub(crate) mod shape_forgot_password_input;

pub(crate) mod shape_get_csv_header_input;

pub(crate) mod shape_get_device_input;

pub(crate) mod shape_get_group_input;

pub(crate) mod shape_get_identity_provider_by_identifier_input;

pub(crate) mod shape_get_log_delivery_configuration_input;

pub(crate) mod shape_get_signing_certificate_input;

pub(crate) mod shape_get_tokens_from_refresh_token_input;

pub(crate) mod shape_get_ui_customization_input;

pub(crate) mod shape_get_user_attribute_verification_code_input;

pub(crate) mod shape_get_user_auth_factors_input;

pub(crate) mod shape_get_user_input;

pub(crate) mod shape_get_user_pool_mfa_config_input;

pub(crate) mod shape_global_sign_out_input;

pub(crate) mod shape_group_exists_exception;

pub(crate) mod shape_initiate_auth_input;

pub(crate) mod shape_internal_error_exception;

pub(crate) mod shape_invalid_email_role_access_policy_exception;

pub(crate) mod shape_invalid_lambda_response_exception;

pub(crate) mod shape_invalid_o_auth_flow_exception;

pub(crate) mod shape_invalid_parameter_exception;

pub(crate) mod shape_invalid_password_exception;

pub(crate) mod shape_invalid_sms_role_access_policy_exception;

pub(crate) mod shape_invalid_sms_role_trust_relationship_exception;

pub(crate) mod shape_invalid_user_pool_configuration_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_list_devices_input;

pub(crate) mod shape_list_groups_input;

pub(crate) mod shape_list_identity_providers_input;

pub(crate) mod shape_list_resource_servers_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_list_user_import_jobs_input;

pub(crate) mod shape_list_user_pool_clients_input;

pub(crate) mod shape_list_user_pools_input;

pub(crate) mod shape_list_users_in_group_input;

pub(crate) mod shape_list_users_input;

pub(crate) mod shape_list_web_authn_credentials_input;

pub(crate) mod shape_managed_login_branding_exists_exception;

pub(crate) mod shape_mfa_method_not_found_exception;

pub(crate) mod shape_not_authorized_exception;

pub(crate) mod shape_password_history_policy_violation_exception;

pub(crate) mod shape_password_reset_required_exception;

pub(crate) mod shape_precondition_not_met_exception;

pub(crate) mod shape_refresh_token_reuse_exception;

pub(crate) mod shape_resend_confirmation_code_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_respond_to_auth_challenge_input;

pub(crate) mod shape_revoke_token_input;

pub(crate) mod shape_scope_does_not_exist_exception;

pub(crate) mod shape_set_log_delivery_configuration_input;

pub(crate) mod shape_set_risk_configuration_input;

pub(crate) mod shape_set_ui_customization_input;

pub(crate) mod shape_set_user_mfa_preference_input;

pub(crate) mod shape_set_user_pool_mfa_config_input;

pub(crate) mod shape_set_user_settings_input;

pub(crate) mod shape_sign_up_input;

pub(crate) mod shape_software_token_mfa_not_found_exception;

pub(crate) mod shape_start_user_import_job_input;

pub(crate) mod shape_start_web_authn_registration_input;

pub(crate) mod shape_stop_user_import_job_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_tier_change_not_allowed_exception;

pub(crate) mod shape_too_many_failed_attempts_exception;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_unauthorized_exception;

pub(crate) mod shape_unexpected_lambda_exception;

pub(crate) mod shape_unsupported_identity_provider_exception;

pub(crate) mod shape_unsupported_operation_exception;

pub(crate) mod shape_unsupported_token_type_exception;

pub(crate) mod shape_unsupported_user_state_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_auth_event_feedback_input;

pub(crate) mod shape_update_device_status_input;

pub(crate) mod shape_update_group_input;

pub(crate) mod shape_update_identity_provider_input;

pub(crate) mod shape_update_managed_login_branding_input;

pub(crate) mod shape_update_resource_server_input;

pub(crate) mod shape_update_user_attributes_input;

pub(crate) mod shape_update_user_pool_client_input;

pub(crate) mod shape_update_user_pool_domain_input;

pub(crate) mod shape_update_user_pool_input;

pub(crate) mod shape_user_import_in_progress_exception;

pub(crate) mod shape_user_lambda_validation_exception;

pub(crate) mod shape_user_not_confirmed_exception;

pub(crate) mod shape_user_not_found_exception;

pub(crate) mod shape_user_pool_add_on_not_enabled_exception;

pub(crate) mod shape_user_pool_tagging_exception;

pub(crate) mod shape_username_exists_exception;

pub(crate) mod shape_verify_software_token_input;

pub(crate) mod shape_verify_user_attribute_input;

pub(crate) mod shape_web_authn_challenge_not_found_exception;

pub(crate) mod shape_web_authn_client_mismatch_exception;

pub(crate) mod shape_web_authn_configuration_missing_exception;

pub(crate) mod shape_web_authn_credential_not_supported_exception;

pub(crate) mod shape_web_authn_not_enabled_exception;

pub(crate) mod shape_web_authn_origin_not_allowed_exception;

pub(crate) mod shape_web_authn_relying_party_mismatch_exception;

pub(crate) mod shape_account_recovery_setting_type;

pub(crate) mod shape_account_takeover_risk_configuration_type;

pub(crate) mod shape_admin_create_user_config_type;

pub(crate) mod shape_analytics_configuration_type;

pub(crate) mod shape_analytics_metadata_type;

pub(crate) mod shape_asset_type;

pub(crate) mod shape_attribute_list_type;

pub(crate) mod shape_attribute_type;

pub(crate) mod shape_auth_events_type;

pub(crate) mod shape_authentication_result_type;

pub(crate) mod shape_available_challenge_list_type;

pub(crate) mod shape_challenge_parameters_type;

pub(crate) mod shape_code_delivery_details_list_type;

pub(crate) mod shape_code_delivery_details_type;

pub(crate) mod shape_compromised_credentials_risk_configuration_type;

pub(crate) mod shape_configured_user_auth_factors_list_type;

pub(crate) mod shape_context_data_type;

pub(crate) mod shape_custom_domain_config_type;

pub(crate) mod shape_device_configuration_type;

pub(crate) mod shape_device_list_type;

pub(crate) mod shape_device_secret_verifier_config_type;

pub(crate) mod shape_device_type;

pub(crate) mod shape_domain_description_type;

pub(crate) mod shape_email_configuration_type;

pub(crate) mod shape_email_mfa_config_type;

pub(crate) mod shape_email_mfa_settings_type;

pub(crate) mod shape_group_list_type;

pub(crate) mod shape_group_type;

pub(crate) mod shape_identity_provider_type;

pub(crate) mod shape_lambda_config_type;

pub(crate) mod shape_list_of_string_types;

pub(crate) mod shape_log_configuration_type;

pub(crate) mod shape_log_delivery_configuration_type;

pub(crate) mod shape_managed_login_branding_type;

pub(crate) mod shape_mfa_option_list_type;

pub(crate) mod shape_mfa_option_type;

pub(crate) mod shape_provider_user_identifier_type;

pub(crate) mod shape_providers_list_type;

pub(crate) mod shape_refresh_token_rotation_type;

pub(crate) mod shape_resource_server_scope_type;

pub(crate) mod shape_resource_server_type;

pub(crate) mod shape_resource_servers_list_type;

pub(crate) mod shape_risk_configuration_type;

pub(crate) mod shape_risk_exception_configuration_type;

pub(crate) mod shape_schema_attribute_type;

pub(crate) mod shape_sms_configuration_type;

pub(crate) mod shape_sms_mfa_config_type;

pub(crate) mod shape_sms_mfa_settings_type;

pub(crate) mod shape_software_token_mfa_config_type;

pub(crate) mod shape_software_token_mfa_settings_type;

pub(crate) mod shape_token_validity_units_type;

pub(crate) mod shape_ui_customization_type;

pub(crate) mod shape_user_attribute_update_settings_type;

pub(crate) mod shape_user_context_data_type;

pub(crate) mod shape_user_import_job_type;

pub(crate) mod shape_user_import_jobs_list_type;

pub(crate) mod shape_user_mfa_setting_list_type;

pub(crate) mod shape_user_pool_add_ons_type;

pub(crate) mod shape_user_pool_client_list_type;

pub(crate) mod shape_user_pool_client_type;

pub(crate) mod shape_user_pool_list_type;

pub(crate) mod shape_user_pool_policy_type;

pub(crate) mod shape_user_pool_tags_type;

pub(crate) mod shape_user_pool_type;

pub(crate) mod shape_user_type;

pub(crate) mod shape_username_configuration_type;

pub(crate) mod shape_users_list_type;

pub(crate) mod shape_verification_message_template_type;

pub(crate) mod shape_web_authn_configuration_type;

pub(crate) mod shape_web_authn_credential_description_list_type;

pub(crate) mod shape_account_takeover_actions_type;

pub(crate) mod shape_advanced_security_additional_flows_type;

pub(crate) mod shape_alias_attributes_list_type;

pub(crate) mod shape_asset_list_type;

pub(crate) mod shape_attribute_mapping_type;

pub(crate) mod shape_auth_event_type;

pub(crate) mod shape_callback_urls_list_type;

pub(crate) mod shape_client_permission_list_type;

pub(crate) mod shape_cloud_watch_logs_configuration_type;

pub(crate) mod shape_compromised_credentials_actions_type;

pub(crate) mod shape_custom_email_lambda_version_config_type;

pub(crate) mod shape_custom_sms_lambda_version_config_type;

pub(crate) mod shape_explicit_auth_flows_list_type;

pub(crate) mod shape_firehose_configuration_type;

pub(crate) mod shape_http_header;

pub(crate) mod shape_idp_identifiers_list_type;

pub(crate) mod shape_log_configuration_list_type;

pub(crate) mod shape_logout_urls_list_type;

pub(crate) mod shape_message_template_type;

pub(crate) mod shape_new_device_metadata_type;

pub(crate) mod shape_notify_configuration_type;

pub(crate) mod shape_number_attribute_constraints_type;

pub(crate) mod shape_o_auth_flows_type;

pub(crate) mod shape_password_policy_type;

pub(crate) mod shape_pre_token_generation_version_config_type;

pub(crate) mod shape_provider_description;

pub(crate) mod shape_provider_details_type;

pub(crate) mod shape_recovery_option_type;

pub(crate) mod shape_resource_server_scope_list_type;

pub(crate) mod shape_s3_configuration_type;

pub(crate) mod shape_schema_attributes_list_type;

pub(crate) mod shape_scope_list_type;

pub(crate) mod shape_sign_in_policy_type;

pub(crate) mod shape_string_attribute_constraints_type;

pub(crate) mod shape_supported_identity_providers_list_type;

pub(crate) mod shape_user_pool_client_description;

pub(crate) mod shape_user_pool_description_type;

pub(crate) mod shape_username_attributes_list_type;

pub(crate) mod shape_verified_attributes_list_type;

pub(crate) mod shape_web_authn_credential_description;

pub(crate) mod shape_account_takeover_action_type;

pub(crate) mod shape_attributes_require_verification_before_update_type;

pub(crate) mod shape_blocked_ip_range_list_type;

pub(crate) mod shape_challenge_response_list_type;

pub(crate) mod shape_event_context_data_type;

pub(crate) mod shape_event_feedback_type;

pub(crate) mod shape_event_filters_type;

pub(crate) mod shape_event_risk_type;

pub(crate) mod shape_notify_email_type;

pub(crate) mod shape_recovery_mechanisms_type;

pub(crate) mod shape_skipped_ip_range_list_type;

pub(crate) mod shape_web_authn_authenticator_transports_list;

pub(crate) mod shape_allowed_first_auth_factors_list_type;

pub(crate) mod shape_challenge_response_type;
