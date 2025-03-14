// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_static_file(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::StaticFile,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.image_static_file {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ImageStaticFile").start_object();
        crate::protocol_serde::shape_image_static_file::ser_image_static_file(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.spatial_static_file {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SpatialStaticFile").start_object();
        crate::protocol_serde::shape_spatial_static_file::ser_spatial_static_file(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_static_file<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::StaticFile>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::StaticFileBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ImageStaticFile" => {
                            builder = builder.set_image_static_file(crate::protocol_serde::shape_image_static_file::de_image_static_file(tokens)?);
                        }
                        "SpatialStaticFile" => {
                            builder =
                                builder.set_spatial_static_file(crate::protocol_serde::shape_spatial_static_file::de_spatial_static_file(tokens)?);
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
