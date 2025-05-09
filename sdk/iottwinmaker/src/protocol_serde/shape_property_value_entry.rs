// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_property_value_entry(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PropertyValueEntry,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.entity_property_reference {
        #[allow(unused_mut)]
        let mut object_2 = object.key("entityPropertyReference").start_object();
        crate::protocol_serde::shape_entity_property_reference::ser_entity_property_reference(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.property_values {
        let mut array_4 = object.key("propertyValues").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_property_value::ser_property_value(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}

pub(crate) fn de_property_value_entry<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::PropertyValueEntry>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::PropertyValueEntryBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "entityPropertyReference" => {
                            builder = builder.set_entity_property_reference(
                                crate::protocol_serde::shape_entity_property_reference::de_entity_property_reference(tokens)?,
                            );
                        }
                        "propertyValues" => {
                            builder = builder.set_property_values(crate::protocol_serde::shape_property_values::de_property_values(tokens)?);
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
            Ok(Some(crate::serde_util::property_value_entry_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
