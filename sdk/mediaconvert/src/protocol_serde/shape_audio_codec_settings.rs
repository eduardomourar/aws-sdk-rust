// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_audio_codec_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AudioCodecSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.aac_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("aacSettings").start_object();
        crate::protocol_serde::shape_aac_settings::ser_aac_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.ac3_settings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ac3Settings").start_object();
        crate::protocol_serde::shape_ac3_settings::ser_ac3_settings(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.aiff_settings {
        #[allow(unused_mut)]
        let mut object_6 = object.key("aiffSettings").start_object();
        crate::protocol_serde::shape_aiff_settings::ser_aiff_settings(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.codec {
        object.key("codec").string(var_7.as_str());
    }
    if let Some(var_8) = &input.eac3_atmos_settings {
        #[allow(unused_mut)]
        let mut object_9 = object.key("eac3AtmosSettings").start_object();
        crate::protocol_serde::shape_eac3_atmos_settings::ser_eac3_atmos_settings(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.eac3_settings {
        #[allow(unused_mut)]
        let mut object_11 = object.key("eac3Settings").start_object();
        crate::protocol_serde::shape_eac3_settings::ser_eac3_settings(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.flac_settings {
        #[allow(unused_mut)]
        let mut object_13 = object.key("flacSettings").start_object();
        crate::protocol_serde::shape_flac_settings::ser_flac_settings(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.mp2_settings {
        #[allow(unused_mut)]
        let mut object_15 = object.key("mp2Settings").start_object();
        crate::protocol_serde::shape_mp2_settings::ser_mp2_settings(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.mp3_settings {
        #[allow(unused_mut)]
        let mut object_17 = object.key("mp3Settings").start_object();
        crate::protocol_serde::shape_mp3_settings::ser_mp3_settings(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.opus_settings {
        #[allow(unused_mut)]
        let mut object_19 = object.key("opusSettings").start_object();
        crate::protocol_serde::shape_opus_settings::ser_opus_settings(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.vorbis_settings {
        #[allow(unused_mut)]
        let mut object_21 = object.key("vorbisSettings").start_object();
        crate::protocol_serde::shape_vorbis_settings::ser_vorbis_settings(&mut object_21, var_20)?;
        object_21.finish();
    }
    if let Some(var_22) = &input.wav_settings {
        #[allow(unused_mut)]
        let mut object_23 = object.key("wavSettings").start_object();
        crate::protocol_serde::shape_wav_settings::ser_wav_settings(&mut object_23, var_22)?;
        object_23.finish();
    }
    Ok(())
}

pub(crate) fn de_audio_codec_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AudioCodecSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AudioCodecSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "aacSettings" => {
                            builder = builder.set_aac_settings(crate::protocol_serde::shape_aac_settings::de_aac_settings(tokens)?);
                        }
                        "ac3Settings" => {
                            builder = builder.set_ac3_settings(crate::protocol_serde::shape_ac3_settings::de_ac3_settings(tokens)?);
                        }
                        "aiffSettings" => {
                            builder = builder.set_aiff_settings(crate::protocol_serde::shape_aiff_settings::de_aiff_settings(tokens)?);
                        }
                        "codec" => {
                            builder = builder.set_codec(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AudioCodec::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "eac3AtmosSettings" => {
                            builder =
                                builder.set_eac3_atmos_settings(crate::protocol_serde::shape_eac3_atmos_settings::de_eac3_atmos_settings(tokens)?);
                        }
                        "eac3Settings" => {
                            builder = builder.set_eac3_settings(crate::protocol_serde::shape_eac3_settings::de_eac3_settings(tokens)?);
                        }
                        "flacSettings" => {
                            builder = builder.set_flac_settings(crate::protocol_serde::shape_flac_settings::de_flac_settings(tokens)?);
                        }
                        "mp2Settings" => {
                            builder = builder.set_mp2_settings(crate::protocol_serde::shape_mp2_settings::de_mp2_settings(tokens)?);
                        }
                        "mp3Settings" => {
                            builder = builder.set_mp3_settings(crate::protocol_serde::shape_mp3_settings::de_mp3_settings(tokens)?);
                        }
                        "opusSettings" => {
                            builder = builder.set_opus_settings(crate::protocol_serde::shape_opus_settings::de_opus_settings(tokens)?);
                        }
                        "vorbisSettings" => {
                            builder = builder.set_vorbis_settings(crate::protocol_serde::shape_vorbis_settings::de_vorbis_settings(tokens)?);
                        }
                        "wavSettings" => {
                            builder = builder.set_wav_settings(crate::protocol_serde::shape_wav_settings::de_wav_settings(tokens)?);
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
