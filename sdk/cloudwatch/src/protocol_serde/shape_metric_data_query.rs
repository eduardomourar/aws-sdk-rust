// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_metric_data_query(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::MetricDataQuery,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Id");
    if let Some(var_2) = &input.id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("MetricStat");
    if let Some(var_4) = &input.metric_stat {
        crate::protocol_serde::shape_metric_stat::ser_metric_stat(scope_3, var_4)?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Expression");
    if let Some(var_6) = &input.expression {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Label");
    if let Some(var_8) = &input.label {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("ReturnData");
    if let Some(var_10) = &input.return_data {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Period");
    if let Some(var_12) = &input.period {
        scope_11.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("AccountId");
    if let Some(var_14) = &input.account_id {
        scope_13.string(var_14);
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_metric_data_query(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::MetricDataQuery, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::MetricDataQuery::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.cloudwatch#MetricDataQuery$Id */ =>  {
                let var_15 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_15);
            }
            ,
            s if s.matches("MetricStat") /* MetricStat com.amazonaws.cloudwatch#MetricDataQuery$MetricStat */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_metric_stat::de_metric_stat(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_metric_stat(var_16);
            }
            ,
            s if s.matches("Expression") /* Expression com.amazonaws.cloudwatch#MetricDataQuery$Expression */ =>  {
                let var_17 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_expression(var_17);
            }
            ,
            s if s.matches("Label") /* Label com.amazonaws.cloudwatch#MetricDataQuery$Label */ =>  {
                let var_18 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_label(var_18);
            }
            ,
            s if s.matches("ReturnData") /* ReturnData com.amazonaws.cloudwatch#MetricDataQuery$ReturnData */ =>  {
                let var_19 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudwatch#ReturnData`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_return_data(var_19);
            }
            ,
            s if s.matches("Period") /* Period com.amazonaws.cloudwatch#MetricDataQuery$Period */ =>  {
                let var_20 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudwatch#Period`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_period(var_20);
            }
            ,
            s if s.matches("AccountId") /* AccountId com.amazonaws.cloudwatch#MetricDataQuery$AccountId */ =>  {
                let var_21 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_account_id(var_21);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::metric_data_query_correct_errors(builder).build())
}
