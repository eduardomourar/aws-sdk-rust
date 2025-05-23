// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_successful_queued_purchase_deletion(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::SuccessfulQueuedPurchaseDeletion, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::SuccessfulQueuedPurchaseDeletion::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("reservedInstancesId") /* ReservedInstancesId com.amazonaws.ec2#SuccessfulQueuedPurchaseDeletion$ReservedInstancesId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_reserved_instances_id(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
