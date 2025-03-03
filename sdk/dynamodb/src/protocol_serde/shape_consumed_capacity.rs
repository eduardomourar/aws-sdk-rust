// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_consumed_capacity<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ConsumedCapacity>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ConsumedCapacityBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "TableName" => {
                            builder = builder.set_table_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CapacityUnits" => {
                            builder = builder.set_capacity_units(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()),
                            );
                        }
                        "ReadCapacityUnits" => {
                            builder = builder.set_read_capacity_units(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()),
                            );
                        }
                        "WriteCapacityUnits" => {
                            builder = builder.set_write_capacity_units(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()),
                            );
                        }
                        "Table" => {
                            builder = builder.set_table(crate::protocol_serde::shape_capacity::de_capacity(tokens)?);
                        }
                        "LocalSecondaryIndexes" => {
                            builder = builder.set_local_secondary_indexes(
                                crate::protocol_serde::shape_secondary_indexes_capacity_map::de_secondary_indexes_capacity_map(tokens)?,
                            );
                        }
                        "GlobalSecondaryIndexes" => {
                            builder = builder.set_global_secondary_indexes(
                                crate::protocol_serde::shape_secondary_indexes_capacity_map::de_secondary_indexes_capacity_map(tokens)?,
                            );
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
