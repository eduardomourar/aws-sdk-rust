// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_data_source_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DataSourceConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.data_source_s3_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("DataSourceS3Configuration").start_object();
        crate::protocol_serde::shape_data_source_s3_configuration::ser_data_source_s3_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.csv_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("CsvConfiguration").start_object();
        crate::protocol_serde::shape_csv_configuration::ser_csv_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    {
        object.key("DataFormat").string(input.data_format.as_str());
    }
    Ok(())
}

pub(crate) fn de_data_source_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::DataSourceConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::DataSourceConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "DataSourceS3Configuration" => {
                            builder = builder.set_data_source_s3_configuration(
                                crate::protocol_serde::shape_data_source_s3_configuration::de_data_source_s3_configuration(tokens)?,
                            );
                        }
                        "CsvConfiguration" => {
                            builder = builder.set_csv_configuration(crate::protocol_serde::shape_csv_configuration::de_csv_configuration(tokens)?);
                        }
                        "DataFormat" => {
                            builder = builder.set_data_format(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::BatchLoadDataFormat::from(u.as_ref())))
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
            Ok(Some(
                crate::serde_util::data_source_configuration_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
