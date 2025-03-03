// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_waterfall_chart_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::WaterfallChartConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.field_wells {
        #[allow(unused_mut)]
        let mut object_2 = object.key("FieldWells").start_object();
        crate::protocol_serde::shape_waterfall_chart_field_wells::ser_waterfall_chart_field_wells(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.sort_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SortConfiguration").start_object();
        crate::protocol_serde::shape_waterfall_chart_sort_configuration::ser_waterfall_chart_sort_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.waterfall_chart_options {
        #[allow(unused_mut)]
        let mut object_6 = object.key("WaterfallChartOptions").start_object();
        crate::protocol_serde::shape_waterfall_chart_options::ser_waterfall_chart_options(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.category_axis_label_options {
        #[allow(unused_mut)]
        let mut object_8 = object.key("CategoryAxisLabelOptions").start_object();
        crate::protocol_serde::shape_chart_axis_label_options::ser_chart_axis_label_options(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.category_axis_display_options {
        #[allow(unused_mut)]
        let mut object_10 = object.key("CategoryAxisDisplayOptions").start_object();
        crate::protocol_serde::shape_axis_display_options::ser_axis_display_options(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.primary_y_axis_label_options {
        #[allow(unused_mut)]
        let mut object_12 = object.key("PrimaryYAxisLabelOptions").start_object();
        crate::protocol_serde::shape_chart_axis_label_options::ser_chart_axis_label_options(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.primary_y_axis_display_options {
        #[allow(unused_mut)]
        let mut object_14 = object.key("PrimaryYAxisDisplayOptions").start_object();
        crate::protocol_serde::shape_axis_display_options::ser_axis_display_options(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.legend {
        #[allow(unused_mut)]
        let mut object_16 = object.key("Legend").start_object();
        crate::protocol_serde::shape_legend_options::ser_legend_options(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.data_labels {
        #[allow(unused_mut)]
        let mut object_18 = object.key("DataLabels").start_object();
        crate::protocol_serde::shape_data_label_options::ser_data_label_options(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.visual_palette {
        #[allow(unused_mut)]
        let mut object_20 = object.key("VisualPalette").start_object();
        crate::protocol_serde::shape_visual_palette::ser_visual_palette(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.color_configuration {
        #[allow(unused_mut)]
        let mut object_22 = object.key("ColorConfiguration").start_object();
        crate::protocol_serde::shape_waterfall_chart_color_configuration::ser_waterfall_chart_color_configuration(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.interactions {
        #[allow(unused_mut)]
        let mut object_24 = object.key("Interactions").start_object();
        crate::protocol_serde::shape_visual_interaction_options::ser_visual_interaction_options(&mut object_24, var_23)?;
        object_24.finish();
    }
    Ok(())
}

pub(crate) fn de_waterfall_chart_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::WaterfallChartConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::WaterfallChartConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "FieldWells" => {
                            builder = builder.set_field_wells(
                                crate::protocol_serde::shape_waterfall_chart_field_wells::de_waterfall_chart_field_wells(tokens)?,
                            );
                        }
                        "SortConfiguration" => {
                            builder = builder.set_sort_configuration(
                                crate::protocol_serde::shape_waterfall_chart_sort_configuration::de_waterfall_chart_sort_configuration(tokens)?,
                            );
                        }
                        "WaterfallChartOptions" => {
                            builder = builder.set_waterfall_chart_options(
                                crate::protocol_serde::shape_waterfall_chart_options::de_waterfall_chart_options(tokens)?,
                            );
                        }
                        "CategoryAxisLabelOptions" => {
                            builder = builder.set_category_axis_label_options(
                                crate::protocol_serde::shape_chart_axis_label_options::de_chart_axis_label_options(tokens)?,
                            );
                        }
                        "CategoryAxisDisplayOptions" => {
                            builder = builder.set_category_axis_display_options(
                                crate::protocol_serde::shape_axis_display_options::de_axis_display_options(tokens)?,
                            );
                        }
                        "PrimaryYAxisLabelOptions" => {
                            builder = builder.set_primary_y_axis_label_options(
                                crate::protocol_serde::shape_chart_axis_label_options::de_chart_axis_label_options(tokens)?,
                            );
                        }
                        "PrimaryYAxisDisplayOptions" => {
                            builder = builder.set_primary_y_axis_display_options(
                                crate::protocol_serde::shape_axis_display_options::de_axis_display_options(tokens)?,
                            );
                        }
                        "Legend" => {
                            builder = builder.set_legend(crate::protocol_serde::shape_legend_options::de_legend_options(tokens)?);
                        }
                        "DataLabels" => {
                            builder = builder.set_data_labels(crate::protocol_serde::shape_data_label_options::de_data_label_options(tokens)?);
                        }
                        "VisualPalette" => {
                            builder = builder.set_visual_palette(crate::protocol_serde::shape_visual_palette::de_visual_palette(tokens)?);
                        }
                        "ColorConfiguration" => {
                            builder = builder.set_color_configuration(
                                crate::protocol_serde::shape_waterfall_chart_color_configuration::de_waterfall_chart_color_configuration(tokens)?,
                            );
                        }
                        "Interactions" => {
                            builder = builder.set_interactions(
                                crate::protocol_serde::shape_visual_interaction_options::de_visual_interaction_options(tokens)?,
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
