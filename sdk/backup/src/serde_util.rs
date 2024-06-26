// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn create_restore_testing_plan_output_output_correct_errors(
    mut builder: crate::operation::create_restore_testing_plan::builders::CreateRestoreTestingPlanOutputBuilder,
) -> crate::operation::create_restore_testing_plan::builders::CreateRestoreTestingPlanOutputBuilder {
    if builder.creation_time.is_none() {
        builder.creation_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.restore_testing_plan_arn.is_none() {
        builder.restore_testing_plan_arn = Some(Default::default())
    }
    if builder.restore_testing_plan_name.is_none() {
        builder.restore_testing_plan_name = Some(Default::default())
    }
    builder
}

pub(crate) fn create_restore_testing_selection_output_output_correct_errors(
    mut builder: crate::operation::create_restore_testing_selection::builders::CreateRestoreTestingSelectionOutputBuilder,
) -> crate::operation::create_restore_testing_selection::builders::CreateRestoreTestingSelectionOutputBuilder {
    if builder.creation_time.is_none() {
        builder.creation_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.restore_testing_plan_arn.is_none() {
        builder.restore_testing_plan_arn = Some(Default::default())
    }
    if builder.restore_testing_plan_name.is_none() {
        builder.restore_testing_plan_name = Some(Default::default())
    }
    if builder.restore_testing_selection_name.is_none() {
        builder.restore_testing_selection_name = Some(Default::default())
    }
    builder
}

pub(crate) fn get_restore_testing_inferred_metadata_output_output_correct_errors(
    mut builder: crate::operation::get_restore_testing_inferred_metadata::builders::GetRestoreTestingInferredMetadataOutputBuilder,
) -> crate::operation::get_restore_testing_inferred_metadata::builders::GetRestoreTestingInferredMetadataOutputBuilder {
    if builder.inferred_metadata.is_none() {
        builder.inferred_metadata = Some(Default::default())
    }
    builder
}

pub(crate) fn get_restore_testing_plan_output_output_correct_errors(
    mut builder: crate::operation::get_restore_testing_plan::builders::GetRestoreTestingPlanOutputBuilder,
) -> crate::operation::get_restore_testing_plan::builders::GetRestoreTestingPlanOutputBuilder {
    if builder.restore_testing_plan.is_none() {
        builder.restore_testing_plan = {
            let builder = crate::types::builders::RestoreTestingPlanForGetBuilder::default();
            crate::serde_util::restore_testing_plan_for_get_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn get_restore_testing_selection_output_output_correct_errors(
    mut builder: crate::operation::get_restore_testing_selection::builders::GetRestoreTestingSelectionOutputBuilder,
) -> crate::operation::get_restore_testing_selection::builders::GetRestoreTestingSelectionOutputBuilder {
    if builder.restore_testing_selection.is_none() {
        builder.restore_testing_selection = {
            let builder = crate::types::builders::RestoreTestingSelectionForGetBuilder::default();
            crate::serde_util::restore_testing_selection_for_get_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn list_restore_testing_plans_output_output_correct_errors(
    mut builder: crate::operation::list_restore_testing_plans::builders::ListRestoreTestingPlansOutputBuilder,
) -> crate::operation::list_restore_testing_plans::builders::ListRestoreTestingPlansOutputBuilder {
    if builder.restore_testing_plans.is_none() {
        builder.restore_testing_plans = Some(Default::default())
    }
    builder
}

pub(crate) fn list_restore_testing_selections_output_output_correct_errors(
    mut builder: crate::operation::list_restore_testing_selections::builders::ListRestoreTestingSelectionsOutputBuilder,
) -> crate::operation::list_restore_testing_selections::builders::ListRestoreTestingSelectionsOutputBuilder {
    if builder.restore_testing_selections.is_none() {
        builder.restore_testing_selections = Some(Default::default())
    }
    builder
}

pub(crate) fn update_restore_testing_plan_output_output_correct_errors(
    mut builder: crate::operation::update_restore_testing_plan::builders::UpdateRestoreTestingPlanOutputBuilder,
) -> crate::operation::update_restore_testing_plan::builders::UpdateRestoreTestingPlanOutputBuilder {
    if builder.creation_time.is_none() {
        builder.creation_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.restore_testing_plan_arn.is_none() {
        builder.restore_testing_plan_arn = Some(Default::default())
    }
    if builder.restore_testing_plan_name.is_none() {
        builder.restore_testing_plan_name = Some(Default::default())
    }
    if builder.update_time.is_none() {
        builder.update_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn update_restore_testing_selection_output_output_correct_errors(
    mut builder: crate::operation::update_restore_testing_selection::builders::UpdateRestoreTestingSelectionOutputBuilder,
) -> crate::operation::update_restore_testing_selection::builders::UpdateRestoreTestingSelectionOutputBuilder {
    if builder.creation_time.is_none() {
        builder.creation_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.restore_testing_plan_arn.is_none() {
        builder.restore_testing_plan_arn = Some(Default::default())
    }
    if builder.restore_testing_plan_name.is_none() {
        builder.restore_testing_plan_name = Some(Default::default())
    }
    if builder.restore_testing_selection_name.is_none() {
        builder.restore_testing_selection_name = Some(Default::default())
    }
    if builder.update_time.is_none() {
        builder.update_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn restore_testing_plan_for_get_correct_errors(
    mut builder: crate::types::builders::RestoreTestingPlanForGetBuilder,
) -> crate::types::builders::RestoreTestingPlanForGetBuilder {
    if builder.creation_time.is_none() {
        builder.creation_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.recovery_point_selection.is_none() {
        builder.recovery_point_selection = {
            let builder = crate::types::builders::RestoreTestingRecoveryPointSelectionBuilder::default();
            Some(builder.build())
        }
    }
    if builder.restore_testing_plan_arn.is_none() {
        builder.restore_testing_plan_arn = Some(Default::default())
    }
    if builder.restore_testing_plan_name.is_none() {
        builder.restore_testing_plan_name = Some(Default::default())
    }
    if builder.schedule_expression.is_none() {
        builder.schedule_expression = Some(Default::default())
    }
    builder
}

pub(crate) fn restore_testing_selection_for_get_correct_errors(
    mut builder: crate::types::builders::RestoreTestingSelectionForGetBuilder,
) -> crate::types::builders::RestoreTestingSelectionForGetBuilder {
    if builder.creation_time.is_none() {
        builder.creation_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.iam_role_arn.is_none() {
        builder.iam_role_arn = Some(Default::default())
    }
    if builder.protected_resource_type.is_none() {
        builder.protected_resource_type = Some(Default::default())
    }
    if builder.restore_testing_plan_name.is_none() {
        builder.restore_testing_plan_name = Some(Default::default())
    }
    if builder.restore_testing_selection_name.is_none() {
        builder.restore_testing_selection_name = Some(Default::default())
    }
    builder
}

pub(crate) fn backup_plan_correct_errors(mut builder: crate::types::builders::BackupPlanBuilder) -> crate::types::builders::BackupPlanBuilder {
    if builder.backup_plan_name.is_none() {
        builder.backup_plan_name = Some(Default::default())
    }
    if builder.rules.is_none() {
        builder.rules = Some(Default::default())
    }
    builder
}

pub(crate) fn backup_selection_correct_errors(
    mut builder: crate::types::builders::BackupSelectionBuilder,
) -> crate::types::builders::BackupSelectionBuilder {
    if builder.selection_name.is_none() {
        builder.selection_name = Some(Default::default())
    }
    if builder.iam_role_arn.is_none() {
        builder.iam_role_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn date_range_correct_errors(mut builder: crate::types::builders::DateRangeBuilder) -> crate::types::builders::DateRangeBuilder {
    if builder.from_date.is_none() {
        builder.from_date = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.to_date.is_none() {
        builder.to_date = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn framework_control_correct_errors(
    mut builder: crate::types::builders::FrameworkControlBuilder,
) -> crate::types::builders::FrameworkControlBuilder {
    if builder.control_name.is_none() {
        builder.control_name = Some(Default::default())
    }
    builder
}

pub(crate) fn report_delivery_channel_correct_errors(
    mut builder: crate::types::builders::ReportDeliveryChannelBuilder,
) -> crate::types::builders::ReportDeliveryChannelBuilder {
    if builder.s3_bucket_name.is_none() {
        builder.s3_bucket_name = Some(Default::default())
    }
    builder
}

pub(crate) fn report_setting_correct_errors(
    mut builder: crate::types::builders::ReportSettingBuilder,
) -> crate::types::builders::ReportSettingBuilder {
    if builder.report_template.is_none() {
        builder.report_template = Some(Default::default())
    }
    builder
}

pub(crate) fn restore_testing_plan_for_list_correct_errors(
    mut builder: crate::types::builders::RestoreTestingPlanForListBuilder,
) -> crate::types::builders::RestoreTestingPlanForListBuilder {
    if builder.creation_time.is_none() {
        builder.creation_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.restore_testing_plan_arn.is_none() {
        builder.restore_testing_plan_arn = Some(Default::default())
    }
    if builder.restore_testing_plan_name.is_none() {
        builder.restore_testing_plan_name = Some(Default::default())
    }
    if builder.schedule_expression.is_none() {
        builder.schedule_expression = Some(Default::default())
    }
    builder
}

pub(crate) fn restore_testing_selection_for_list_correct_errors(
    mut builder: crate::types::builders::RestoreTestingSelectionForListBuilder,
) -> crate::types::builders::RestoreTestingSelectionForListBuilder {
    if builder.creation_time.is_none() {
        builder.creation_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.iam_role_arn.is_none() {
        builder.iam_role_arn = Some(Default::default())
    }
    if builder.protected_resource_type.is_none() {
        builder.protected_resource_type = Some(Default::default())
    }
    if builder.restore_testing_plan_name.is_none() {
        builder.restore_testing_plan_name = Some(Default::default())
    }
    if builder.restore_testing_selection_name.is_none() {
        builder.restore_testing_selection_name = Some(Default::default())
    }
    builder
}

pub(crate) fn backup_rule_correct_errors(mut builder: crate::types::builders::BackupRuleBuilder) -> crate::types::builders::BackupRuleBuilder {
    if builder.rule_name.is_none() {
        builder.rule_name = Some(Default::default())
    }
    if builder.target_backup_vault_name.is_none() {
        builder.target_backup_vault_name = Some(Default::default())
    }
    builder
}

pub(crate) fn condition_correct_errors(mut builder: crate::types::builders::ConditionBuilder) -> crate::types::builders::ConditionBuilder {
    if builder.condition_type.is_none() {
        builder.condition_type = "no value was set".parse::<crate::types::ConditionType>().ok()
    }
    if builder.condition_key.is_none() {
        builder.condition_key = Some(Default::default())
    }
    if builder.condition_value.is_none() {
        builder.condition_value = Some(Default::default())
    }
    builder
}

pub(crate) fn key_value_correct_errors(mut builder: crate::types::builders::KeyValueBuilder) -> crate::types::builders::KeyValueBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn copy_action_correct_errors(mut builder: crate::types::builders::CopyActionBuilder) -> crate::types::builders::CopyActionBuilder {
    if builder.destination_backup_vault_arn.is_none() {
        builder.destination_backup_vault_arn = Some(Default::default())
    }
    builder
}
