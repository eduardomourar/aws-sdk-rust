// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_add_prefix_list_entry(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::AddPrefixListEntry,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Cidr");
    if let Some(var_2) = &input.cidr {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Description");
    if let Some(var_4) = &input.description {
        scope_3.string(var_4);
    }
    Ok(())
}
