// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_exponential_rollout_rate(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ExponentialRolloutRate,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("baseRatePerMinute").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.base_rate_per_minute).into()),
        );
    }
    {
        object.key("incrementFactor").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((input.increment_factor).into()),
        );
    }
    if let Some(var_1) = &input.rate_increase_criteria {
        #[allow(unused_mut)]
        let mut object_2 = object.key("rateIncreaseCriteria").start_object();
        crate::protocol_serde::shape_rate_increase_criteria::ser_rate_increase_criteria(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_exponential_rollout_rate<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ExponentialRolloutRate>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ExponentialRolloutRateBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "baseRatePerMinute" => {
                            builder = builder.set_base_rate_per_minute(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "incrementFactor" => {
                            builder = builder.set_increment_factor(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()),
                            );
                        }
                        "rateIncreaseCriteria" => {
                            builder = builder
                                .set_rate_increase_criteria(crate::protocol_serde::shape_rate_increase_criteria::de_rate_increase_criteria(tokens)?);
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
            Ok(Some(
                crate::serde_util::exponential_rollout_rate_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
