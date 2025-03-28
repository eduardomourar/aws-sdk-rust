// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_job_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::JobSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.ad_avail_offset {
        object.key("adAvailOffset").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.avail_blanking {
        #[allow(unused_mut)]
        let mut object_3 = object.key("availBlanking").start_object();
        crate::protocol_serde::shape_avail_blanking::ser_avail_blanking(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.color_conversion3_dlut_settings {
        let mut array_5 = object.key("colorConversion3DLUTSettings").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_color_conversion3_dlut_setting::ser_color_conversion3_dlut_setting(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.esam {
        #[allow(unused_mut)]
        let mut object_9 = object.key("esam").start_object();
        crate::protocol_serde::shape_esam_settings::ser_esam_settings(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.extended_data_services {
        #[allow(unused_mut)]
        let mut object_11 = object.key("extendedDataServices").start_object();
        crate::protocol_serde::shape_extended_data_services::ser_extended_data_services(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.follow_source {
        object.key("followSource").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    if let Some(var_13) = &input.inputs {
        let mut array_14 = object.key("inputs").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_input::ser_input(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.kantar_watermark {
        #[allow(unused_mut)]
        let mut object_18 = object.key("kantarWatermark").start_object();
        crate::protocol_serde::shape_kantar_watermark_settings::ser_kantar_watermark_settings(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.motion_image_inserter {
        #[allow(unused_mut)]
        let mut object_20 = object.key("motionImageInserter").start_object();
        crate::protocol_serde::shape_motion_image_inserter::ser_motion_image_inserter(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.nielsen_configuration {
        #[allow(unused_mut)]
        let mut object_22 = object.key("nielsenConfiguration").start_object();
        crate::protocol_serde::shape_nielsen_configuration::ser_nielsen_configuration(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.nielsen_non_linear_watermark {
        #[allow(unused_mut)]
        let mut object_24 = object.key("nielsenNonLinearWatermark").start_object();
        crate::protocol_serde::shape_nielsen_non_linear_watermark_settings::ser_nielsen_non_linear_watermark_settings(&mut object_24, var_23)?;
        object_24.finish();
    }
    if let Some(var_25) = &input.output_groups {
        let mut array_26 = object.key("outputGroups").start_array();
        for item_27 in var_25 {
            {
                #[allow(unused_mut)]
                let mut object_28 = array_26.value().start_object();
                crate::protocol_serde::shape_output_group::ser_output_group(&mut object_28, item_27)?;
                object_28.finish();
            }
        }
        array_26.finish();
    }
    if let Some(var_29) = &input.timecode_config {
        #[allow(unused_mut)]
        let mut object_30 = object.key("timecodeConfig").start_object();
        crate::protocol_serde::shape_timecode_config::ser_timecode_config(&mut object_30, var_29)?;
        object_30.finish();
    }
    if let Some(var_31) = &input.timed_metadata_insertion {
        #[allow(unused_mut)]
        let mut object_32 = object.key("timedMetadataInsertion").start_object();
        crate::protocol_serde::shape_timed_metadata_insertion::ser_timed_metadata_insertion(&mut object_32, var_31)?;
        object_32.finish();
    }
    Ok(())
}

pub(crate) fn de_job_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::JobSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::JobSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "adAvailOffset" => {
                            builder = builder.set_ad_avail_offset(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "availBlanking" => {
                            builder = builder.set_avail_blanking(crate::protocol_serde::shape_avail_blanking::de_avail_blanking(tokens)?);
                        }
                        "colorConversion3DLUTSettings" => {
                            builder = builder.set_color_conversion3_dlut_settings(
                                crate::protocol_serde::shape_list_of_color_conversion3_dlut_setting::de_list_of_color_conversion3_dlut_setting(
                                    tokens,
                                )?,
                            );
                        }
                        "esam" => {
                            builder = builder.set_esam(crate::protocol_serde::shape_esam_settings::de_esam_settings(tokens)?);
                        }
                        "extendedDataServices" => {
                            builder = builder
                                .set_extended_data_services(crate::protocol_serde::shape_extended_data_services::de_extended_data_services(tokens)?);
                        }
                        "followSource" => {
                            builder = builder.set_follow_source(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "inputs" => {
                            builder = builder.set_inputs(crate::protocol_serde::shape_list_of_input::de_list_of_input(tokens)?);
                        }
                        "kantarWatermark" => {
                            builder = builder.set_kantar_watermark(
                                crate::protocol_serde::shape_kantar_watermark_settings::de_kantar_watermark_settings(tokens)?,
                            );
                        }
                        "motionImageInserter" => {
                            builder = builder
                                .set_motion_image_inserter(crate::protocol_serde::shape_motion_image_inserter::de_motion_image_inserter(tokens)?);
                        }
                        "nielsenConfiguration" => {
                            builder = builder
                                .set_nielsen_configuration(crate::protocol_serde::shape_nielsen_configuration::de_nielsen_configuration(tokens)?);
                        }
                        "nielsenNonLinearWatermark" => {
                            builder = builder.set_nielsen_non_linear_watermark(
                                crate::protocol_serde::shape_nielsen_non_linear_watermark_settings::de_nielsen_non_linear_watermark_settings(tokens)?,
                            );
                        }
                        "outputGroups" => {
                            builder = builder.set_output_groups(crate::protocol_serde::shape_list_of_output_group::de_list_of_output_group(tokens)?);
                        }
                        "timecodeConfig" => {
                            builder = builder.set_timecode_config(crate::protocol_serde::shape_timecode_config::de_timecode_config(tokens)?);
                        }
                        "timedMetadataInsertion" => {
                            builder = builder.set_timed_metadata_insertion(
                                crate::protocol_serde::shape_timed_metadata_insertion::de_timed_metadata_insertion(tokens)?,
                            );
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
