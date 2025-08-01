// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_data_source_parameters(
    object_5: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DataSourceParameters,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::DataSourceParameters::AmazonElasticsearchParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_5.key("AmazonElasticsearchParameters").start_object();
            crate::protocol_serde::shape_amazon_elasticsearch_parameters::ser_amazon_elasticsearch_parameters(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::DataSourceParameters::AthenaParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_5.key("AthenaParameters").start_object();
            crate::protocol_serde::shape_athena_parameters::ser_athena_parameters(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::DataSourceParameters::AuroraParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_3 = object_5.key("AuroraParameters").start_object();
            crate::protocol_serde::shape_aurora_parameters::ser_aurora_parameters(&mut object_3, inner)?;
            object_3.finish();
        }
        crate::types::DataSourceParameters::AuroraPostgreSqlParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_4 = object_5.key("AuroraPostgreSqlParameters").start_object();
            crate::protocol_serde::shape_aurora_postgre_sql_parameters::ser_aurora_postgre_sql_parameters(&mut object_4, inner)?;
            object_4.finish();
        }
        crate::types::DataSourceParameters::AwsIotAnalyticsParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_5 = object_5.key("AwsIotAnalyticsParameters").start_object();
            crate::protocol_serde::shape_aws_iot_analytics_parameters::ser_aws_iot_analytics_parameters(&mut object_5, inner)?;
            object_5.finish();
        }
        crate::types::DataSourceParameters::JiraParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_6 = object_5.key("JiraParameters").start_object();
            crate::protocol_serde::shape_jira_parameters::ser_jira_parameters(&mut object_6, inner)?;
            object_6.finish();
        }
        crate::types::DataSourceParameters::MariaDbParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_7 = object_5.key("MariaDbParameters").start_object();
            crate::protocol_serde::shape_maria_db_parameters::ser_maria_db_parameters(&mut object_7, inner)?;
            object_7.finish();
        }
        crate::types::DataSourceParameters::MySqlParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_8 = object_5.key("MySqlParameters").start_object();
            crate::protocol_serde::shape_my_sql_parameters::ser_my_sql_parameters(&mut object_8, inner)?;
            object_8.finish();
        }
        crate::types::DataSourceParameters::OracleParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_9 = object_5.key("OracleParameters").start_object();
            crate::protocol_serde::shape_oracle_parameters::ser_oracle_parameters(&mut object_9, inner)?;
            object_9.finish();
        }
        crate::types::DataSourceParameters::PostgreSqlParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_10 = object_5.key("PostgreSqlParameters").start_object();
            crate::protocol_serde::shape_postgre_sql_parameters::ser_postgre_sql_parameters(&mut object_10, inner)?;
            object_10.finish();
        }
        crate::types::DataSourceParameters::PrestoParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_11 = object_5.key("PrestoParameters").start_object();
            crate::protocol_serde::shape_presto_parameters::ser_presto_parameters(&mut object_11, inner)?;
            object_11.finish();
        }
        crate::types::DataSourceParameters::RdsParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_12 = object_5.key("RdsParameters").start_object();
            crate::protocol_serde::shape_rds_parameters::ser_rds_parameters(&mut object_12, inner)?;
            object_12.finish();
        }
        crate::types::DataSourceParameters::RedshiftParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_13 = object_5.key("RedshiftParameters").start_object();
            crate::protocol_serde::shape_redshift_parameters::ser_redshift_parameters(&mut object_13, inner)?;
            object_13.finish();
        }
        crate::types::DataSourceParameters::S3Parameters(inner) => {
            #[allow(unused_mut)]
            let mut object_14 = object_5.key("S3Parameters").start_object();
            crate::protocol_serde::shape_s3_parameters::ser_s3_parameters(&mut object_14, inner)?;
            object_14.finish();
        }
        crate::types::DataSourceParameters::ServiceNowParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_15 = object_5.key("ServiceNowParameters").start_object();
            crate::protocol_serde::shape_service_now_parameters::ser_service_now_parameters(&mut object_15, inner)?;
            object_15.finish();
        }
        crate::types::DataSourceParameters::SnowflakeParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_16 = object_5.key("SnowflakeParameters").start_object();
            crate::protocol_serde::shape_snowflake_parameters::ser_snowflake_parameters(&mut object_16, inner)?;
            object_16.finish();
        }
        crate::types::DataSourceParameters::SparkParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_17 = object_5.key("SparkParameters").start_object();
            crate::protocol_serde::shape_spark_parameters::ser_spark_parameters(&mut object_17, inner)?;
            object_17.finish();
        }
        crate::types::DataSourceParameters::SqlServerParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_18 = object_5.key("SqlServerParameters").start_object();
            crate::protocol_serde::shape_sql_server_parameters::ser_sql_server_parameters(&mut object_18, inner)?;
            object_18.finish();
        }
        crate::types::DataSourceParameters::TeradataParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_19 = object_5.key("TeradataParameters").start_object();
            crate::protocol_serde::shape_teradata_parameters::ser_teradata_parameters(&mut object_19, inner)?;
            object_19.finish();
        }
        crate::types::DataSourceParameters::TwitterParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_20 = object_5.key("TwitterParameters").start_object();
            crate::protocol_serde::shape_twitter_parameters::ser_twitter_parameters(&mut object_20, inner)?;
            object_20.finish();
        }
        crate::types::DataSourceParameters::AmazonOpenSearchParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_21 = object_5.key("AmazonOpenSearchParameters").start_object();
            crate::protocol_serde::shape_amazon_open_search_parameters::ser_amazon_open_search_parameters(&mut object_21, inner)?;
            object_21.finish();
        }
        crate::types::DataSourceParameters::ExasolParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_22 = object_5.key("ExasolParameters").start_object();
            crate::protocol_serde::shape_exasol_parameters::ser_exasol_parameters(&mut object_22, inner)?;
            object_22.finish();
        }
        crate::types::DataSourceParameters::DatabricksParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_23 = object_5.key("DatabricksParameters").start_object();
            crate::protocol_serde::shape_databricks_parameters::ser_databricks_parameters(&mut object_23, inner)?;
            object_23.finish();
        }
        crate::types::DataSourceParameters::StarburstParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_24 = object_5.key("StarburstParameters").start_object();
            crate::protocol_serde::shape_starburst_parameters::ser_starburst_parameters(&mut object_24, inner)?;
            object_24.finish();
        }
        crate::types::DataSourceParameters::TrinoParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_25 = object_5.key("TrinoParameters").start_object();
            crate::protocol_serde::shape_trino_parameters::ser_trino_parameters(&mut object_25, inner)?;
            object_25.finish();
        }
        crate::types::DataSourceParameters::BigQueryParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_26 = object_5.key("BigQueryParameters").start_object();
            crate::protocol_serde::shape_big_query_parameters::ser_big_query_parameters(&mut object_26, inner)?;
            object_26.finish();
        }
        crate::types::DataSourceParameters::ImpalaParameters(inner) => {
            #[allow(unused_mut)]
            let mut object_27 = object_5.key("ImpalaParameters").start_object();
            crate::protocol_serde::shape_impala_parameters::ser_impala_parameters(&mut object_27, inner)?;
            object_27.finish();
        }
        crate::types::DataSourceParameters::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "DataSourceParameters",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_data_source_parameters<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::DataSourceParameters>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
                        "AmazonElasticsearchParameters" => Some(crate::types::DataSourceParameters::AmazonElasticsearchParameters(
                            crate::protocol_serde::shape_amazon_elasticsearch_parameters::de_amazon_elasticsearch_parameters(tokens)?.ok_or_else(
                                || {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                        "value for 'AmazonElasticsearchParameters' cannot be null",
                                    )
                                },
                            )?,
                        )),
                        "AthenaParameters" => Some(crate::types::DataSourceParameters::AthenaParameters(
                            crate::protocol_serde::shape_athena_parameters::de_athena_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'AthenaParameters' cannot be null")
                            })?,
                        )),
                        "AuroraParameters" => Some(crate::types::DataSourceParameters::AuroraParameters(
                            crate::protocol_serde::shape_aurora_parameters::de_aurora_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'AuroraParameters' cannot be null")
                            })?,
                        )),
                        "AuroraPostgreSqlParameters" => Some(crate::types::DataSourceParameters::AuroraPostgreSqlParameters(
                            crate::protocol_serde::shape_aurora_postgre_sql_parameters::de_aurora_postgre_sql_parameters(tokens)?.ok_or_else(
                                || {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                        "value for 'AuroraPostgreSqlParameters' cannot be null",
                                    )
                                },
                            )?,
                        )),
                        "AwsIotAnalyticsParameters" => Some(crate::types::DataSourceParameters::AwsIotAnalyticsParameters(
                            crate::protocol_serde::shape_aws_iot_analytics_parameters::de_aws_iot_analytics_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                    "value for 'AwsIotAnalyticsParameters' cannot be null",
                                )
                            })?,
                        )),
                        "JiraParameters" => Some(crate::types::DataSourceParameters::JiraParameters(
                            crate::protocol_serde::shape_jira_parameters::de_jira_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'JiraParameters' cannot be null")
                            })?,
                        )),
                        "MariaDbParameters" => Some(crate::types::DataSourceParameters::MariaDbParameters(
                            crate::protocol_serde::shape_maria_db_parameters::de_maria_db_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'MariaDbParameters' cannot be null")
                            })?,
                        )),
                        "MySqlParameters" => Some(crate::types::DataSourceParameters::MySqlParameters(
                            crate::protocol_serde::shape_my_sql_parameters::de_my_sql_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'MySqlParameters' cannot be null")
                            })?,
                        )),
                        "OracleParameters" => Some(crate::types::DataSourceParameters::OracleParameters(
                            crate::protocol_serde::shape_oracle_parameters::de_oracle_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'OracleParameters' cannot be null")
                            })?,
                        )),
                        "PostgreSqlParameters" => Some(crate::types::DataSourceParameters::PostgreSqlParameters(
                            crate::protocol_serde::shape_postgre_sql_parameters::de_postgre_sql_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'PostgreSqlParameters' cannot be null")
                            })?,
                        )),
                        "PrestoParameters" => Some(crate::types::DataSourceParameters::PrestoParameters(
                            crate::protocol_serde::shape_presto_parameters::de_presto_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'PrestoParameters' cannot be null")
                            })?,
                        )),
                        "RdsParameters" => Some(crate::types::DataSourceParameters::RdsParameters(
                            crate::protocol_serde::shape_rds_parameters::de_rds_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'RdsParameters' cannot be null")
                            })?,
                        )),
                        "RedshiftParameters" => Some(crate::types::DataSourceParameters::RedshiftParameters(
                            crate::protocol_serde::shape_redshift_parameters::de_redshift_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'RedshiftParameters' cannot be null")
                            })?,
                        )),
                        "S3Parameters" => Some(crate::types::DataSourceParameters::S3Parameters(
                            crate::protocol_serde::shape_s3_parameters::de_s3_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'S3Parameters' cannot be null")
                            })?,
                        )),
                        "ServiceNowParameters" => Some(crate::types::DataSourceParameters::ServiceNowParameters(
                            crate::protocol_serde::shape_service_now_parameters::de_service_now_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ServiceNowParameters' cannot be null")
                            })?,
                        )),
                        "SnowflakeParameters" => Some(crate::types::DataSourceParameters::SnowflakeParameters(
                            crate::protocol_serde::shape_snowflake_parameters::de_snowflake_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'SnowflakeParameters' cannot be null")
                            })?,
                        )),
                        "SparkParameters" => Some(crate::types::DataSourceParameters::SparkParameters(
                            crate::protocol_serde::shape_spark_parameters::de_spark_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'SparkParameters' cannot be null")
                            })?,
                        )),
                        "SqlServerParameters" => Some(crate::types::DataSourceParameters::SqlServerParameters(
                            crate::protocol_serde::shape_sql_server_parameters::de_sql_server_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'SqlServerParameters' cannot be null")
                            })?,
                        )),
                        "TeradataParameters" => Some(crate::types::DataSourceParameters::TeradataParameters(
                            crate::protocol_serde::shape_teradata_parameters::de_teradata_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'TeradataParameters' cannot be null")
                            })?,
                        )),
                        "TwitterParameters" => Some(crate::types::DataSourceParameters::TwitterParameters(
                            crate::protocol_serde::shape_twitter_parameters::de_twitter_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'TwitterParameters' cannot be null")
                            })?,
                        )),
                        "AmazonOpenSearchParameters" => Some(crate::types::DataSourceParameters::AmazonOpenSearchParameters(
                            crate::protocol_serde::shape_amazon_open_search_parameters::de_amazon_open_search_parameters(tokens)?.ok_or_else(
                                || {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                        "value for 'AmazonOpenSearchParameters' cannot be null",
                                    )
                                },
                            )?,
                        )),
                        "ExasolParameters" => Some(crate::types::DataSourceParameters::ExasolParameters(
                            crate::protocol_serde::shape_exasol_parameters::de_exasol_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ExasolParameters' cannot be null")
                            })?,
                        )),
                        "DatabricksParameters" => Some(crate::types::DataSourceParameters::DatabricksParameters(
                            crate::protocol_serde::shape_databricks_parameters::de_databricks_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'DatabricksParameters' cannot be null")
                            })?,
                        )),
                        "StarburstParameters" => Some(crate::types::DataSourceParameters::StarburstParameters(
                            crate::protocol_serde::shape_starburst_parameters::de_starburst_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'StarburstParameters' cannot be null")
                            })?,
                        )),
                        "TrinoParameters" => Some(crate::types::DataSourceParameters::TrinoParameters(
                            crate::protocol_serde::shape_trino_parameters::de_trino_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'TrinoParameters' cannot be null")
                            })?,
                        )),
                        "BigQueryParameters" => Some(crate::types::DataSourceParameters::BigQueryParameters(
                            crate::protocol_serde::shape_big_query_parameters::de_big_query_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'BigQueryParameters' cannot be null")
                            })?,
                        )),
                        "ImpalaParameters" => Some(crate::types::DataSourceParameters::ImpalaParameters(
                            crate::protocol_serde::shape_impala_parameters::de_impala_parameters(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ImpalaParameters' cannot be null")
                            })?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::DataSourceParameters::Unknown)
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
