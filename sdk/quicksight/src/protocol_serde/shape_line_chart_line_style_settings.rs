// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_line_chart_line_style_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::LineChartLineStyleSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.line_visibility {
        object.key("LineVisibility").string(var_1.as_str());
    }
    if let Some(var_2) = &input.line_interpolation {
        object.key("LineInterpolation").string(var_2.as_str());
    }
    if let Some(var_3) = &input.line_style {
        object.key("LineStyle").string(var_3.as_str());
    }
    if let Some(var_4) = &input.line_width {
        object.key("LineWidth").string(var_4.as_str());
    }
    Ok(())
}

pub(crate) fn de_line_chart_line_style_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::LineChartLineStyleSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::LineChartLineStyleSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "LineVisibility" => {
                            builder = builder.set_line_visibility(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Visibility::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "LineInterpolation" => {
                            builder = builder.set_line_interpolation(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::LineInterpolation::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "LineStyle" => {
                            builder = builder.set_line_style(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::LineChartLineStyle::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "LineWidth" => {
                            builder = builder.set_line_width(
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
