// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_sample_data_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_sample_data::GetSampleDataInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.s3_source_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("S3SourceConfig").start_object();
        crate::protocol_serde::shape_sample_data_s3_source_config::ser_sample_data_s3_source_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
