// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resource_filter_criteria(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ResourceFilterCriteria,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.account_id {
        let mut array_2 = object.key("accountId").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_resource_string_filter::ser_resource_string_filter(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.resource_id {
        let mut array_6 = object.key("resourceId").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_resource_string_filter::ser_resource_string_filter(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.resource_type {
        let mut array_10 = object.key("resourceType").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_resource_string_filter::ser_resource_string_filter(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.ecr_repository_name {
        let mut array_14 = object.key("ecrRepositoryName").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_resource_string_filter::ser_resource_string_filter(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.lambda_function_name {
        let mut array_18 = object.key("lambdaFunctionName").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_resource_string_filter::ser_resource_string_filter(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.ecr_image_tags {
        let mut array_22 = object.key("ecrImageTags").start_array();
        for item_23 in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_24 = array_22.value().start_object();
                crate::protocol_serde::shape_resource_string_filter::ser_resource_string_filter(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if let Some(var_25) = &input.ec2_instance_tags {
        let mut array_26 = object.key("ec2InstanceTags").start_array();
        for item_27 in var_25 {
            {
                #[allow(unused_mut)]
                let mut object_28 = array_26.value().start_object();
                crate::protocol_serde::shape_resource_map_filter::ser_resource_map_filter(&mut object_28, item_27)?;
                object_28.finish();
            }
        }
        array_26.finish();
    }
    if let Some(var_29) = &input.lambda_function_tags {
        let mut array_30 = object.key("lambdaFunctionTags").start_array();
        for item_31 in var_29 {
            {
                #[allow(unused_mut)]
                let mut object_32 = array_30.value().start_object();
                crate::protocol_serde::shape_resource_map_filter::ser_resource_map_filter(&mut object_32, item_31)?;
                object_32.finish();
            }
        }
        array_30.finish();
    }
    Ok(())
}

pub(crate) fn de_resource_filter_criteria<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ResourceFilterCriteria>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ResourceFilterCriteriaBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "accountId" => {
                            builder = builder.set_account_id(
                                crate::protocol_serde::shape_resource_string_filter_list::de_resource_string_filter_list(tokens)?,
                            );
                        }
                        "resourceId" => {
                            builder = builder.set_resource_id(
                                crate::protocol_serde::shape_resource_string_filter_list::de_resource_string_filter_list(tokens)?,
                            );
                        }
                        "resourceType" => {
                            builder = builder.set_resource_type(
                                crate::protocol_serde::shape_resource_string_filter_list::de_resource_string_filter_list(tokens)?,
                            );
                        }
                        "ecrRepositoryName" => {
                            builder = builder.set_ecr_repository_name(
                                crate::protocol_serde::shape_resource_string_filter_list::de_resource_string_filter_list(tokens)?,
                            );
                        }
                        "lambdaFunctionName" => {
                            builder = builder.set_lambda_function_name(
                                crate::protocol_serde::shape_resource_string_filter_list::de_resource_string_filter_list(tokens)?,
                            );
                        }
                        "ecrImageTags" => {
                            builder = builder.set_ecr_image_tags(
                                crate::protocol_serde::shape_resource_string_filter_list::de_resource_string_filter_list(tokens)?,
                            );
                        }
                        "ec2InstanceTags" => {
                            builder = builder.set_ec2_instance_tags(
                                crate::protocol_serde::shape_resource_map_filter_list::de_resource_map_filter_list(tokens)?,
                            );
                        }
                        "lambdaFunctionTags" => {
                            builder = builder.set_lambda_function_tags(
                                crate::protocol_serde::shape_resource_map_filter_list::de_resource_map_filter_list(tokens)?,
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
