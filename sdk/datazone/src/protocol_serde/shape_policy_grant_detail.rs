// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_policy_grant_detail(
    object_3: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PolicyGrantDetail,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::PolicyGrantDetail::CreateDomainUnit(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_3.key("createDomainUnit").start_object();
            crate::protocol_serde::shape_create_domain_unit_policy_grant_detail::ser_create_domain_unit_policy_grant_detail(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::PolicyGrantDetail::OverrideDomainUnitOwners(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_3.key("overrideDomainUnitOwners").start_object();
            crate::protocol_serde::shape_override_domain_unit_owners_policy_grant_detail::ser_override_domain_unit_owners_policy_grant_detail(
                &mut object_2,
                inner,
            )?;
            object_2.finish();
        }
        crate::types::PolicyGrantDetail::AddToProjectMemberPool(inner) => {
            #[allow(unused_mut)]
            let mut object_3 = object_3.key("addToProjectMemberPool").start_object();
            crate::protocol_serde::shape_add_to_project_member_pool_policy_grant_detail::ser_add_to_project_member_pool_policy_grant_detail(
                &mut object_3,
                inner,
            )?;
            object_3.finish();
        }
        crate::types::PolicyGrantDetail::OverrideProjectOwners(inner) => {
            #[allow(unused_mut)]
            let mut object_4 = object_3.key("overrideProjectOwners").start_object();
            crate::protocol_serde::shape_override_project_owners_policy_grant_detail::ser_override_project_owners_policy_grant_detail(
                &mut object_4,
                inner,
            )?;
            object_4.finish();
        }
        crate::types::PolicyGrantDetail::CreateGlossary(inner) => {
            #[allow(unused_mut)]
            let mut object_5 = object_3.key("createGlossary").start_object();
            crate::protocol_serde::shape_create_glossary_policy_grant_detail::ser_create_glossary_policy_grant_detail(&mut object_5, inner)?;
            object_5.finish();
        }
        crate::types::PolicyGrantDetail::CreateFormType(inner) => {
            #[allow(unused_mut)]
            let mut object_6 = object_3.key("createFormType").start_object();
            crate::protocol_serde::shape_create_form_type_policy_grant_detail::ser_create_form_type_policy_grant_detail(&mut object_6, inner)?;
            object_6.finish();
        }
        crate::types::PolicyGrantDetail::CreateAssetType(inner) => {
            #[allow(unused_mut)]
            let mut object_7 = object_3.key("createAssetType").start_object();
            crate::protocol_serde::shape_create_asset_type_policy_grant_detail::ser_create_asset_type_policy_grant_detail(&mut object_7, inner)?;
            object_7.finish();
        }
        crate::types::PolicyGrantDetail::CreateProject(inner) => {
            #[allow(unused_mut)]
            let mut object_8 = object_3.key("createProject").start_object();
            crate::protocol_serde::shape_create_project_policy_grant_detail::ser_create_project_policy_grant_detail(&mut object_8, inner)?;
            object_8.finish();
        }
        crate::types::PolicyGrantDetail::CreateEnvironmentProfile(inner) => {
            #[allow(unused_mut)]
            let mut object_9 = object_3.key("createEnvironmentProfile").start_object();
            crate::protocol_serde::shape_create_environment_profile_policy_grant_detail::ser_create_environment_profile_policy_grant_detail(
                &mut object_9,
                inner,
            )?;
            object_9.finish();
        }
        crate::types::PolicyGrantDetail::DelegateCreateEnvironmentProfile(inner) => {
            #[allow(unused_mut)]
            let mut object_10 = object_3.key("delegateCreateEnvironmentProfile").start_object();
            crate::protocol_serde::shape_unit::ser_unit(&mut object_10, inner)?;
            object_10.finish();
        }
        crate::types::PolicyGrantDetail::CreateEnvironment(inner) => {
            #[allow(unused_mut)]
            let mut object_11 = object_3.key("createEnvironment").start_object();
            crate::protocol_serde::shape_unit::ser_unit(&mut object_11, inner)?;
            object_11.finish();
        }
        crate::types::PolicyGrantDetail::CreateEnvironmentFromBlueprint(inner) => {
            #[allow(unused_mut)]
            let mut object_12 = object_3.key("createEnvironmentFromBlueprint").start_object();
            crate::protocol_serde::shape_unit::ser_unit(&mut object_12, inner)?;
            object_12.finish();
        }
        crate::types::PolicyGrantDetail::CreateProjectFromProjectProfile(inner) => {
            #[allow(unused_mut)]
            let mut object_13 = object_3.key("createProjectFromProjectProfile").start_object();
            crate::protocol_serde::shape_create_project_from_project_profile_policy_grant_detail::ser_create_project_from_project_profile_policy_grant_detail(&mut object_13, inner)?;
            object_13.finish();
        }
        crate::types::PolicyGrantDetail::UseAssetType(inner) => {
            #[allow(unused_mut)]
            let mut object_14 = object_3.key("useAssetType").start_object();
            crate::protocol_serde::shape_use_asset_type_policy_grant_detail::ser_use_asset_type_policy_grant_detail(&mut object_14, inner)?;
            object_14.finish();
        }
        crate::types::PolicyGrantDetail::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "PolicyGrantDetail",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_policy_grant_detail<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::PolicyGrantDetail>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
                        tokens.peek()
                    {
                        let _ = tokens.next().expect("peek returned a token")?;
                        continue;
                    }
                    let key = key.to_unescaped()?;
                    if key == "__type" {
                        ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                        continue;
                    }
                    if variant.is_some() {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            "encountered mixed variants in union",
                        ));
                    }
                    variant = match key.as_ref() {
                            "createDomainUnit" => {
                                Some(crate::types::PolicyGrantDetail::CreateDomainUnit(
                                    crate::protocol_serde::shape_create_domain_unit_policy_grant_detail::de_create_domain_unit_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'createDomainUnit' cannot be null"))?
                                ))
                            }
                            "overrideDomainUnitOwners" => {
                                Some(crate::types::PolicyGrantDetail::OverrideDomainUnitOwners(
                                    crate::protocol_serde::shape_override_domain_unit_owners_policy_grant_detail::de_override_domain_unit_owners_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'overrideDomainUnitOwners' cannot be null"))?
                                ))
                            }
                            "addToProjectMemberPool" => {
                                Some(crate::types::PolicyGrantDetail::AddToProjectMemberPool(
                                    crate::protocol_serde::shape_add_to_project_member_pool_policy_grant_detail::de_add_to_project_member_pool_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'addToProjectMemberPool' cannot be null"))?
                                ))
                            }
                            "overrideProjectOwners" => {
                                Some(crate::types::PolicyGrantDetail::OverrideProjectOwners(
                                    crate::protocol_serde::shape_override_project_owners_policy_grant_detail::de_override_project_owners_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'overrideProjectOwners' cannot be null"))?
                                ))
                            }
                            "createGlossary" => {
                                Some(crate::types::PolicyGrantDetail::CreateGlossary(
                                    crate::protocol_serde::shape_create_glossary_policy_grant_detail::de_create_glossary_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'createGlossary' cannot be null"))?
                                ))
                            }
                            "createFormType" => {
                                Some(crate::types::PolicyGrantDetail::CreateFormType(
                                    crate::protocol_serde::shape_create_form_type_policy_grant_detail::de_create_form_type_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'createFormType' cannot be null"))?
                                ))
                            }
                            "createAssetType" => {
                                Some(crate::types::PolicyGrantDetail::CreateAssetType(
                                    crate::protocol_serde::shape_create_asset_type_policy_grant_detail::de_create_asset_type_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'createAssetType' cannot be null"))?
                                ))
                            }
                            "createProject" => {
                                Some(crate::types::PolicyGrantDetail::CreateProject(
                                    crate::protocol_serde::shape_create_project_policy_grant_detail::de_create_project_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'createProject' cannot be null"))?
                                ))
                            }
                            "createEnvironmentProfile" => {
                                Some(crate::types::PolicyGrantDetail::CreateEnvironmentProfile(
                                    crate::protocol_serde::shape_create_environment_profile_policy_grant_detail::de_create_environment_profile_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'createEnvironmentProfile' cannot be null"))?
                                ))
                            }
                            "delegateCreateEnvironmentProfile" => {
                                Some(crate::types::PolicyGrantDetail::DelegateCreateEnvironmentProfile(
                                    crate::protocol_serde::shape_unit::de_unit(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'delegateCreateEnvironmentProfile' cannot be null"))?
                                ))
                            }
                            "createEnvironment" => {
                                Some(crate::types::PolicyGrantDetail::CreateEnvironment(
                                    crate::protocol_serde::shape_unit::de_unit(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'createEnvironment' cannot be null"))?
                                ))
                            }
                            "createEnvironmentFromBlueprint" => {
                                Some(crate::types::PolicyGrantDetail::CreateEnvironmentFromBlueprint(
                                    crate::protocol_serde::shape_unit::de_unit(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'createEnvironmentFromBlueprint' cannot be null"))?
                                ))
                            }
                            "createProjectFromProjectProfile" => {
                                Some(crate::types::PolicyGrantDetail::CreateProjectFromProjectProfile(
                                    crate::protocol_serde::shape_create_project_from_project_profile_policy_grant_detail::de_create_project_from_project_profile_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'createProjectFromProjectProfile' cannot be null"))?
                                ))
                            }
                            "useAssetType" => {
                                Some(crate::types::PolicyGrantDetail::UseAssetType(
                                    crate::protocol_serde::shape_use_asset_type_policy_grant_detail::de_use_asset_type_policy_grant_detail(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'useAssetType' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                              ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                              Some(crate::types::PolicyGrantDetail::Unknown)
                                                                            }
                        };
                }
                other => {
                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )))
                }
            }
        },
        _ => {
            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ))
        }
    }
    if variant.is_none() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "Union did not contain a valid variant.",
        ));
    }
    Ok(variant)
}
