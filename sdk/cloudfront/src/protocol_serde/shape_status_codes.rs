// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_status_codes(
    input: &crate::types::StatusCodes,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    {
        let mut inner_writer = scope.start_el("Quantity").finish();
        inner_writer.data(::aws_smithy_types::primitive::Encoder::from(input.quantity).encode());
    }
    {
        let mut inner_writer = scope.start_el("Items").finish();
        for list_item_1 in &input.items {
            {
                let mut inner_writer = inner_writer.start_el("StatusCode").finish();
                inner_writer.data(::aws_smithy_types::primitive::Encoder::from(*list_item_1).encode());
            }
        }
    }
    scope.finish();
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_status_codes(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::StatusCodes, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::StatusCodes::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Quantity") /* Quantity com.amazonaws.cloudfront#StatusCodes$Quantity */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudfront#integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_quantity(var_2);
            }
            ,
            s if s.matches("Items") /* Items com.amazonaws.cloudfront#StatusCodes$Items */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_status_code_list::de_status_code_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_items(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::status_codes_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
