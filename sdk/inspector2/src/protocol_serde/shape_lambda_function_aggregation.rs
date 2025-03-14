// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lambda_function_aggregation(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::LambdaFunctionAggregation,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.resource_ids {
        let mut array_2 = object.key("resourceIds").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_string_filter::ser_string_filter(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.function_names {
        let mut array_6 = object.key("functionNames").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_string_filter::ser_string_filter(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.runtimes {
        let mut array_10 = object.key("runtimes").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_string_filter::ser_string_filter(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.function_tags {
        let mut array_14 = object.key("functionTags").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_map_filter::ser_map_filter(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.sort_order {
        object.key("sortOrder").string(var_17.as_str());
    }
    if let Some(var_18) = &input.sort_by {
        object.key("sortBy").string(var_18.as_str());
    }
    Ok(())
}
