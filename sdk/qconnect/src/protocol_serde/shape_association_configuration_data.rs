// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_association_configuration_data(
    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AssociationConfigurationData,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::AssociationConfigurationData::KnowledgeBaseAssociationConfigurationData(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_4.key("knowledgeBaseAssociationConfigurationData").start_object();
            crate::protocol_serde::shape_knowledge_base_association_configuration_data::ser_knowledge_base_association_configuration_data(
                &mut object_1,
                inner,
            )?;
            object_1.finish();
        }
        crate::types::AssociationConfigurationData::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "AssociationConfigurationData",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_association_configuration_data<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AssociationConfigurationData>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
                        tokens.peek()
                    {
                        let _ = tokens.next().expect("peek returned a token")?;
                        continue;
                    }
                    let key = key.to_unescaped()?;
                    if key == "__type" {
                        ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                        continue;
                    }
                    if variant.is_some() {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            "encountered mixed variants in union",
                        ));
                    }
                    variant = match key.as_ref() {
                            "knowledgeBaseAssociationConfigurationData" => {
                                Some(crate::types::AssociationConfigurationData::KnowledgeBaseAssociationConfigurationData(
                                    crate::protocol_serde::shape_knowledge_base_association_configuration_data::de_knowledge_base_association_configuration_data(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'knowledgeBaseAssociationConfigurationData' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                              ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                              Some(crate::types::AssociationConfigurationData::Unknown)
                                                                            }
                        };
                }
                other => {
                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )))
                }
            }
        },
        _ => {
            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ))
        }
    }
    if variant.is_none() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "Union did not contain a valid variant.",
        ));
    }
    Ok(variant)
}
