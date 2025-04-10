// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_geospatial_layer_definition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GeospatialLayerDefinition,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.point_layer {
        #[allow(unused_mut)]
        let mut object_2 = object.key("PointLayer").start_object();
        crate::protocol_serde::shape_geospatial_point_layer::ser_geospatial_point_layer(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.line_layer {
        #[allow(unused_mut)]
        let mut object_4 = object.key("LineLayer").start_object();
        crate::protocol_serde::shape_geospatial_line_layer::ser_geospatial_line_layer(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.polygon_layer {
        #[allow(unused_mut)]
        let mut object_6 = object.key("PolygonLayer").start_object();
        crate::protocol_serde::shape_geospatial_polygon_layer::ser_geospatial_polygon_layer(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_geospatial_layer_definition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::GeospatialLayerDefinition>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::GeospatialLayerDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "PointLayer" => {
                            builder =
                                builder.set_point_layer(crate::protocol_serde::shape_geospatial_point_layer::de_geospatial_point_layer(tokens)?);
                        }
                        "LineLayer" => {
                            builder = builder.set_line_layer(crate::protocol_serde::shape_geospatial_line_layer::de_geospatial_line_layer(tokens)?);
                        }
                        "PolygonLayer" => {
                            builder = builder.set_polygon_layer(crate::protocol_serde::shape_geospatial_polygon_layer::de_geospatial_polygon_layer(
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
