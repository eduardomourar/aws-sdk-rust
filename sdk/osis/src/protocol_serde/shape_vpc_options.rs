// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_vpc_options(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::VpcOptions,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("SubnetIds").start_array();
        for item_2 in &input.subnet_ids {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    if let Some(var_3) = &input.security_group_ids {
        let mut array_4 = object.key("SecurityGroupIds").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.vpc_attachment_options {
        #[allow(unused_mut)]
        let mut object_7 = object.key("VpcAttachmentOptions").start_object();
        crate::protocol_serde::shape_vpc_attachment_options::ser_vpc_attachment_options(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.vpc_endpoint_management {
        object.key("VpcEndpointManagement").string(var_8.as_str());
    }
    Ok(())
}

pub(crate) fn de_vpc_options<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::VpcOptions>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::VpcOptionsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "SubnetIds" => {
                            builder = builder.set_subnet_ids(crate::protocol_serde::shape_subnet_ids::de_subnet_ids(tokens)?);
                        }
                        "SecurityGroupIds" => {
                            builder = builder.set_security_group_ids(crate::protocol_serde::shape_security_group_ids::de_security_group_ids(tokens)?);
                        }
                        "VpcAttachmentOptions" => {
                            builder = builder
                                .set_vpc_attachment_options(crate::protocol_serde::shape_vpc_attachment_options::de_vpc_attachment_options(tokens)?);
                        }
                        "VpcEndpointManagement" => {
                            builder = builder.set_vpc_endpoint_management(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::VpcEndpointManagement::from(u.as_ref())))
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
            Ok(Some(crate::serde_util::vpc_options_correct_errors(builder).build().map_err(|err| {
                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
            })?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
