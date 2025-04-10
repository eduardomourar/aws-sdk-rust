// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_application_version_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_application_version::CreateApplicationVersionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.source_code_archive_url {
        object.key("sourceCodeArchiveUrl").string(var_1.as_str());
    }
    if let Some(var_2) = &input.source_code_url {
        object.key("sourceCodeUrl").string(var_2.as_str());
    }
    if let Some(var_3) = &input.template_body {
        object.key("templateBody").string(var_3.as_str());
    }
    if let Some(var_4) = &input.template_url {
        object.key("templateUrl").string(var_4.as_str());
    }
    Ok(())
}
