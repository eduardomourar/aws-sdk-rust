// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_batch_segment_job_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_batch_segment_job::CreateBatchSegmentJobInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.job_name {
        object.key("jobName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.solution_version_arn {
        object.key("solutionVersionArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.filter_arn {
        object.key("filterArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.num_results {
        object.key("numResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.job_input {
        #[allow(unused_mut)]
        let mut object_6 = object.key("jobInput").start_object();
        crate::protocol_serde::shape_batch_segment_job_input::ser_batch_segment_job_input(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.job_output {
        #[allow(unused_mut)]
        let mut object_8 = object.key("jobOutput").start_object();
        crate::protocol_serde::shape_batch_segment_job_output::ser_batch_segment_job_output(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.role_arn {
        object.key("roleArn").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags {
        let mut array_11 = object.key("tags").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
