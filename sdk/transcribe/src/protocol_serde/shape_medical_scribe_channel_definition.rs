// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_medical_scribe_channel_definition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MedicalScribeChannelDefinition,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("ChannelId").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.channel_id).into()),
        );
    }
    {
        object.key("ParticipantRole").string(input.participant_role.as_str());
    }
    Ok(())
}

pub(crate) fn de_medical_scribe_channel_definition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::MedicalScribeChannelDefinition>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::MedicalScribeChannelDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ChannelId" => {
                            builder = builder.set_channel_id(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "ParticipantRole" => {
                            builder = builder.set_participant_role(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::MedicalScribeParticipantRole::from(u.as_ref())))
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
            Ok(Some(
                crate::serde_util::medical_scribe_channel_definition_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
