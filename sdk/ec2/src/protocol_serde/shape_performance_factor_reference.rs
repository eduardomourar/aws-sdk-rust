// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_performance_factor_reference(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::PerformanceFactorReference,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("InstanceFamily");
    if let Some(var_2) = &input.instance_family {
        scope_1.string(var_2);
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_performance_factor_reference(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::PerformanceFactorReference, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::PerformanceFactorReference::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceFamily") /* InstanceFamily com.amazonaws.ec2#PerformanceFactorReference$InstanceFamily */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_family(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
