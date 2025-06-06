// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_e_tag_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("ETag");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_field_level_encryption_profile_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::FieldLevelEncryptionProfile>,
    crate::operation::create_field_level_encryption_profile::CreateFieldLevelEncryptionProfileError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_create_field_level_encryption_profile_output::de_field_level_encryption_profile(body)
                .map_err(crate::operation::create_field_level_encryption_profile::CreateFieldLevelEncryptionProfileError::unhandled)
        })
        .transpose()
}

pub(crate) fn de_location_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Location");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub fn de_field_level_encryption_profile(
    inp: &[u8],
) -> std::result::Result<crate::types::FieldLevelEncryptionProfile, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("FieldLevelEncryptionProfile")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected FieldLevelEncryptionProfile got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_field_level_encryption_profile::de_field_level_encryption_profile(&mut decoder)
}
