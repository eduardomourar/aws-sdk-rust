// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_global_secondary_index(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GlobalSecondaryIndex,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("IndexName").string(input.index_name.as_str());
    }
    {
        let mut array_1 = object.key("KeySchema").start_array();
        for item_2 in &input.key_schema {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_key_schema_element::ser_key_schema_element(&mut object_3, item_2)?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    if let Some(var_4) = &input.projection {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Projection").start_object();
        crate::protocol_serde::shape_projection::ser_projection(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.provisioned_throughput {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ProvisionedThroughput").start_object();
        crate::protocol_serde::shape_provisioned_throughput::ser_provisioned_throughput(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.on_demand_throughput {
        #[allow(unused_mut)]
        let mut object_9 = object.key("OnDemandThroughput").start_object();
        crate::protocol_serde::shape_on_demand_throughput::ser_on_demand_throughput(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.warm_throughput {
        #[allow(unused_mut)]
        let mut object_11 = object.key("WarmThroughput").start_object();
        crate::protocol_serde::shape_warm_throughput::ser_warm_throughput(&mut object_11, var_10)?;
        object_11.finish();
    }
    Ok(())
}

pub(crate) fn de_global_secondary_index<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::GlobalSecondaryIndex>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::GlobalSecondaryIndexBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "IndexName" => {
                            builder = builder.set_index_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "KeySchema" => {
                            builder = builder.set_key_schema(crate::protocol_serde::shape_key_schema::de_key_schema(tokens)?);
                        }
                        "Projection" => {
                            builder = builder.set_projection(crate::protocol_serde::shape_projection::de_projection(tokens)?);
                        }
                        "ProvisionedThroughput" => {
                            builder = builder
                                .set_provisioned_throughput(crate::protocol_serde::shape_provisioned_throughput::de_provisioned_throughput(tokens)?);
                        }
                        "OnDemandThroughput" => {
                            builder =
                                builder.set_on_demand_throughput(crate::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(tokens)?);
                        }
                        "WarmThroughput" => {
                            builder = builder.set_warm_throughput(crate::protocol_serde::shape_warm_throughput::de_warm_throughput(tokens)?);
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
            Ok(Some(crate::serde_util::global_secondary_index_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
