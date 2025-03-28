// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_query_objects_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::query_objects::QueryObjectsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.pipeline_id {
        object.key("pipelineId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.query {
        #[allow(unused_mut)]
        let mut object_3 = object.key("query").start_object();
        crate::protocol_serde::shape_query::ser_query(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.sphere {
        object.key("sphere").string(var_4.as_str());
    }
    if let Some(var_5) = &input.marker {
        object.key("marker").string(var_5.as_str());
    }
    if let Some(var_6) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    Ok(())
}
