// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_email_message_template_content(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EmailMessageTemplateContent,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.subject {
        object.key("subject").string(var_1.as_str());
    }
    if let Some(var_2) = &input.body {
        #[allow(unused_mut)]
        let mut object_3 = object.key("body").start_object();
        crate::protocol_serde::shape_email_message_template_content_body::ser_email_message_template_content_body(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.headers {
        let mut array_5 = object.key("headers").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_email_header::ser_email_header(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}

pub(crate) fn de_email_message_template_content<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::EmailMessageTemplateContent>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::EmailMessageTemplateContentBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "subject" => {
                            builder = builder.set_subject(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "body" => {
                            builder = builder.set_body(
                                crate::protocol_serde::shape_email_message_template_content_body::de_email_message_template_content_body(tokens)?,
                            );
                        }
                        "headers" => {
                            builder = builder.set_headers(crate::protocol_serde::shape_email_headers::de_email_headers(tokens)?);
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
