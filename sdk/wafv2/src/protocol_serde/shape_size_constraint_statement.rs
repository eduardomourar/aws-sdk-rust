// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_size_constraint_statement(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SizeConstraintStatement,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.field_to_match {
        #[allow(unused_mut)]
        let mut object_2 = object.key("FieldToMatch").start_object();
        crate::protocol_serde::shape_field_to_match::ser_field_to_match(&mut object_2, var_1)?;
        object_2.finish();
    }
    {
        object.key("ComparisonOperator").string(input.comparison_operator.as_str());
    }
    {
        object.key("Size").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.size).into()),
        );
    }
    {
        let mut array_3 = object.key("TextTransformations").start_array();
        for item_4 in &input.text_transformations {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_text_transformation::ser_text_transformation(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub(crate) fn de_size_constraint_statement<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::SizeConstraintStatement>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SizeConstraintStatementBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "FieldToMatch" => {
                            builder = builder.set_field_to_match(crate::protocol_serde::shape_field_to_match::de_field_to_match(tokens)?);
                        }
                        "ComparisonOperator" => {
                            builder = builder.set_comparison_operator(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ComparisonOperator::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "Size" => {
                            builder = builder.set_size(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "TextTransformations" => {
                            builder =
                                builder.set_text_transformations(crate::protocol_serde::shape_text_transformations::de_text_transformations(tokens)?);
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
                crate::serde_util::size_constraint_statement_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
