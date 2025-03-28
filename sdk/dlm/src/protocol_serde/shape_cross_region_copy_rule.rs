// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cross_region_copy_rule(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CrossRegionCopyRule,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.target_region {
        object.key("TargetRegion").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target {
        object.key("Target").string(var_2.as_str());
    }
    if let Some(var_3) = &input.encrypted {
        object.key("Encrypted").boolean(*var_3);
    }
    if let Some(var_4) = &input.cmk_arn {
        object.key("CmkArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.copy_tags {
        object.key("CopyTags").boolean(*var_5);
    }
    if let Some(var_6) = &input.retain_rule {
        #[allow(unused_mut)]
        let mut object_7 = object.key("RetainRule").start_object();
        crate::protocol_serde::shape_cross_region_copy_retain_rule::ser_cross_region_copy_retain_rule(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.deprecate_rule {
        #[allow(unused_mut)]
        let mut object_9 = object.key("DeprecateRule").start_object();
        crate::protocol_serde::shape_cross_region_copy_deprecate_rule::ser_cross_region_copy_deprecate_rule(&mut object_9, var_8)?;
        object_9.finish();
    }
    Ok(())
}

pub(crate) fn de_cross_region_copy_rule<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::CrossRegionCopyRule>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CrossRegionCopyRuleBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "TargetRegion" => {
                            builder = builder.set_target_region(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Target" => {
                            builder = builder.set_target(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Encrypted" => {
                            builder = builder.set_encrypted(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "CmkArn" => {
                            builder = builder.set_cmk_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CopyTags" => {
                            builder = builder.set_copy_tags(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "RetainRule" => {
                            builder = builder.set_retain_rule(
                                crate::protocol_serde::shape_cross_region_copy_retain_rule::de_cross_region_copy_retain_rule(tokens)?,
                            );
                        }
                        "DeprecateRule" => {
                            builder = builder.set_deprecate_rule(
                                crate::protocol_serde::shape_cross_region_copy_deprecate_rule::de_cross_region_copy_deprecate_rule(tokens)?,
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
            Ok(Some(crate::serde_util::cross_region_copy_rule_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
