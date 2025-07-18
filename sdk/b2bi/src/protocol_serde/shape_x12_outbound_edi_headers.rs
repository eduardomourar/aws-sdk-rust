// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_x12_outbound_edi_headers<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::X12OutboundEdiHeaders>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::X12OutboundEdiHeadersBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "interchangeControlHeaders" => {
                            builder = builder.set_interchange_control_headers(
                                crate::protocol_serde::shape_x12_interchange_control_headers::de_x12_interchange_control_headers(tokens)?,
                            );
                        }
                        "functionalGroupHeaders" => {
                            builder = builder.set_functional_group_headers(
                                crate::protocol_serde::shape_x12_functional_group_headers::de_x12_functional_group_headers(tokens)?,
                            );
                        }
                        "delimiters" => {
                            builder = builder.set_delimiters(crate::protocol_serde::shape_x12_delimiters::de_x12_delimiters(tokens)?);
                        }
                        "validateEdi" => {
                            builder = builder.set_validate_edi(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "controlNumbers" => {
                            builder = builder.set_control_numbers(crate::protocol_serde::shape_x12_control_numbers::de_x12_control_numbers(tokens)?);
                        }
                        "gs05TimeFormat" => {
                            builder = builder.set_gs05_time_format(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::X12Gs05TimeFormat::from(u.as_ref())))
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

pub fn ser_x12_outbound_edi_headers(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::X12OutboundEdiHeaders,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.interchange_control_headers {
        #[allow(unused_mut)]
        let mut object_2 = object.key("interchangeControlHeaders").start_object();
        crate::protocol_serde::shape_x12_interchange_control_headers::ser_x12_interchange_control_headers(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.functional_group_headers {
        #[allow(unused_mut)]
        let mut object_4 = object.key("functionalGroupHeaders").start_object();
        crate::protocol_serde::shape_x12_functional_group_headers::ser_x12_functional_group_headers(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.delimiters {
        #[allow(unused_mut)]
        let mut object_6 = object.key("delimiters").start_object();
        crate::protocol_serde::shape_x12_delimiters::ser_x12_delimiters(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.validate_edi {
        object.key("validateEdi").boolean(*var_7);
    }
    if let Some(var_8) = &input.control_numbers {
        #[allow(unused_mut)]
        let mut object_9 = object.key("controlNumbers").start_object();
        crate::protocol_serde::shape_x12_control_numbers::ser_x12_control_numbers(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.gs05_time_format {
        object.key("gs05TimeFormat").string(var_10.as_str());
    }
    Ok(())
}
