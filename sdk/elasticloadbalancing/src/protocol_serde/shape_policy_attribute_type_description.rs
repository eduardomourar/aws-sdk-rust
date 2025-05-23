// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_policy_attribute_type_description(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::PolicyAttributeTypeDescription, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::PolicyAttributeTypeDescription::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AttributeName") /* AttributeName com.amazonaws.elasticloadbalancing#PolicyAttributeTypeDescription$AttributeName */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_attribute_name(var_1);
            }
            ,
            s if s.matches("AttributeType") /* AttributeType com.amazonaws.elasticloadbalancing#PolicyAttributeTypeDescription$AttributeType */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_attribute_type(var_2);
            }
            ,
            s if s.matches("Description") /* Description com.amazonaws.elasticloadbalancing#PolicyAttributeTypeDescription$Description */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_3);
            }
            ,
            s if s.matches("DefaultValue") /* DefaultValue com.amazonaws.elasticloadbalancing#PolicyAttributeTypeDescription$DefaultValue */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_default_value(var_4);
            }
            ,
            s if s.matches("Cardinality") /* Cardinality com.amazonaws.elasticloadbalancing#PolicyAttributeTypeDescription$Cardinality */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cardinality(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
