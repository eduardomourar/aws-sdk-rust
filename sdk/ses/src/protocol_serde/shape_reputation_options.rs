// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_reputation_options(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ReputationOptions, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ReputationOptions::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("SendingEnabled") /* SendingEnabled com.amazonaws.ses#ReputationOptions$SendingEnabled */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ses#Enabled`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_sending_enabled(var_1);
            }
            ,
            s if s.matches("ReputationMetricsEnabled") /* ReputationMetricsEnabled com.amazonaws.ses#ReputationOptions$ReputationMetricsEnabled */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ses#Enabled`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_reputation_metrics_enabled(var_2);
            }
            ,
            s if s.matches("LastFreshStart") /* LastFreshStart com.amazonaws.ses#ReputationOptions$LastFreshStart */ =>  {
                let var_3 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ses#LastFreshStart`)"))
                        ?
                    )
                ;
                builder = builder.set_last_fresh_start(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
