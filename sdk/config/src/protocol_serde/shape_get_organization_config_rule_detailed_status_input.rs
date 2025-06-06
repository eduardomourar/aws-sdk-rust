// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_organization_config_rule_detailed_status_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.organization_config_rule_name {
        object.key("OrganizationConfigRuleName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filters {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Filters").start_object();
        crate::protocol_serde::shape_status_detail_filters::ser_status_detail_filters(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.next_token {
        object.key("NextToken").string(var_5.as_str());
    }
    Ok(())
}
