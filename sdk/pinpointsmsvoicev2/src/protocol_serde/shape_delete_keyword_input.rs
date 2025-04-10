// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_keyword_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_keyword::DeleteKeywordInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.origination_identity {
        object.key("OriginationIdentity").string(var_1.as_str());
    }
    if let Some(var_2) = &input.keyword {
        object.key("Keyword").string(var_2.as_str());
    }
    Ok(())
}
