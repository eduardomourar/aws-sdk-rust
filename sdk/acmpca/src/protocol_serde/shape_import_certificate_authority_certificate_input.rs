// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_certificate_authority_certificate_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::import_certificate_authority_certificate::ImportCertificateAuthorityCertificateInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.certificate {
        object.key("Certificate").string_unchecked(&::aws_smithy_types::base64::encode(var_2));
    }
    if let Some(var_3) = &input.certificate_chain {
        object
            .key("CertificateChain")
            .string_unchecked(&::aws_smithy_types::base64::encode(var_3));
    }
    Ok(())
}
