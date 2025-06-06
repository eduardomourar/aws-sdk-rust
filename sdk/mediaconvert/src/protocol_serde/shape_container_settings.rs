// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_container_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ContainerSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cmfc_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("cmfcSettings").start_object();
        crate::protocol_serde::shape_cmfc_settings::ser_cmfc_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.container {
        object.key("container").string(var_3.as_str());
    }
    if let Some(var_4) = &input.f4v_settings {
        #[allow(unused_mut)]
        let mut object_5 = object.key("f4vSettings").start_object();
        crate::protocol_serde::shape_f4v_settings::ser_f4v_settings(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.m2ts_settings {
        #[allow(unused_mut)]
        let mut object_7 = object.key("m2tsSettings").start_object();
        crate::protocol_serde::shape_m2ts_settings::ser_m2ts_settings(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.m3u8_settings {
        #[allow(unused_mut)]
        let mut object_9 = object.key("m3u8Settings").start_object();
        crate::protocol_serde::shape_m3u8_settings::ser_m3u8_settings(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.mov_settings {
        #[allow(unused_mut)]
        let mut object_11 = object.key("movSettings").start_object();
        crate::protocol_serde::shape_mov_settings::ser_mov_settings(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.mp4_settings {
        #[allow(unused_mut)]
        let mut object_13 = object.key("mp4Settings").start_object();
        crate::protocol_serde::shape_mp4_settings::ser_mp4_settings(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.mpd_settings {
        #[allow(unused_mut)]
        let mut object_15 = object.key("mpdSettings").start_object();
        crate::protocol_serde::shape_mpd_settings::ser_mpd_settings(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.mxf_settings {
        #[allow(unused_mut)]
        let mut object_17 = object.key("mxfSettings").start_object();
        crate::protocol_serde::shape_mxf_settings::ser_mxf_settings(&mut object_17, var_16)?;
        object_17.finish();
    }
    Ok(())
}

pub(crate) fn de_container_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ContainerSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ContainerSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "cmfcSettings" => {
                            builder = builder.set_cmfc_settings(crate::protocol_serde::shape_cmfc_settings::de_cmfc_settings(tokens)?);
                        }
                        "container" => {
                            builder = builder.set_container(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ContainerType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "f4vSettings" => {
                            builder = builder.set_f4v_settings(crate::protocol_serde::shape_f4v_settings::de_f4v_settings(tokens)?);
                        }
                        "m2tsSettings" => {
                            builder = builder.set_m2ts_settings(crate::protocol_serde::shape_m2ts_settings::de_m2ts_settings(tokens)?);
                        }
                        "m3u8Settings" => {
                            builder = builder.set_m3u8_settings(crate::protocol_serde::shape_m3u8_settings::de_m3u8_settings(tokens)?);
                        }
                        "movSettings" => {
                            builder = builder.set_mov_settings(crate::protocol_serde::shape_mov_settings::de_mov_settings(tokens)?);
                        }
                        "mp4Settings" => {
                            builder = builder.set_mp4_settings(crate::protocol_serde::shape_mp4_settings::de_mp4_settings(tokens)?);
                        }
                        "mpdSettings" => {
                            builder = builder.set_mpd_settings(crate::protocol_serde::shape_mpd_settings::de_mpd_settings(tokens)?);
                        }
                        "mxfSettings" => {
                            builder = builder.set_mxf_settings(crate::protocol_serde::shape_mxf_settings::de_mxf_settings(tokens)?);
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
