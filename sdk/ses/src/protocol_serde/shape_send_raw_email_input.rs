// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_raw_email_input_input_input(
    input: &crate::operation::send_raw_email::SendRawEmailInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "SendRawEmail", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Source");
    if let Some(var_2) = &input.source {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Destinations");
    if let Some(var_4) = &input.destinations {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("RawMessage");
    if let Some(var_9) = &input.raw_message {
        crate::protocol_serde::shape_raw_message::ser_raw_message(scope_8, var_9)?;
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("FromArn");
    if let Some(var_11) = &input.from_arn {
        scope_10.string(var_11);
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("SourceArn");
    if let Some(var_13) = &input.source_arn {
        scope_12.string(var_13);
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("ReturnPathArn");
    if let Some(var_15) = &input.return_path_arn {
        scope_14.string(var_15);
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("Tags");
    if let Some(var_17) = &input.tags {
        let mut list_19 = scope_16.start_list(false, None);
        for item_18 in var_17 {
            #[allow(unused_mut)]
            let mut entry_20 = list_19.entry();
            crate::protocol_serde::shape_message_tag::ser_message_tag(entry_20, item_18)?;
        }
        list_19.finish();
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("ConfigurationSetName");
    if let Some(var_22) = &input.configuration_set_name {
        scope_21.string(var_22);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
