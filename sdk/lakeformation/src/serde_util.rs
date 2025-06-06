// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn get_query_state_output_output_correct_errors(
    mut builder: crate::operation::get_query_state::builders::GetQueryStateOutputBuilder,
) -> crate::operation::get_query_state::builders::GetQueryStateOutputBuilder {
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::QueryStateString>().ok()
    }
    builder
}

pub(crate) fn get_work_units_output_output_correct_errors(
    mut builder: crate::operation::get_work_units::builders::GetWorkUnitsOutputBuilder,
) -> crate::operation::get_work_units::builders::GetWorkUnitsOutputBuilder {
    if builder.query_id.is_none() {
        builder.query_id = Some(Default::default())
    }
    if builder.work_unit_ranges.is_none() {
        builder.work_unit_ranges = Some(Default::default())
    }
    builder
}

pub(crate) fn start_query_planning_output_output_correct_errors(
    mut builder: crate::operation::start_query_planning::builders::StartQueryPlanningOutputBuilder,
) -> crate::operation::start_query_planning::builders::StartQueryPlanningOutputBuilder {
    if builder.query_id.is_none() {
        builder.query_id = Some(Default::default())
    }
    builder
}

pub(crate) fn data_cells_filter_correct_errors(
    mut builder: crate::types::builders::DataCellsFilterBuilder,
) -> crate::types::builders::DataCellsFilterBuilder {
    if builder.table_catalog_id.is_none() {
        builder.table_catalog_id = Some(Default::default())
    }
    if builder.database_name.is_none() {
        builder.database_name = Some(Default::default())
    }
    if builder.table_name.is_none() {
        builder.table_name = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn external_filtering_configuration_correct_errors(
    mut builder: crate::types::builders::ExternalFilteringConfigurationBuilder,
) -> crate::types::builders::ExternalFilteringConfigurationBuilder {
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::EnableStatus>().ok()
    }
    if builder.authorized_targets.is_none() {
        builder.authorized_targets = Some(Default::default())
    }
    builder
}

pub(crate) fn lf_tag_correct_errors(mut builder: crate::types::builders::LfTagBuilder) -> crate::types::builders::LfTagBuilder {
    if builder.tag_key.is_none() {
        builder.tag_key = Some(Default::default())
    }
    if builder.tag_values.is_none() {
        builder.tag_values = Some(Default::default())
    }
    builder
}

pub(crate) fn lf_tag_pair_correct_errors(mut builder: crate::types::builders::LfTagPairBuilder) -> crate::types::builders::LfTagPairBuilder {
    if builder.tag_key.is_none() {
        builder.tag_key = Some(Default::default())
    }
    if builder.tag_values.is_none() {
        builder.tag_values = Some(Default::default())
    }
    builder
}

pub(crate) fn work_unit_range_correct_errors(
    mut builder: crate::types::builders::WorkUnitRangeBuilder,
) -> crate::types::builders::WorkUnitRangeBuilder {
    if builder.work_unit_id_max.is_none() {
        builder.work_unit_id_max = Some(Default::default())
    }
    if builder.work_unit_id_min.is_none() {
        builder.work_unit_id_min = Some(Default::default())
    }
    if builder.work_unit_token.is_none() {
        builder.work_unit_token = Some(Default::default())
    }
    builder
}

pub(crate) fn batch_permissions_request_entry_correct_errors(
    mut builder: crate::types::builders::BatchPermissionsRequestEntryBuilder,
) -> crate::types::builders::BatchPermissionsRequestEntryBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    builder
}

pub(crate) fn database_resource_correct_errors(
    mut builder: crate::types::builders::DatabaseResourceBuilder,
) -> crate::types::builders::DatabaseResourceBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn table_resource_correct_errors(
    mut builder: crate::types::builders::TableResourceBuilder,
) -> crate::types::builders::TableResourceBuilder {
    if builder.database_name.is_none() {
        builder.database_name = Some(Default::default())
    }
    builder
}

pub(crate) fn data_location_resource_correct_errors(
    mut builder: crate::types::builders::DataLocationResourceBuilder,
) -> crate::types::builders::DataLocationResourceBuilder {
    if builder.resource_arn.is_none() {
        builder.resource_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn lf_tag_expression_resource_correct_errors(
    mut builder: crate::types::builders::LfTagExpressionResourceBuilder,
) -> crate::types::builders::LfTagExpressionResourceBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn lf_tag_key_resource_correct_errors(
    mut builder: crate::types::builders::LfTagKeyResourceBuilder,
) -> crate::types::builders::LfTagKeyResourceBuilder {
    if builder.tag_key.is_none() {
        builder.tag_key = Some(Default::default())
    }
    if builder.tag_values.is_none() {
        builder.tag_values = Some(Default::default())
    }
    builder
}

pub(crate) fn lf_tag_policy_resource_correct_errors(
    mut builder: crate::types::builders::LfTagPolicyResourceBuilder,
) -> crate::types::builders::LfTagPolicyResourceBuilder {
    if builder.resource_type.is_none() {
        builder.resource_type = "no value was set".parse::<crate::types::ResourceType>().ok()
    }
    builder
}

pub(crate) fn table_with_columns_resource_correct_errors(
    mut builder: crate::types::builders::TableWithColumnsResourceBuilder,
) -> crate::types::builders::TableWithColumnsResourceBuilder {
    if builder.database_name.is_none() {
        builder.database_name = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}
