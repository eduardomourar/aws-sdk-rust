// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_funnel_chart_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FunnelChartConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.field_wells {
        #[allow(unused_mut)]
        let mut object_2 = object.key("FieldWells").start_object();
        crate::protocol_serde::shape_funnel_chart_field_wells::ser_funnel_chart_field_wells(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.sort_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SortConfiguration").start_object();
        crate::protocol_serde::shape_funnel_chart_sort_configuration::ser_funnel_chart_sort_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.category_label_options {
        #[allow(unused_mut)]
        let mut object_6 = object.key("CategoryLabelOptions").start_object();
        crate::protocol_serde::shape_chart_axis_label_options::ser_chart_axis_label_options(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.value_label_options {
        #[allow(unused_mut)]
        let mut object_8 = object.key("ValueLabelOptions").start_object();
        crate::protocol_serde::shape_chart_axis_label_options::ser_chart_axis_label_options(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.tooltip {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Tooltip").start_object();
        crate::protocol_serde::shape_tooltip_options::ser_tooltip_options(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.data_label_options {
        #[allow(unused_mut)]
        let mut object_12 = object.key("DataLabelOptions").start_object();
        crate::protocol_serde::shape_funnel_chart_data_label_options::ser_funnel_chart_data_label_options(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.visual_palette {
        #[allow(unused_mut)]
        let mut object_14 = object.key("VisualPalette").start_object();
        crate::protocol_serde::shape_visual_palette::ser_visual_palette(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.interactions {
        #[allow(unused_mut)]
        let mut object_16 = object.key("Interactions").start_object();
        crate::protocol_serde::shape_visual_interaction_options::ser_visual_interaction_options(&mut object_16, var_15)?;
        object_16.finish();
    }
    Ok(())
}

pub(crate) fn de_funnel_chart_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::FunnelChartConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::FunnelChartConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "FieldWells" => {
                            builder = builder.set_field_wells(crate::protocol_serde::shape_funnel_chart_field_wells::de_funnel_chart_field_wells(
                                tokens,
                            )?);
                        }
                        "SortConfiguration" => {
                            builder = builder.set_sort_configuration(
                                crate::protocol_serde::shape_funnel_chart_sort_configuration::de_funnel_chart_sort_configuration(tokens)?,
                            );
                        }
                        "CategoryLabelOptions" => {
                            builder = builder.set_category_label_options(
                                crate::protocol_serde::shape_chart_axis_label_options::de_chart_axis_label_options(tokens)?,
                            );
                        }
                        "ValueLabelOptions" => {
                            builder = builder.set_value_label_options(
                                crate::protocol_serde::shape_chart_axis_label_options::de_chart_axis_label_options(tokens)?,
                            );
                        }
                        "Tooltip" => {
                            builder = builder.set_tooltip(crate::protocol_serde::shape_tooltip_options::de_tooltip_options(tokens)?);
                        }
                        "DataLabelOptions" => {
                            builder = builder.set_data_label_options(
                                crate::protocol_serde::shape_funnel_chart_data_label_options::de_funnel_chart_data_label_options(tokens)?,
                            );
                        }
                        "VisualPalette" => {
                            builder = builder.set_visual_palette(crate::protocol_serde::shape_visual_palette::de_visual_palette(tokens)?);
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
