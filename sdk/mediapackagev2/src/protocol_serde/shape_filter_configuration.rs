// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_filter_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FilterConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.manifest_filter {
        object.key("ManifestFilter").string(var_1.as_str());
    }
    if let Some(var_2) = &input.start {
        object
            .key("Start")
            .date_time(var_2, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_3) = &input.end {
        object.key("End").date_time(var_3, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.time_delay_seconds {
        object.key("TimeDelaySeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.clip_start_time {
        object
            .key("ClipStartTime")
            .date_time(var_5, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub(crate) fn de_filter_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::FilterConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::FilterConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ManifestFilter" => {
                            builder = builder.set_manifest_filter(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Start" => {
                            builder = builder.set_start(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "End" => {
                            builder = builder.set_end(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "TimeDelaySeconds" => {
                            builder = builder.set_time_delay_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "ClipStartTime" => {
                            builder = builder.set_clip_start_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
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
