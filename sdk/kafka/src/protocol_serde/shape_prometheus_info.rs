// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_prometheus_info(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PrometheusInfo,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.jmx_exporter {
        #[allow(unused_mut)]
        let mut object_2 = object.key("jmxExporter").start_object();
        crate::protocol_serde::shape_jmx_exporter_info::ser_jmx_exporter_info(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.node_exporter {
        #[allow(unused_mut)]
        let mut object_4 = object.key("nodeExporter").start_object();
        crate::protocol_serde::shape_node_exporter_info::ser_node_exporter_info(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_prometheus_info<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::PrometheusInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::PrometheusInfoBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "jmxExporter" => {
                            builder = builder.set_jmx_exporter(crate::protocol_serde::shape_jmx_exporter_info::de_jmx_exporter_info(tokens)?);
                        }
                        "nodeExporter" => {
                            builder = builder.set_node_exporter(crate::protocol_serde::shape_node_exporter_info::de_node_exporter_info(tokens)?);
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
