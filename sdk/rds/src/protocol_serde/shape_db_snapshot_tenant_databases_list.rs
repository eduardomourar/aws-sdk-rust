// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_db_snapshot_tenant_databases_list(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<::std::vec::Vec<crate::types::DbSnapshotTenantDatabase>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DBSnapshotTenantDatabase") /* member com.amazonaws.rds#DBSnapshotTenantDatabasesList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_db_snapshot_tenant_database::de_db_snapshot_tenant_database(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
