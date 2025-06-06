// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_closed_days(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ClosedDays,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.email {
        let mut array_2 = object.key("EMAIL").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_closed_days_rule::ser_closed_days_rule(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.sms {
        let mut array_6 = object.key("SMS").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_closed_days_rule::ser_closed_days_rule(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.push {
        let mut array_10 = object.key("PUSH").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_closed_days_rule::ser_closed_days_rule(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.voice {
        let mut array_14 = object.key("VOICE").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_closed_days_rule::ser_closed_days_rule(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.custom {
        let mut array_18 = object.key("CUSTOM").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_closed_days_rule::ser_closed_days_rule(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    Ok(())
}

pub(crate) fn de_closed_days<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ClosedDays>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ClosedDaysBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "EMAIL" => {
                            builder = builder.set_email(crate::protocol_serde::shape_list_of_closed_days_rules::de_list_of_closed_days_rules(
                                tokens,
                            )?);
                        }
                        "SMS" => {
                            builder = builder.set_sms(crate::protocol_serde::shape_list_of_closed_days_rules::de_list_of_closed_days_rules(
                                tokens,
                            )?);
                        }
                        "PUSH" => {
                            builder = builder.set_push(crate::protocol_serde::shape_list_of_closed_days_rules::de_list_of_closed_days_rules(
                                tokens,
                            )?);
                        }
                        "VOICE" => {
                            builder = builder.set_voice(crate::protocol_serde::shape_list_of_closed_days_rules::de_list_of_closed_days_rules(
                                tokens,
                            )?);
                        }
                        "CUSTOM" => {
                            builder = builder.set_custom(crate::protocol_serde::shape_list_of_closed_days_rules::de_list_of_closed_days_rules(
                                tokens,
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
