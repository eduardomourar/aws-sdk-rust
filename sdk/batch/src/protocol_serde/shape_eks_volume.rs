// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_eks_volume(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EksVolume,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.host_path {
        #[allow(unused_mut)]
        let mut object_3 = object.key("hostPath").start_object();
        crate::protocol_serde::shape_eks_host_path::ser_eks_host_path(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.empty_dir {
        #[allow(unused_mut)]
        let mut object_5 = object.key("emptyDir").start_object();
        crate::protocol_serde::shape_eks_empty_dir::ser_eks_empty_dir(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.secret {
        #[allow(unused_mut)]
        let mut object_7 = object.key("secret").start_object();
        crate::protocol_serde::shape_eks_secret::ser_eks_secret(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.persistent_volume_claim {
        #[allow(unused_mut)]
        let mut object_9 = object.key("persistentVolumeClaim").start_object();
        crate::protocol_serde::shape_eks_persistent_volume_claim::ser_eks_persistent_volume_claim(&mut object_9, var_8)?;
        object_9.finish();
    }
    Ok(())
}

pub(crate) fn de_eks_volume<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::EksVolume>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::EksVolumeBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "hostPath" => {
                            builder = builder.set_host_path(crate::protocol_serde::shape_eks_host_path::de_eks_host_path(tokens)?);
                        }
                        "emptyDir" => {
                            builder = builder.set_empty_dir(crate::protocol_serde::shape_eks_empty_dir::de_eks_empty_dir(tokens)?);
                        }
                        "secret" => {
                            builder = builder.set_secret(crate::protocol_serde::shape_eks_secret::de_eks_secret(tokens)?);
                        }
                        "persistentVolumeClaim" => {
                            builder = builder.set_persistent_volume_claim(
                                crate::protocol_serde::shape_eks_persistent_volume_claim::de_eks_persistent_volume_claim(tokens)?,
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
            Ok(Some(crate::serde_util::eks_volume_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
