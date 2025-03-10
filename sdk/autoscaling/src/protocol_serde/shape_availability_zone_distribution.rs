// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_availability_zone_distribution(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::AvailabilityZoneDistribution,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CapacityDistributionStrategy");
    if let Some(var_2) = &input.capacity_distribution_strategy {
        scope_1.string(var_2.as_str());
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_availability_zone_distribution(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::AvailabilityZoneDistribution, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AvailabilityZoneDistribution::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CapacityDistributionStrategy") /* CapacityDistributionStrategy com.amazonaws.autoscaling#AvailabilityZoneDistribution$CapacityDistributionStrategy */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::CapacityDistributionStrategy, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::CapacityDistributionStrategy::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_capacity_distribution_strategy(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
