// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_purchase(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::Purchase, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Purchase::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("currencyCode") /* CurrencyCode com.amazonaws.ec2#Purchase$CurrencyCode */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::CurrencyCodeValues, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::CurrencyCodeValues::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_currency_code(var_1);
            }
            ,
            s if s.matches("duration") /* Duration com.amazonaws.ec2#Purchase$Duration */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_duration(var_2);
            }
            ,
            s if s.matches("hostIdSet") /* HostIdSet com.amazonaws.ec2#Purchase$HostIdSet */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_response_host_id_set::de_response_host_id_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_host_id_set(var_3);
            }
            ,
            s if s.matches("hostReservationId") /* HostReservationId com.amazonaws.ec2#Purchase$HostReservationId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_host_reservation_id(var_4);
            }
            ,
            s if s.matches("hourlyPrice") /* HourlyPrice com.amazonaws.ec2#Purchase$HourlyPrice */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_hourly_price(var_5);
            }
            ,
            s if s.matches("instanceFamily") /* InstanceFamily com.amazonaws.ec2#Purchase$InstanceFamily */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_family(var_6);
            }
            ,
            s if s.matches("paymentOption") /* PaymentOption com.amazonaws.ec2#Purchase$PaymentOption */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::PaymentOption, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::PaymentOption::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_payment_option(var_7);
            }
            ,
            s if s.matches("upfrontPrice") /* UpfrontPrice com.amazonaws.ec2#Purchase$UpfrontPrice */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_upfront_price(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
