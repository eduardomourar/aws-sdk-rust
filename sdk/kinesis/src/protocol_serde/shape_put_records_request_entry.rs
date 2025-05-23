// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_records_request_entry(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PutRecordsRequestEntry,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Data").string_unchecked(&::aws_smithy_types::base64::encode(&input.data));
    }
    if let Some(var_1) = &input.explicit_hash_key {
        object.key("ExplicitHashKey").string(var_1.as_str());
    }
    {
        object.key("PartitionKey").string(input.partition_key.as_str());
    }
    Ok(())
}
