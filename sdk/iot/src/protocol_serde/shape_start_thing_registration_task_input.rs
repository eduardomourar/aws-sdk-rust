// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_thing_registration_task_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_thing_registration_task::StartThingRegistrationTaskInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.input_file_bucket {
        object.key("inputFileBucket").string(var_1.as_str());
    }
    if let Some(var_2) = &input.input_file_key {
        object.key("inputFileKey").string(var_2.as_str());
    }
    if let Some(var_3) = &input.role_arn {
        object.key("roleArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.template_body {
        object.key("templateBody").string(var_4.as_str());
    }
    Ok(())
}
