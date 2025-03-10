// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_classification_result(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ClassificationResult,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.mime_type {
        object.key("MimeType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.size_classified {
        object.key("SizeClassified").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.additional_occurrences {
        object.key("AdditionalOccurrences").boolean(*var_3);
    }
    if let Some(var_4) = &input.status {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Status").start_object();
        crate::protocol_serde::shape_classification_status::ser_classification_status(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.sensitive_data {
        let mut array_7 = object.key("SensitiveData").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_sensitive_data_result::ser_sensitive_data_result(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.custom_data_identifiers {
        #[allow(unused_mut)]
        let mut object_11 = object.key("CustomDataIdentifiers").start_object();
        crate::protocol_serde::shape_custom_data_identifiers_result::ser_custom_data_identifiers_result(&mut object_11, var_10)?;
        object_11.finish();
    }
    Ok(())
}

pub(crate) fn de_classification_result<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ClassificationResult>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ClassificationResultBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "MimeType" => {
                            builder = builder.set_mime_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "SizeClassified" => {
                            builder = builder.set_size_classified(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "AdditionalOccurrences" => {
                            builder = builder.set_additional_occurrences(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "Status" => {
                            builder = builder.set_status(crate::protocol_serde::shape_classification_status::de_classification_status(tokens)?);
                        }
                        "SensitiveData" => {
                            builder = builder.set_sensitive_data(
                                crate::protocol_serde::shape_sensitive_data_result_list::de_sensitive_data_result_list(tokens)?,
                            );
                        }
                        "CustomDataIdentifiers" => {
                            builder = builder.set_custom_data_identifiers(
                                crate::protocol_serde::shape_custom_data_identifiers_result::de_custom_data_identifiers_result(tokens)?,
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
