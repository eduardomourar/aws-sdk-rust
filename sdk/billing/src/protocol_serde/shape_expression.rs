// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_expression(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Expression,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.dimensions {
        #[allow(unused_mut)]
        let mut object_2 = object.key("dimensions").start_object();
        crate::protocol_serde::shape_dimension_values::ser_dimension_values(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.tags {
        #[allow(unused_mut)]
        let mut object_4 = object.key("tags").start_object();
        crate::protocol_serde::shape_tag_values::ser_tag_values(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_expression<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::Expression>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ExpressionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "dimensions" => {
                            builder = builder.set_dimensions(crate::protocol_serde::shape_dimension_values::de_dimension_values(tokens)?);
                        }
                        "tags" => {
                            builder = builder.set_tags(crate::protocol_serde::shape_tag_values::de_tag_values(tokens)?);
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
