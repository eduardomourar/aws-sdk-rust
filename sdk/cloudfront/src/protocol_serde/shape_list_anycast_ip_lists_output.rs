// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_anycast_ip_lists_payload(
    body: &[u8],
) -> std::result::Result<::std::option::Option<crate::types::AnycastIpListCollection>, crate::operation::list_anycast_ip_lists::ListAnycastIpListsError>
{
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_list_anycast_ip_lists_output::de_anycast_ip_lists(body)
                .map_err(crate::operation::list_anycast_ip_lists::ListAnycastIpListsError::unhandled)
        })
        .transpose()
}

pub fn de_anycast_ip_lists(inp: &[u8]) -> std::result::Result<crate::types::AnycastIpListCollection, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("AnycastIpListCollection")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected AnycastIpListCollection got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_anycast_ip_list_collection::de_anycast_ip_list_collection(&mut decoder)
}
