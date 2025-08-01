// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_query_vectors_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::query_vectors::QueryVectorsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.filter {
        object.key("filter").document(var_1);
    }
    if let Some(var_2) = &input.index_arn {
        object.key("indexArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.index_name {
        object.key("indexName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.query_vector {
        #[allow(unused_mut)]
        let mut object_5 = object.key("queryVector").start_object();
        crate::protocol_serde::shape_vector_data::ser_vector_data(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.return_distance {
        object.key("returnDistance").boolean(*var_6);
    }
    if let Some(var_7) = &input.return_metadata {
        object.key("returnMetadata").boolean(*var_7);
    }
    if let Some(var_8) = &input.top_k {
        object.key("topK").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.vector_bucket_name {
        object.key("vectorBucketName").string(var_9.as_str());
    }
    Ok(())
}
