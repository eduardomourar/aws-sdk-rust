// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_signing_job_parameter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::StartSigningJobParameter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.signing_profile_parameter {
        #[allow(unused_mut)]
        let mut object_2 = object.key("signingProfileParameter").start_object();
        crate::protocol_serde::shape_signing_profile_parameter::ser_signing_profile_parameter(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.signing_profile_name {
        object.key("signingProfileName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.destination {
        #[allow(unused_mut)]
        let mut object_5 = object.key("destination").start_object();
        crate::protocol_serde::shape_destination::ser_destination(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

pub(crate) fn de_start_signing_job_parameter<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::StartSigningJobParameter>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::StartSigningJobParameterBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "signingProfileParameter" => {
                            builder = builder.set_signing_profile_parameter(
                                crate::protocol_serde::shape_signing_profile_parameter::de_signing_profile_parameter(tokens)?,
                            );
                        }
                        "signingProfileName" => {
                            builder = builder.set_signing_profile_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "destination" => {
                            builder = builder.set_destination(crate::protocol_serde::shape_destination::de_destination(tokens)?);
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
