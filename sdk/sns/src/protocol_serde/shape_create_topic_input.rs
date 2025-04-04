// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_topic_input_input_input(
    input: &crate::operation::create_topic::CreateTopicInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateTopic", "2010-03-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Name");
    if let Some(var_2) = &input.name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Attributes");
    if let Some(var_4) = &input.attributes {
        let mut map_5 = scope_3.start_map(false, "key", "value");
        for (key_6, value_7) in var_4 {
            #[allow(unused_mut)]
            let mut entry_8 = map_5.entry(key_6);
            {
                entry_8.string(value_7);
            }
        }
        map_5.finish();
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Tags");
    if let Some(var_10) = &input.tags {
        let mut list_12 = scope_9.start_list(false, None);
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_13, item_11)?;
        }
        list_12.finish();
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("DataProtectionPolicy");
    if let Some(var_15) = &input.data_protection_policy {
        scope_14.string(var_15);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
