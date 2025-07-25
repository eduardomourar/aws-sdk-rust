// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_initialization_status_details(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::InitializationStatusDetails, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InitializationStatusDetails::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("initializationType") /* InitializationType com.amazonaws.ec2#InitializationStatusDetails$InitializationType */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::InitializationType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InitializationType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_initialization_type(var_1);
            }
            ,
            s if s.matches("progress") /* Progress com.amazonaws.ec2#InitializationStatusDetails$Progress */ =>  {
                let var_2 =
                    Some(
                         {
                            <i64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.ec2#Long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_progress(var_2);
            }
            ,
            s if s.matches("estimatedTimeToCompleteInSeconds") /* EstimatedTimeToCompleteInSeconds com.amazonaws.ec2#InitializationStatusDetails$EstimatedTimeToCompleteInSeconds */ =>  {
                let var_3 =
                    Some(
                         {
                            <i64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.ec2#Long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_estimated_time_to_complete_in_seconds(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
