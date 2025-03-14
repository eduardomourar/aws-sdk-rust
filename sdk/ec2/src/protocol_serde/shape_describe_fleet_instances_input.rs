// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_fleet_instances_input_input_input(
    input: &crate::operation::describe_fleet_instances::DescribeFleetInstancesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeFleetInstances", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("MaxResults");
    if let Some(var_4) = &input.max_results {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("NextToken");
    if let Some(var_6) = &input.next_token {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("FleetId");
    if let Some(var_8) = &input.fleet_id {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Filter");
    if let Some(var_10) = &input.filters {
        if !var_10.is_empty() {
            let mut list_12 = scope_9.start_list(true, Some("Filter"));
            for item_11 in var_10 {
                #[allow(unused_mut)]
                let mut entry_13 = list_12.entry();
                crate::protocol_serde::shape_filter::ser_filter(entry_13, item_11)?;
            }
            list_12.finish();
        }
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
