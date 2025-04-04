// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_outbound_additional_recipients(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::OutboundAdditionalRecipients,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cc_email_addresses {
        let mut array_2 = object.key("CcEmailAddresses").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_email_address_info::ser_email_address_info(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}
