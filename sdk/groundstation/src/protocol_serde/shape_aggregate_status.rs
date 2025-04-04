// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aggregate_status(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AggregateStatus,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("status").string(input.status.as_str());
    }
    if let Some(var_1) = &input.signature_map {
        #[allow(unused_mut)]
        let mut object_2 = object.key("signatureMap").start_object();
        for (key_3, value_4) in var_1 {
            {
                object_2.key(key_3.as_str()).boolean(*value_4);
            }
        }
        object_2.finish();
    }
    Ok(())
}
