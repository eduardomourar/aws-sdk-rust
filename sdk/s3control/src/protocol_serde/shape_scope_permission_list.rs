// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_scope_permission_list(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<::std::vec::Vec<crate::types::ScopePermission>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Permission") /* member com.amazonaws.s3control#ScopePermissionList$member */ =>  {
                out.push(
                    Result::<crate::types::ScopePermission, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                        crate::types::ScopePermission::from(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        )
                    )
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
