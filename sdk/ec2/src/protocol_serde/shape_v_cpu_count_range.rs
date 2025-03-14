// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_v_cpu_count_range(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::VCpuCountRange,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Min");
    if let Some(var_2) = &input.min {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Max");
    if let Some(var_4) = &input.max {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_v_cpu_count_range(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::VCpuCountRange, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VCpuCountRange::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("min") /* Min com.amazonaws.ec2#VCpuCountRange$Min */ =>  {
                let var_5 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_min(var_5);
            }
            ,
            s if s.matches("max") /* Max com.amazonaws.ec2#VCpuCountRange$Max */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
