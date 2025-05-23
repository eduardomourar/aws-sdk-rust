// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_configured_table_analysis_rule_policy_v1(
    object_1: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ConfiguredTableAnalysisRulePolicyV1,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::ConfiguredTableAnalysisRulePolicyV1::List(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_1.key("list").start_object();
            crate::protocol_serde::shape_analysis_rule_list::ser_analysis_rule_list(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::ConfiguredTableAnalysisRulePolicyV1::Aggregation(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_1.key("aggregation").start_object();
            crate::protocol_serde::shape_analysis_rule_aggregation::ser_analysis_rule_aggregation(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::ConfiguredTableAnalysisRulePolicyV1::Custom(inner) => {
            #[allow(unused_mut)]
            let mut object_3 = object_1.key("custom").start_object();
            crate::protocol_serde::shape_analysis_rule_custom::ser_analysis_rule_custom(&mut object_3, inner)?;
            object_3.finish();
        }
        crate::types::ConfiguredTableAnalysisRulePolicyV1::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "ConfiguredTableAnalysisRulePolicyV1",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_configured_table_analysis_rule_policy_v1<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ConfiguredTableAnalysisRulePolicyV1>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
                        tokens.peek()
                    {
                        let _ = tokens.next().expect("peek returned a token")?;
                        continue;
                    }
                    let key = key.to_unescaped()?;
                    if key == "__type" {
                        ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                        continue;
                    }
                    if variant.is_some() {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            "encountered mixed variants in union",
                        ));
                    }
                    variant = match key.as_ref() {
                        "list" => Some(crate::types::ConfiguredTableAnalysisRulePolicyV1::List(
                            crate::protocol_serde::shape_analysis_rule_list::de_analysis_rule_list(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'list' cannot be null"))?,
                        )),
                        "aggregation" => Some(crate::types::ConfiguredTableAnalysisRulePolicyV1::Aggregation(
                            crate::protocol_serde::shape_analysis_rule_aggregation::de_analysis_rule_aggregation(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'aggregation' cannot be null")
                            })?,
                        )),
                        "custom" => Some(crate::types::ConfiguredTableAnalysisRulePolicyV1::Custom(
                            crate::protocol_serde::shape_analysis_rule_custom::de_analysis_rule_custom(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'custom' cannot be null")
                            })?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::ConfiguredTableAnalysisRulePolicyV1::Unknown)
                        }
                    };
                }
                other => {
                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )))
                }
            }
        },
        _ => {
            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ))
        }
    }
    if variant.is_none() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "Union did not contain a valid variant.",
        ));
    }
    Ok(variant)
}
