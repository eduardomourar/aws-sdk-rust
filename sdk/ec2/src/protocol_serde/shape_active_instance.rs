// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_active_instance(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ActiveInstance, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ActiveInstance::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceId") /* InstanceId com.amazonaws.ec2#ActiveInstance$InstanceId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_id(var_1);
            }
            ,
            s if s.matches("instanceType") /* InstanceType com.amazonaws.ec2#ActiveInstance$InstanceType */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_type(var_2);
            }
            ,
            s if s.matches("spotInstanceRequestId") /* SpotInstanceRequestId com.amazonaws.ec2#ActiveInstance$SpotInstanceRequestId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_spot_instance_request_id(var_3);
            }
            ,
            s if s.matches("instanceHealth") /* InstanceHealth com.amazonaws.ec2#ActiveInstance$InstanceHealth */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::InstanceHealthStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InstanceHealthStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_health(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
