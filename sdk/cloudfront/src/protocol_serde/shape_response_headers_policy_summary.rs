// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_response_headers_policy_summary(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ResponseHeadersPolicySummary, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ResponseHeadersPolicySummary::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Type") /* Type com.amazonaws.cloudfront#ResponseHeadersPolicySummary$Type */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::ResponseHeadersPolicyType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ResponseHeadersPolicyType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_1);
            }
            ,
            s if s.matches("ResponseHeadersPolicy") /* ResponseHeadersPolicy com.amazonaws.cloudfront#ResponseHeadersPolicySummary$ResponseHeadersPolicy */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_response_headers_policy::de_response_headers_policy(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_response_headers_policy(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::response_headers_policy_summary_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
