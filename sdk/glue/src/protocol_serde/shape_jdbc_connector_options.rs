// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_jdbc_connector_options(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::JdbcConnectorOptions,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.filter_predicate {
        object.key("FilterPredicate").string(var_1.as_str());
    }
    if let Some(var_2) = &input.partition_column {
        object.key("PartitionColumn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.lower_bound {
        object.key("LowerBound").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.upper_bound {
        object.key("UpperBound").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.num_partitions {
        object.key("NumPartitions").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.job_bookmark_keys {
        let mut array_7 = object.key("JobBookmarkKeys").start_array();
        for item_8 in var_6 {
            {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.job_bookmark_keys_sort_order {
        object.key("JobBookmarkKeysSortOrder").string(var_9.as_str());
    }
    if let Some(var_10) = &input.data_type_mapping {
        #[allow(unused_mut)]
        let mut object_11 = object.key("DataTypeMapping").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    Ok(())
}

pub(crate) fn de_jdbc_connector_options<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::JdbcConnectorOptions>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::JdbcConnectorOptionsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "FilterPredicate" => {
                            builder = builder.set_filter_predicate(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PartitionColumn" => {
                            builder = builder.set_partition_column(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "LowerBound" => {
                            builder = builder.set_lower_bound(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "UpperBound" => {
                            builder = builder.set_upper_bound(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "NumPartitions" => {
                            builder = builder.set_num_partitions(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "JobBookmarkKeys" => {
                            builder = builder.set_job_bookmark_keys(
                                crate::protocol_serde::shape_enclosed_in_string_properties::de_enclosed_in_string_properties(tokens)?,
                            );
                        }
                        "JobBookmarkKeysSortOrder" => {
                            builder = builder.set_job_bookmark_keys_sort_order(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "DataTypeMapping" => {
                            builder = builder
                                .set_data_type_mapping(crate::protocol_serde::shape_jdbc_data_type_mapping::de_jdbc_data_type_mapping(tokens)?);
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
