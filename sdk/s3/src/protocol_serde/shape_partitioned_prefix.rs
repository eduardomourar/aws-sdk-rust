// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_partitioned_prefix(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::PartitionedPrefix, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::PartitionedPrefix::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("PartitionDateSource") /* PartitionDateSource com.amazonaws.s3#PartitionedPrefix$PartitionDateSource */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::PartitionDateSource, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::PartitionDateSource::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_partition_date_source(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_partitioned_prefix(
    input: &crate::types::PartitionedPrefix,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_2) = &input.partition_date_source {
        let mut inner_writer = scope.start_el("PartitionDateSource").finish();
        inner_writer.data(var_2.as_str());
    }
    scope.finish();
    Ok(())
}
