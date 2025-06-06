// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_shard_count_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_shard_count::UpdateShardCountInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.stream_name {
        object.key("StreamName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_shard_count {
        object.key("TargetShardCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.scaling_type {
        object.key("ScalingType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.stream_arn {
        object.key("StreamARN").string(var_4.as_str());
    }
    Ok(())
}
