// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn access_denied_exception_correct_errors(
    mut builder: crate::types::error::builders::AccessDeniedExceptionBuilder,
) -> crate::types::error::builders::AccessDeniedExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn conflict_exception_correct_errors(
    mut builder: crate::types::error::builders::ConflictExceptionBuilder,
) -> crate::types::error::builders::ConflictExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn internal_server_exception_correct_errors(
    mut builder: crate::types::error::builders::InternalServerExceptionBuilder,
) -> crate::types::error::builders::InternalServerExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn resource_not_found_exception_correct_errors(
    mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
) -> crate::types::error::builders::ResourceNotFoundExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn service_quota_exceeded_exception_correct_errors(
    mut builder: crate::types::error::builders::ServiceQuotaExceededExceptionBuilder,
) -> crate::types::error::builders::ServiceQuotaExceededExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn throttling_exception_correct_errors(
    mut builder: crate::types::error::builders::ThrottlingExceptionBuilder,
) -> crate::types::error::builders::ThrottlingExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_correct_errors(
    mut builder: crate::types::error::builders::ValidationExceptionBuilder,
) -> crate::types::error::builders::ValidationExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn assertion_rule_correct_errors(
    mut builder: crate::types::builders::AssertionRuleBuilder,
) -> crate::types::builders::AssertionRuleBuilder {
    if builder.asserted_controls.is_none() {
        builder.asserted_controls = Some(Default::default())
    }
    if builder.control_panel_arn.is_none() {
        builder.control_panel_arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.rule_config.is_none() {
        builder.rule_config = {
            let builder = crate::types::builders::RuleConfigBuilder::default();
            Some(crate::serde_util::rule_config_correct_errors(builder).build())
        }
    }
    if builder.safety_rule_arn.is_none() {
        builder.safety_rule_arn = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::Status>().ok()
    }
    if builder.wait_period_ms.is_none() {
        builder.wait_period_ms = Some(Default::default())
    }
    builder
}

pub(crate) fn gating_rule_correct_errors(mut builder: crate::types::builders::GatingRuleBuilder) -> crate::types::builders::GatingRuleBuilder {
    if builder.control_panel_arn.is_none() {
        builder.control_panel_arn = Some(Default::default())
    }
    if builder.gating_controls.is_none() {
        builder.gating_controls = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.rule_config.is_none() {
        builder.rule_config = {
            let builder = crate::types::builders::RuleConfigBuilder::default();
            Some(crate::serde_util::rule_config_correct_errors(builder).build())
        }
    }
    if builder.safety_rule_arn.is_none() {
        builder.safety_rule_arn = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::Status>().ok()
    }
    if builder.target_controls.is_none() {
        builder.target_controls = Some(Default::default())
    }
    if builder.wait_period_ms.is_none() {
        builder.wait_period_ms = Some(Default::default())
    }
    builder
}

pub(crate) fn rule_config_correct_errors(mut builder: crate::types::builders::RuleConfigBuilder) -> crate::types::builders::RuleConfigBuilder {
    if builder.inverted.is_none() {
        builder.inverted = Some(Default::default())
    }
    if builder.threshold.is_none() {
        builder.threshold = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::RuleType>().ok()
    }
    builder
}
