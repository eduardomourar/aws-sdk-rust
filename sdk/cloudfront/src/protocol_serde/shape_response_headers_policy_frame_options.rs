// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_response_headers_policy_frame_options(
    input: &crate::types::ResponseHeadersPolicyFrameOptions,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    {
        let mut inner_writer = scope.start_el("Override").finish();
        inner_writer.data(::aws_smithy_types::primitive::Encoder::from(input.r#override).encode());
    }
    {
        let mut inner_writer = scope.start_el("FrameOption").finish();
        inner_writer.data(input.frame_option.as_str());
    }
    scope.finish();
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_response_headers_policy_frame_options(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ResponseHeadersPolicyFrameOptions, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ResponseHeadersPolicyFrameOptions::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Override") /* Override com.amazonaws.cloudfront#ResponseHeadersPolicyFrameOptions$Override */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudfront#boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_override(var_1);
            }
            ,
            s if s.matches("FrameOption") /* FrameOption com.amazonaws.cloudfront#ResponseHeadersPolicyFrameOptions$FrameOption */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::FrameOptionsList, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::FrameOptionsList::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_frame_option(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::response_headers_policy_frame_options_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
