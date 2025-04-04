// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_vpc_block_public_access_exclusion(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::VpcBlockPublicAccessExclusion, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VpcBlockPublicAccessExclusion::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("exclusionId") /* ExclusionId com.amazonaws.ec2#VpcBlockPublicAccessExclusion$ExclusionId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_exclusion_id(var_1);
            }
            ,
            s if s.matches("internetGatewayExclusionMode") /* InternetGatewayExclusionMode com.amazonaws.ec2#VpcBlockPublicAccessExclusion$InternetGatewayExclusionMode */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::InternetGatewayExclusionMode, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InternetGatewayExclusionMode::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_internet_gateway_exclusion_mode(var_2);
            }
            ,
            s if s.matches("resourceArn") /* ResourceArn com.amazonaws.ec2#VpcBlockPublicAccessExclusion$ResourceArn */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_arn(var_3);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#VpcBlockPublicAccessExclusion$State */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::VpcBlockPublicAccessExclusionState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::VpcBlockPublicAccessExclusionState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_4);
            }
            ,
            s if s.matches("reason") /* Reason com.amazonaws.ec2#VpcBlockPublicAccessExclusion$Reason */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_reason(var_5);
            }
            ,
            s if s.matches("creationTimestamp") /* CreationTimestamp com.amazonaws.ec2#VpcBlockPublicAccessExclusion$CreationTimestamp */ =>  {
                let var_6 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_creation_timestamp(var_6);
            }
            ,
            s if s.matches("lastUpdateTimestamp") /* LastUpdateTimestamp com.amazonaws.ec2#VpcBlockPublicAccessExclusion$LastUpdateTimestamp */ =>  {
                let var_7 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_last_update_timestamp(var_7);
            }
            ,
            s if s.matches("deletionTimestamp") /* DeletionTimestamp com.amazonaws.ec2#VpcBlockPublicAccessExclusion$DeletionTimestamp */ =>  {
                let var_8 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_deletion_timestamp(var_8);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#VpcBlockPublicAccessExclusion$Tags */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
