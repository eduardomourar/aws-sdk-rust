// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_service_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_service::UpdateServiceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.auth_type {
        object.key("authType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.certificate_arn {
        object.key("certificateArn").string(var_2.as_str());
    }
    Ok(())
}
