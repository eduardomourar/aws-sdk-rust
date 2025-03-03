// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_policies_input_input_input(
    input: &crate::operation::describe_policies::DescribePoliciesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribePolicies", "2011-01-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AutoScalingGroupName");
    if let Some(var_2) = &input.auto_scaling_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PolicyNames");
    if let Some(var_4) = &input.policy_names {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("PolicyTypes");
    if let Some(var_9) = &input.policy_types {
        let mut list_11 = scope_8.start_list(false, None);
        for item_10 in var_9 {
            #[allow(unused_mut)]
            let mut entry_12 = list_11.entry();
            entry_12.string(item_10);
        }
        list_11.finish();
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("NextToken");
    if let Some(var_14) = &input.next_token {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("MaxRecords");
    if let Some(var_16) = &input.max_records {
        scope_15.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
