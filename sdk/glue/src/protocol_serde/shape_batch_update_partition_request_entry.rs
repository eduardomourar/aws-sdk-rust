// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_update_partition_request_entry(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::BatchUpdatePartitionRequestEntry,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("PartitionValueList").start_array();
        for item_2 in &input.partition_value_list {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    if let Some(var_3) = &input.partition_input {
        #[allow(unused_mut)]
        let mut object_4 = object.key("PartitionInput").start_object();
        crate::protocol_serde::shape_partition_input::ser_partition_input(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
