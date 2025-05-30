// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_application_source(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ApplicationSource,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cloud_formation_stack_arn {
        object.key("CloudFormationStackARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tag_filters {
        let mut array_3 = object.key("TagFilters").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_tag_filter::ser_tag_filter(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub(crate) fn de_application_source<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ApplicationSource>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ApplicationSourceBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "CloudFormationStackARN" => {
                            builder = builder.set_cloud_formation_stack_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TagFilters" => {
                            builder = builder.set_tag_filters(crate::protocol_serde::shape_tag_filters::de_tag_filters(tokens)?);
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
