// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_source_table_feature_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::SourceTableFeatureDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SourceTableFeatureDetailsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "LocalSecondaryIndexes" => {
                            builder = builder.set_local_secondary_indexes(
                                crate::protocol_serde::shape_local_secondary_indexes::de_local_secondary_indexes(tokens)?,
                            );
                        }
                        "GlobalSecondaryIndexes" => {
                            builder = builder.set_global_secondary_indexes(
                                crate::protocol_serde::shape_global_secondary_indexes::de_global_secondary_indexes(tokens)?,
                            );
                        }
                        "StreamDescription" => {
                            builder =
                                builder.set_stream_description(crate::protocol_serde::shape_stream_specification::de_stream_specification(tokens)?);
                        }
                        "TimeToLiveDescription" => {
                            builder = builder.set_time_to_live_description(
                                crate::protocol_serde::shape_time_to_live_description::de_time_to_live_description(tokens)?,
                            );
                        }
                        "SSEDescription" => {
                            builder = builder.set_sse_description(crate::protocol_serde::shape_sse_description::de_sse_description(tokens)?);
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
