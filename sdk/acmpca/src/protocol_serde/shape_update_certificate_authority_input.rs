// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_certificate_authority_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_certificate_authority::UpdateCertificateAuthorityInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.revocation_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("RevocationConfiguration").start_object();
        crate::protocol_serde::shape_revocation_configuration::ser_revocation_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.status {
        object.key("Status").string(var_4.as_str());
    }
    Ok(())
}
