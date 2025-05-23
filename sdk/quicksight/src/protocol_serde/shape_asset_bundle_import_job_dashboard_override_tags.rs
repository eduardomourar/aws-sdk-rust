// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_asset_bundle_import_job_dashboard_override_tags(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AssetBundleImportJobDashboardOverrideTags,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("DashboardIds").start_array();
        for item_2 in &input.dashboard_ids {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    {
        let mut array_3 = object.key("Tags").start_array();
        for item_4 in &input.tags {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub(crate) fn de_asset_bundle_import_job_dashboard_override_tags<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AssetBundleImportJobDashboardOverrideTags>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AssetBundleImportJobDashboardOverrideTagsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DashboardIds" => {
                                builder = builder.set_dashboard_ids(
                                    crate::protocol_serde::shape_asset_bundle_restrictive_resource_id_list::de_asset_bundle_restrictive_resource_id_list(tokens)?
                                );
                            }
                            "Tags" => {
                                builder = builder.set_tags(crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?);
                            }
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(
                crate::serde_util::asset_bundle_import_job_dashboard_override_tags_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
