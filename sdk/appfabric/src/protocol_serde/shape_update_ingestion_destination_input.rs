// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_ingestion_destination_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_ingestion_destination::UpdateIngestionDestinationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.destination_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("destinationConfiguration").start_object();
        crate::protocol_serde::shape_destination_configuration::ser_destination_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
