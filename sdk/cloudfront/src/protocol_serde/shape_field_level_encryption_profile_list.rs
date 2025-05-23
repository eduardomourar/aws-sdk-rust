// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_field_level_encryption_profile_list(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::FieldLevelEncryptionProfileList, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::FieldLevelEncryptionProfileList::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("NextMarker") /* NextMarker com.amazonaws.cloudfront#FieldLevelEncryptionProfileList$NextMarker */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_marker(var_1);
            }
            ,
            s if s.matches("MaxItems") /* MaxItems com.amazonaws.cloudfront#FieldLevelEncryptionProfileList$MaxItems */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudfront#integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_items(var_2);
            }
            ,
            s if s.matches("Quantity") /* Quantity com.amazonaws.cloudfront#FieldLevelEncryptionProfileList$Quantity */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudfront#integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_quantity(var_3);
            }
            ,
            s if s.matches("Items") /* Items com.amazonaws.cloudfront#FieldLevelEncryptionProfileList$Items */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_field_level_encryption_profile_summary_list::de_field_level_encryption_profile_summary_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_items(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::field_level_encryption_profile_list_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
