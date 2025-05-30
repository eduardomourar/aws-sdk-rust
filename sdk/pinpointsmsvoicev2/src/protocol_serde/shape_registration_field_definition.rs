// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_registration_field_definition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::RegistrationFieldDefinition>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RegistrationFieldDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "SectionPath" => {
                            builder = builder.set_section_path(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FieldPath" => {
                            builder = builder.set_field_path(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FieldType" => {
                            builder = builder.set_field_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::FieldType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "FieldRequirement" => {
                            builder = builder.set_field_requirement(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::FieldRequirement::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "SelectValidation" => {
                            builder = builder.set_select_validation(crate::protocol_serde::shape_select_validation::de_select_validation(tokens)?);
                        }
                        "TextValidation" => {
                            builder = builder.set_text_validation(crate::protocol_serde::shape_text_validation::de_text_validation(tokens)?);
                        }
                        "DisplayHints" => {
                            builder = builder.set_display_hints(
                                crate::protocol_serde::shape_registration_field_display_hints::de_registration_field_display_hints(tokens)?,
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
                crate::serde_util::registration_field_definition_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
