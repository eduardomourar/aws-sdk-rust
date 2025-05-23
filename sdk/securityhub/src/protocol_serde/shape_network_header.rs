// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_network_header(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::NetworkHeader,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.protocol {
        object.key("Protocol").string(var_1.as_str());
    }
    if let Some(var_2) = &input.destination {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Destination").start_object();
        crate::protocol_serde::shape_network_path_component_details::ser_network_path_component_details(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.source {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Source").start_object();
        crate::protocol_serde::shape_network_path_component_details::ser_network_path_component_details(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

pub(crate) fn de_network_header<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::NetworkHeader>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::NetworkHeaderBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Protocol" => {
                            builder = builder.set_protocol(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Destination" => {
                            builder = builder.set_destination(
                                crate::protocol_serde::shape_network_path_component_details::de_network_path_component_details(tokens)?,
                            );
                        }
                        "Source" => {
                            builder = builder
                                .set_source(crate::protocol_serde::shape_network_path_component_details::de_network_path_component_details(tokens)?);
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
