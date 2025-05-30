// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_compliance_details_by_config_rule_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_compliance_details_by_config_rule::GetComplianceDetailsByConfigRuleInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.config_rule_name {
        object.key("ConfigRuleName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.compliance_types {
        let mut array_3 = object.key("ComplianceTypes").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.next_token {
        object.key("NextToken").string(var_6.as_str());
    }
    Ok(())
}
