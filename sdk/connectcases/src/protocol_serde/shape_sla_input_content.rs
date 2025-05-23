// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sla_input_content(
    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SlaInputContent,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::SlaInputContent::SlaInputConfiguration(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_4.key("slaInputConfiguration").start_object();
            crate::protocol_serde::shape_sla_input_configuration::ser_sla_input_configuration(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::SlaInputContent::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "SlaInputContent",
            ))
        }
    }
    Ok(())
}
