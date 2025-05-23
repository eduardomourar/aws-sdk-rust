// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_display_format_options(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DisplayFormatOptions,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if input.use_blank_cell_format {
        object.key("UseBlankCellFormat").boolean(input.use_blank_cell_format);
    }
    if let Some(var_1) = &input.blank_cell_format {
        object.key("BlankCellFormat").string(var_1.as_str());
    }
    if let Some(var_2) = &input.date_format {
        object.key("DateFormat").string(var_2.as_str());
    }
    if let Some(var_3) = &input.decimal_separator {
        object.key("DecimalSeparator").string(var_3.as_str());
    }
    if let Some(var_4) = &input.grouping_separator {
        object.key("GroupingSeparator").string(var_4.as_str());
    }
    if input.use_grouping {
        object.key("UseGrouping").boolean(input.use_grouping);
    }
    if input.fraction_digits != 0 {
        object.key("FractionDigits").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.fraction_digits).into()),
        );
    }
    if let Some(var_5) = &input.prefix {
        object.key("Prefix").string(var_5.as_str());
    }
    if let Some(var_6) = &input.suffix {
        object.key("Suffix").string(var_6.as_str());
    }
    if let Some(var_7) = &input.unit_scaler {
        object.key("UnitScaler").string(var_7.as_str());
    }
    if let Some(var_8) = &input.negative_format {
        #[allow(unused_mut)]
        let mut object_9 = object.key("NegativeFormat").start_object();
        crate::protocol_serde::shape_negative_format::ser_negative_format(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.currency_symbol {
        object.key("CurrencySymbol").string(var_10.as_str());
    }
    Ok(())
}

pub(crate) fn de_display_format_options<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::DisplayFormatOptions>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::DisplayFormatOptionsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "UseBlankCellFormat" => {
                            builder = builder.set_use_blank_cell_format(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "BlankCellFormat" => {
                            builder = builder.set_blank_cell_format(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "DateFormat" => {
                            builder = builder.set_date_format(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "DecimalSeparator" => {
                            builder = builder.set_decimal_separator(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::TopicNumericSeparatorSymbol::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "GroupingSeparator" => {
                            builder = builder.set_grouping_separator(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "UseGrouping" => {
                            builder = builder.set_use_grouping(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "FractionDigits" => {
                            builder = builder.set_fraction_digits(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "Prefix" => {
                            builder = builder.set_prefix(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Suffix" => {
                            builder = builder.set_suffix(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "UnitScaler" => {
                            builder = builder.set_unit_scaler(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::NumberScale::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "NegativeFormat" => {
                            builder = builder.set_negative_format(crate::protocol_serde::shape_negative_format::de_negative_format(tokens)?);
                        }
                        "CurrencySymbol" => {
                            builder = builder.set_currency_symbol(
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
