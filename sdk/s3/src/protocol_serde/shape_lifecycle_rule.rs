// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_lifecycle_rule(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::LifecycleRule, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LifecycleRule::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Expiration") /* Expiration com.amazonaws.s3#LifecycleRule$Expiration */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_lifecycle_expiration::de_lifecycle_expiration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_expiration(var_1);
            }
            ,
            s if s.matches("ID") /* ID com.amazonaws.s3#LifecycleRule$ID */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_2);
            }
            ,
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3#LifecycleRule$Prefix */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_3);
            }
            ,
            s if s.matches("Filter") /* Filter com.amazonaws.s3#LifecycleRule$Filter */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_lifecycle_rule_filter::de_lifecycle_rule_filter(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_filter(var_4);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.s3#LifecycleRule$Status */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::ExpirationStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ExpirationStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_5);
            }
            ,
            s if s.matches("Transition") /* Transitions com.amazonaws.s3#LifecycleRule$Transitions */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::vec::Vec::<crate::types::Transition>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_7 = builder.transitions.take().unwrap_or_default();
                            list_7.push(
                                crate::protocol_serde::shape_transition::de_transition(&mut tag)
                                ?
                            );
                            list_7
                        })
                        ?
                    )
                ;
                builder = builder.set_transitions(var_6);
            }
            ,
            s if s.matches("NoncurrentVersionTransition") /* NoncurrentVersionTransitions com.amazonaws.s3#LifecycleRule$NoncurrentVersionTransitions */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::vec::Vec::<crate::types::NoncurrentVersionTransition>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_9 = builder.noncurrent_version_transitions.take().unwrap_or_default();
                            list_9.push(
                                crate::protocol_serde::shape_noncurrent_version_transition::de_noncurrent_version_transition(&mut tag)
                                ?
                            );
                            list_9
                        })
                        ?
                    )
                ;
                builder = builder.set_noncurrent_version_transitions(var_8);
            }
            ,
            s if s.matches("NoncurrentVersionExpiration") /* NoncurrentVersionExpiration com.amazonaws.s3#LifecycleRule$NoncurrentVersionExpiration */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_noncurrent_version_expiration::de_noncurrent_version_expiration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_noncurrent_version_expiration(var_10);
            }
            ,
            s if s.matches("AbortIncompleteMultipartUpload") /* AbortIncompleteMultipartUpload com.amazonaws.s3#LifecycleRule$AbortIncompleteMultipartUpload */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_abort_incomplete_multipart_upload::de_abort_incomplete_multipart_upload(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_abort_incomplete_multipart_upload(var_11);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::lifecycle_rule_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}

pub fn ser_lifecycle_rule(
    input: &crate::types::LifecycleRule,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_12) = &input.expiration {
        let inner_writer = scope.start_el("Expiration");
        crate::protocol_serde::shape_lifecycle_expiration::ser_lifecycle_expiration(var_12, inner_writer)?
    }
    if let Some(var_13) = &input.id {
        let mut inner_writer = scope.start_el("ID").finish();
        inner_writer.data(var_13.as_str());
    }
    if let Some(var_14) = &input.prefix {
        let mut inner_writer = scope.start_el("Prefix").finish();
        inner_writer.data(var_14.as_str());
    }
    if let Some(var_15) = &input.filter {
        let inner_writer = scope.start_el("Filter");
        crate::protocol_serde::shape_lifecycle_rule_filter::ser_lifecycle_rule_filter(var_15, inner_writer)?
    }
    {
        let mut inner_writer = scope.start_el("Status").finish();
        inner_writer.data(input.status.as_str());
    }
    if let Some(var_16) = &input.transitions {
        for list_item_17 in var_16 {
            {
                let inner_writer = scope.start_el("Transition");
                crate::protocol_serde::shape_transition::ser_transition(list_item_17, inner_writer)?
            }
        }
    }
    if let Some(var_18) = &input.noncurrent_version_transitions {
        for list_item_19 in var_18 {
            {
                let inner_writer = scope.start_el("NoncurrentVersionTransition");
                crate::protocol_serde::shape_noncurrent_version_transition::ser_noncurrent_version_transition(list_item_19, inner_writer)?
            }
        }
    }
    if let Some(var_20) = &input.noncurrent_version_expiration {
        let inner_writer = scope.start_el("NoncurrentVersionExpiration");
        crate::protocol_serde::shape_noncurrent_version_expiration::ser_noncurrent_version_expiration(var_20, inner_writer)?
    }
    if let Some(var_21) = &input.abort_incomplete_multipart_upload {
        let inner_writer = scope.start_el("AbortIncompleteMultipartUpload");
        crate::protocol_serde::shape_abort_incomplete_multipart_upload::ser_abort_incomplete_multipart_upload(var_21, inner_writer)?
    }
    scope.finish();
    Ok(())
}
