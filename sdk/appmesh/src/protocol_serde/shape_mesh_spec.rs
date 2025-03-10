// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_mesh_spec(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MeshSpec,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.egress_filter {
        #[allow(unused_mut)]
        let mut object_2 = object.key("egressFilter").start_object();
        crate::protocol_serde::shape_egress_filter::ser_egress_filter(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.service_discovery {
        #[allow(unused_mut)]
        let mut object_4 = object.key("serviceDiscovery").start_object();
        crate::protocol_serde::shape_mesh_service_discovery::ser_mesh_service_discovery(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_mesh_spec<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::MeshSpec>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::MeshSpecBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "egressFilter" => {
                            builder = builder.set_egress_filter(crate::protocol_serde::shape_egress_filter::de_egress_filter(tokens)?);
                        }
                        "serviceDiscovery" => {
                            builder = builder
                                .set_service_discovery(crate::protocol_serde::shape_mesh_service_discovery::de_mesh_service_discovery(tokens)?);
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
