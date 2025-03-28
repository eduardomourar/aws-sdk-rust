// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_neuron_device_core_info(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::NeuronDeviceCoreInfo, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::NeuronDeviceCoreInfo::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("count") /* Count com.amazonaws.ec2#NeuronDeviceCoreInfo$Count */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#NeuronDeviceCoreCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_count(var_1);
            }
            ,
            s if s.matches("version") /* Version com.amazonaws.ec2#NeuronDeviceCoreInfo$Version */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#NeuronDeviceCoreVersion`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_version(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
