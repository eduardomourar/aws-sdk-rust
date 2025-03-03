// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_address_transfer(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::AddressTransfer, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AddressTransfer::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("publicIp") /* PublicIp com.amazonaws.ec2#AddressTransfer$PublicIp */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_public_ip(var_1);
            }
            ,
            s if s.matches("allocationId") /* AllocationId com.amazonaws.ec2#AddressTransfer$AllocationId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_allocation_id(var_2);
            }
            ,
            s if s.matches("transferAccountId") /* TransferAccountId com.amazonaws.ec2#AddressTransfer$TransferAccountId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transfer_account_id(var_3);
            }
            ,
            s if s.matches("transferOfferExpirationTimestamp") /* TransferOfferExpirationTimestamp com.amazonaws.ec2#AddressTransfer$TransferOfferExpirationTimestamp */ =>  {
                let var_4 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_transfer_offer_expiration_timestamp(var_4);
            }
            ,
            s if s.matches("transferOfferAcceptedTimestamp") /* TransferOfferAcceptedTimestamp com.amazonaws.ec2#AddressTransfer$TransferOfferAcceptedTimestamp */ =>  {
                let var_5 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_transfer_offer_accepted_timestamp(var_5);
            }
            ,
            s if s.matches("addressTransferStatus") /* AddressTransferStatus com.amazonaws.ec2#AddressTransfer$AddressTransferStatus */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::AddressTransferStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::AddressTransferStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_address_transfer_status(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
