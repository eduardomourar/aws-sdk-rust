// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_manage_propertygraph_statistics_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::manage_propertygraph_statistics::ManagePropertygraphStatisticsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.mode {
        object.key("mode").string(var_1.as_str());
    }
    Ok(())
}
