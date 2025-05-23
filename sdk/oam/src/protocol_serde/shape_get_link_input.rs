// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_link_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_link::GetLinkInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.identifier {
        object.key("Identifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.include_tags {
        object.key("IncludeTags").boolean(*var_2);
    }
    Ok(())
}
