// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_policy_attribute_descriptions(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<::std::vec::Vec<crate::types::PolicyAttributeDescription>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("member") /* member com.amazonaws.elasticloadbalancing#PolicyAttributeDescriptions$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_policy_attribute_description::de_policy_attribute_description(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
