// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_ml_configuration_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_ml_configuration::PutMlConfigurationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.default_output_location {
        #[allow(unused_mut)]
        let mut object_2 = object.key("defaultOutputLocation").start_object();
        crate::protocol_serde::shape_ml_output_configuration::ser_ml_output_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
