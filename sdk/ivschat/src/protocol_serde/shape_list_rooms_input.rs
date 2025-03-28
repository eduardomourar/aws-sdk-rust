// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_rooms_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_rooms::ListRoomsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.logging_configuration_identifier {
        object.key("loggingConfigurationIdentifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.message_review_handler_uri {
        object.key("messageReviewHandlerUri").string(var_3.as_str());
    }
    if let Some(var_4) = &input.name {
        object.key("name").string(var_4.as_str());
    }
    if let Some(var_5) = &input.next_token {
        object.key("nextToken").string(var_5.as_str());
    }
    Ok(())
}
