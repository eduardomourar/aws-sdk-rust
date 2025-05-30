// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_instance_monitoring(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::InstanceMonitoring, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InstanceMonitoring::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceId") /* InstanceId com.amazonaws.ec2#InstanceMonitoring$InstanceId */ =>  {
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
            s if s.matches("monitoring") /* Monitoring com.amazonaws.ec2#InstanceMonitoring$Monitoring */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_monitoring::de_monitoring(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_monitoring(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
