// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_duration_range(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DurationRange,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.min_seconds {
        object.key("minSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.max_seconds {
        object.key("maxSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    Ok(())
}
