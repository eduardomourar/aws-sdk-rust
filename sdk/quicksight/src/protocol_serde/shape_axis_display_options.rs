// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_axis_display_options(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AxisDisplayOptions,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.tick_label_options {
        #[allow(unused_mut)]
        let mut object_2 = object.key("TickLabelOptions").start_object();
        crate::protocol_serde::shape_axis_tick_label_options::ser_axis_tick_label_options(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.axis_line_visibility {
        object.key("AxisLineVisibility").string(var_3.as_str());
    }
    if let Some(var_4) = &input.grid_line_visibility {
        object.key("GridLineVisibility").string(var_4.as_str());
    }
    if let Some(var_5) = &input.data_options {
        #[allow(unused_mut)]
        let mut object_6 = object.key("DataOptions").start_object();
        crate::protocol_serde::shape_axis_data_options::ser_axis_data_options(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.scrollbar_options {
        #[allow(unused_mut)]
        let mut object_8 = object.key("ScrollbarOptions").start_object();
        crate::protocol_serde::shape_scroll_bar_options::ser_scroll_bar_options(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.axis_offset {
        object.key("AxisOffset").string(var_9.as_str());
    }
    Ok(())
}

pub(crate) fn de_axis_display_options<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AxisDisplayOptions>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AxisDisplayOptionsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "TickLabelOptions" => {
                            builder = builder
                                .set_tick_label_options(crate::protocol_serde::shape_axis_tick_label_options::de_axis_tick_label_options(tokens)?);
                        }
                        "AxisLineVisibility" => {
                            builder = builder.set_axis_line_visibility(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Visibility::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "GridLineVisibility" => {
                            builder = builder.set_grid_line_visibility(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Visibility::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "DataOptions" => {
                            builder = builder.set_data_options(crate::protocol_serde::shape_axis_data_options::de_axis_data_options(tokens)?);
                        }
                        "ScrollbarOptions" => {
                            builder = builder.set_scrollbar_options(crate::protocol_serde::shape_scroll_bar_options::de_scroll_bar_options(tokens)?);
                        }
                        "AxisOffset" => {
                            builder = builder.set_axis_offset(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
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
