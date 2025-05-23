// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_node_configuration_options_filter(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::NodeConfigurationOptionsFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Name");
    if let Some(var_2) = &input.name {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Operator");
    if let Some(var_4) = &input.operator {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Value");
    if let Some(var_6) = &input.values {
        let mut list_8 = scope_5.start_list(false, Some("item"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            entry_9.string(item_7);
        }
        list_8.finish();
    }
    Ok(())
}
