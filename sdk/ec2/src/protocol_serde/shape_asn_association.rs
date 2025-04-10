// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_asn_association(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::AsnAssociation, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AsnAssociation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("asn") /* Asn com.amazonaws.ec2#AsnAssociation$Asn */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_asn(var_1);
            }
            ,
            s if s.matches("cidr") /* Cidr com.amazonaws.ec2#AsnAssociation$Cidr */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cidr(var_2);
            }
            ,
            s if s.matches("statusMessage") /* StatusMessage com.amazonaws.ec2#AsnAssociation$StatusMessage */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_message(var_3);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#AsnAssociation$State */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::AsnAssociationState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::AsnAssociationState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
