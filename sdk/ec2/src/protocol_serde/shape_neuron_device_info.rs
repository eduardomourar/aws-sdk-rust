// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_neuron_device_info(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::NeuronDeviceInfo, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::NeuronDeviceInfo::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("count") /* Count com.amazonaws.ec2#NeuronDeviceInfo$Count */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#NeuronDeviceCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_count(var_1);
            }
            ,
            s if s.matches("name") /* Name com.amazonaws.ec2#NeuronDeviceInfo$Name */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_2);
            }
            ,
            s if s.matches("coreInfo") /* CoreInfo com.amazonaws.ec2#NeuronDeviceInfo$CoreInfo */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_neuron_device_core_info::de_neuron_device_core_info(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_core_info(var_3);
            }
            ,
            s if s.matches("memoryInfo") /* MemoryInfo com.amazonaws.ec2#NeuronDeviceInfo$MemoryInfo */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_neuron_device_memory_info::de_neuron_device_memory_info(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_memory_info(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
