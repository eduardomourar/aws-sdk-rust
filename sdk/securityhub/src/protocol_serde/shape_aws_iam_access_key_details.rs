// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_iam_access_key_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsIamAccessKeyDetails,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.user_name {
        object.key("UserName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.status {
        object.key("Status").string(var_2.as_str());
    }
    if let Some(var_3) = &input.created_at {
        object.key("CreatedAt").string(var_3.as_str());
    }
    if let Some(var_4) = &input.principal_id {
        object.key("PrincipalId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.principal_type {
        object.key("PrincipalType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.principal_name {
        object.key("PrincipalName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.account_id {
        object.key("AccountId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.access_key_id {
        object.key("AccessKeyId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.session_context {
        #[allow(unused_mut)]
        let mut object_10 = object.key("SessionContext").start_object();
        crate::protocol_serde::shape_aws_iam_access_key_session_context::ser_aws_iam_access_key_session_context(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_iam_access_key_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AwsIamAccessKeyDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsIamAccessKeyDetailsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "UserName" => {
                            builder = builder.set_user_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Status" => {
                            builder = builder.set_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AwsIamAccessKeyStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "CreatedAt" => {
                            builder = builder.set_created_at(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PrincipalId" => {
                            builder = builder.set_principal_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PrincipalType" => {
                            builder = builder.set_principal_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PrincipalName" => {
                            builder = builder.set_principal_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "AccountId" => {
                            builder = builder.set_account_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "AccessKeyId" => {
                            builder = builder.set_access_key_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "SessionContext" => {
                            builder = builder.set_session_context(
                                crate::protocol_serde::shape_aws_iam_access_key_session_context::de_aws_iam_access_key_session_context(tokens)?,
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
