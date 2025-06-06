// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_auto_ml_channel(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AutoMlChannel,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.data_source {
        #[allow(unused_mut)]
        let mut object_2 = object.key("DataSource").start_object();
        crate::protocol_serde::shape_auto_ml_data_source::ser_auto_ml_data_source(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.compression_type {
        object.key("CompressionType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.target_attribute_name {
        object.key("TargetAttributeName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.content_type {
        object.key("ContentType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.channel_type {
        object.key("ChannelType").string(var_6.as_str());
    }
    if let Some(var_7) = &input.sample_weight_attribute_name {
        object.key("SampleWeightAttributeName").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_auto_ml_channel<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AutoMlChannel>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AutoMlChannelBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "DataSource" => {
                            builder = builder.set_data_source(crate::protocol_serde::shape_auto_ml_data_source::de_auto_ml_data_source(tokens)?);
                        }
                        "CompressionType" => {
                            builder = builder.set_compression_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::CompressionType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "TargetAttributeName" => {
                            builder = builder.set_target_attribute_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ContentType" => {
                            builder = builder.set_content_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ChannelType" => {
                            builder = builder.set_channel_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AutoMlChannelType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "SampleWeightAttributeName" => {
                            builder = builder.set_sample_weight_attribute_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
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
            Ok(Some(crate::serde_util::auto_ml_channel_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
