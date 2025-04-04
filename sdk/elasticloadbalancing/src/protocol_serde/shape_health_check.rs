// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_health_check(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::HealthCheck,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Target");
    {
        scope_1.string(&input.target);
    }
    #[allow(unused_mut)]
    let mut scope_2 = writer.prefix("Interval");
    {
        scope_2.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Timeout");
    {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.timeout).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_4 = writer.prefix("UnhealthyThreshold");
    {
        scope_4.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.unhealthy_threshold).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("HealthyThreshold");
    {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.healthy_threshold).into()),
        );
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_health_check(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::HealthCheck, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::HealthCheck::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Target") /* Target com.amazonaws.elasticloadbalancing#HealthCheck$Target */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_target(var_6);
            }
            ,
            s if s.matches("Interval") /* Interval com.amazonaws.elasticloadbalancing#HealthCheck$Interval */ =>  {
                let var_7 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.elasticloadbalancing#HealthCheckInterval`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_interval(var_7);
            }
            ,
            s if s.matches("Timeout") /* Timeout com.amazonaws.elasticloadbalancing#HealthCheck$Timeout */ =>  {
                let var_8 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.elasticloadbalancing#HealthCheckTimeout`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_timeout(var_8);
            }
            ,
            s if s.matches("UnhealthyThreshold") /* UnhealthyThreshold com.amazonaws.elasticloadbalancing#HealthCheck$UnhealthyThreshold */ =>  {
                let var_9 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.elasticloadbalancing#UnhealthyThreshold`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_unhealthy_threshold(var_9);
            }
            ,
            s if s.matches("HealthyThreshold") /* HealthyThreshold com.amazonaws.elasticloadbalancing#HealthCheck$HealthyThreshold */ =>  {
                let var_10 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.elasticloadbalancing#HealthyThreshold`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_healthy_threshold(var_10);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::health_check_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
