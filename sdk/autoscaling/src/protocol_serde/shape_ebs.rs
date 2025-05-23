// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_ebs(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::Ebs,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SnapshotId");
    if let Some(var_2) = &input.snapshot_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VolumeSize");
    if let Some(var_4) = &input.volume_size {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("VolumeType");
    if let Some(var_6) = &input.volume_type {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DeleteOnTermination");
    if let Some(var_8) = &input.delete_on_termination {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Iops");
    if let Some(var_10) = &input.iops {
        scope_9.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_10).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Encrypted");
    if let Some(var_12) = &input.encrypted {
        scope_11.boolean(*var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("Throughput");
    if let Some(var_14) = &input.throughput {
        scope_13.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_ebs(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::Ebs, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Ebs::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("SnapshotId") /* SnapshotId com.amazonaws.autoscaling#Ebs$SnapshotId */ =>  {
                let var_15 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_snapshot_id(var_15);
            }
            ,
            s if s.matches("VolumeSize") /* VolumeSize com.amazonaws.autoscaling#Ebs$VolumeSize */ =>  {
                let var_16 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#BlockDeviceEbsVolumeSize`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_volume_size(var_16);
            }
            ,
            s if s.matches("VolumeType") /* VolumeType com.amazonaws.autoscaling#Ebs$VolumeType */ =>  {
                let var_17 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_volume_type(var_17);
            }
            ,
            s if s.matches("DeleteOnTermination") /* DeleteOnTermination com.amazonaws.autoscaling#Ebs$DeleteOnTermination */ =>  {
                let var_18 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.autoscaling#BlockDeviceEbsDeleteOnTermination`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_delete_on_termination(var_18);
            }
            ,
            s if s.matches("Iops") /* Iops com.amazonaws.autoscaling#Ebs$Iops */ =>  {
                let var_19 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#BlockDeviceEbsIops`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_iops(var_19);
            }
            ,
            s if s.matches("Encrypted") /* Encrypted com.amazonaws.autoscaling#Ebs$Encrypted */ =>  {
                let var_20 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.autoscaling#BlockDeviceEbsEncrypted`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_encrypted(var_20);
            }
            ,
            s if s.matches("Throughput") /* Throughput com.amazonaws.autoscaling#Ebs$Throughput */ =>  {
                let var_21 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#BlockDeviceEbsThroughput`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_throughput(var_21);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
