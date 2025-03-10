// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_loader_job_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_loader_job::StartLoaderJobInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.dependencies {
        let mut array_2 = object.key("dependencies").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.fail_on_error {
        object.key("failOnError").boolean(*var_4);
    }
    if let Some(var_5) = &input.format {
        object.key("format").string(var_5.as_str());
    }
    if let Some(var_6) = &input.iam_role_arn {
        object.key("iamRoleArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.mode {
        object.key("mode").string(var_7.as_str());
    }
    if let Some(var_8) = &input.parallelism {
        object.key("parallelism").string(var_8.as_str());
    }
    if let Some(var_9) = &input.parser_configuration {
        #[allow(unused_mut)]
        let mut object_10 = object.key("parserConfiguration").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.queue_request {
        object.key("queueRequest").boolean(*var_13);
    }
    if let Some(var_14) = &input.s3_bucket_region {
        object.key("region").string(var_14.as_str());
    }
    if let Some(var_15) = &input.source {
        object.key("source").string(var_15.as_str());
    }
    if let Some(var_16) = &input.update_single_cardinality_properties {
        object.key("updateSingleCardinalityProperties").boolean(*var_16);
    }
    if let Some(var_17) = &input.user_provided_edge_ids {
        object.key("userProvidedEdgeIds").boolean(*var_17);
    }
    Ok(())
}
