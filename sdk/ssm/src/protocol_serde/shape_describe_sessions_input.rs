// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_sessions_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_sessions::DescribeSessionsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.state {
        object.key("State").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.filters {
        let mut array_5 = object.key("Filters").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_session_filter::ser_session_filter(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}
