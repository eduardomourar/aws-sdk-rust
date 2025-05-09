// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn distribution_configuration_correct_errors(
    mut builder: crate::types::builders::DistributionConfigurationBuilder,
) -> crate::types::builders::DistributionConfigurationBuilder {
    if builder.timeout_minutes.is_none() {
        builder.timeout_minutes = Some(Default::default())
    }
    builder
}

pub(crate) fn target_container_repository_correct_errors(
    mut builder: crate::types::builders::TargetContainerRepositoryBuilder,
) -> crate::types::builders::TargetContainerRepositoryBuilder {
    if builder.service.is_none() {
        builder.service = "no value was set".parse::<crate::types::ContainerRepositoryService>().ok()
    }
    if builder.repository_name.is_none() {
        builder.repository_name = Some(Default::default())
    }
    builder
}

pub(crate) fn component_configuration_correct_errors(
    mut builder: crate::types::builders::ComponentConfigurationBuilder,
) -> crate::types::builders::ComponentConfigurationBuilder {
    if builder.component_arn.is_none() {
        builder.component_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn component_parameter_detail_correct_errors(
    mut builder: crate::types::builders::ComponentParameterDetailBuilder,
) -> crate::types::builders::ComponentParameterDetailBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = Some(Default::default())
    }
    builder
}

pub(crate) fn distribution_correct_errors(mut builder: crate::types::builders::DistributionBuilder) -> crate::types::builders::DistributionBuilder {
    if builder.region.is_none() {
        builder.region = Some(Default::default())
    }
    builder
}

pub(crate) fn lifecycle_policy_detail_correct_errors(
    mut builder: crate::types::builders::LifecyclePolicyDetailBuilder,
) -> crate::types::builders::LifecyclePolicyDetailBuilder {
    if builder.action.is_none() {
        builder.action = {
            let builder = crate::types::builders::LifecyclePolicyDetailActionBuilder::default();
            crate::serde_util::lifecycle_policy_detail_action_correct_errors(builder).build().ok()
        }
    }
    if builder.filter.is_none() {
        builder.filter = {
            let builder = crate::types::builders::LifecyclePolicyDetailFilterBuilder::default();
            crate::serde_util::lifecycle_policy_detail_filter_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn package_vulnerability_details_correct_errors(
    mut builder: crate::types::builders::PackageVulnerabilityDetailsBuilder,
) -> crate::types::builders::PackageVulnerabilityDetailsBuilder {
    if builder.vulnerability_id.is_none() {
        builder.vulnerability_id = Some(Default::default())
    }
    builder
}

pub(crate) fn product_code_list_item_correct_errors(
    mut builder: crate::types::builders::ProductCodeListItemBuilder,
) -> crate::types::builders::ProductCodeListItemBuilder {
    if builder.product_code_id.is_none() {
        builder.product_code_id = Some(Default::default())
    }
    if builder.product_code_type.is_none() {
        builder.product_code_type = "no value was set".parse::<crate::types::ProductCodeType>().ok()
    }
    builder
}

pub(crate) fn workflow_configuration_correct_errors(
    mut builder: crate::types::builders::WorkflowConfigurationBuilder,
) -> crate::types::builders::WorkflowConfigurationBuilder {
    if builder.workflow_arn.is_none() {
        builder.workflow_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn workflow_parameter_detail_correct_errors(
    mut builder: crate::types::builders::WorkflowParameterDetailBuilder,
) -> crate::types::builders::WorkflowParameterDetailBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = Some(Default::default())
    }
    builder
}

pub(crate) fn container_distribution_configuration_correct_errors(
    mut builder: crate::types::builders::ContainerDistributionConfigurationBuilder,
) -> crate::types::builders::ContainerDistributionConfigurationBuilder {
    if builder.target_repository.is_none() {
        builder.target_repository = {
            let builder = crate::types::builders::TargetContainerRepositoryBuilder::default();
            crate::serde_util::target_container_repository_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn lifecycle_policy_detail_action_correct_errors(
    mut builder: crate::types::builders::LifecyclePolicyDetailActionBuilder,
) -> crate::types::builders::LifecyclePolicyDetailActionBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::LifecyclePolicyDetailActionType>().ok()
    }
    builder
}

pub(crate) fn lifecycle_policy_detail_filter_correct_errors(
    mut builder: crate::types::builders::LifecyclePolicyDetailFilterBuilder,
) -> crate::types::builders::LifecyclePolicyDetailFilterBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::LifecyclePolicyDetailFilterType>().ok()
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn lifecycle_policy_resource_selection_recipe_correct_errors(
    mut builder: crate::types::builders::LifecyclePolicyResourceSelectionRecipeBuilder,
) -> crate::types::builders::LifecyclePolicyResourceSelectionRecipeBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.semantic_version.is_none() {
        builder.semantic_version = Some(Default::default())
    }
    builder
}

pub(crate) fn s3_export_configuration_correct_errors(
    mut builder: crate::types::builders::S3ExportConfigurationBuilder,
) -> crate::types::builders::S3ExportConfigurationBuilder {
    if builder.role_name.is_none() {
        builder.role_name = Some(Default::default())
    }
    if builder.disk_image_format.is_none() {
        builder.disk_image_format = "no value was set".parse::<crate::types::DiskImageFormat>().ok()
    }
    if builder.s3_bucket.is_none() {
        builder.s3_bucket = Some(Default::default())
    }
    builder
}

pub(crate) fn component_parameter_correct_errors(
    mut builder: crate::types::builders::ComponentParameterBuilder,
) -> crate::types::builders::ComponentParameterBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn fast_launch_configuration_correct_errors(
    mut builder: crate::types::builders::FastLaunchConfigurationBuilder,
) -> crate::types::builders::FastLaunchConfigurationBuilder {
    if builder.enabled.is_none() {
        builder.enabled = Some(Default::default())
    }
    builder
}

pub(crate) fn launch_template_configuration_correct_errors(
    mut builder: crate::types::builders::LaunchTemplateConfigurationBuilder,
) -> crate::types::builders::LaunchTemplateConfigurationBuilder {
    if builder.launch_template_id.is_none() {
        builder.launch_template_id = Some(Default::default())
    }
    builder
}

pub(crate) fn ssm_parameter_configuration_correct_errors(
    mut builder: crate::types::builders::SsmParameterConfigurationBuilder,
) -> crate::types::builders::SsmParameterConfigurationBuilder {
    if builder.parameter_name.is_none() {
        builder.parameter_name = Some(Default::default())
    }
    builder
}

pub(crate) fn workflow_parameter_correct_errors(
    mut builder: crate::types::builders::WorkflowParameterBuilder,
) -> crate::types::builders::WorkflowParameterBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn lifecycle_policy_detail_exclusion_rules_amis_last_launched_correct_errors(
    mut builder: crate::types::builders::LifecyclePolicyDetailExclusionRulesAmisLastLaunchedBuilder,
) -> crate::types::builders::LifecyclePolicyDetailExclusionRulesAmisLastLaunchedBuilder {
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    if builder.unit.is_none() {
        builder.unit = "no value was set".parse::<crate::types::LifecyclePolicyTimeUnit>().ok()
    }
    builder
}
