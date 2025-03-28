// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_user_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UserSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.execution_role {
        object.key("ExecutionRole").string(var_1.as_str());
    }
    if let Some(var_2) = &input.security_groups {
        let mut array_3 = object.key("SecurityGroups").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.sharing_settings {
        #[allow(unused_mut)]
        let mut object_6 = object.key("SharingSettings").start_object();
        crate::protocol_serde::shape_sharing_settings::ser_sharing_settings(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.jupyter_server_app_settings {
        #[allow(unused_mut)]
        let mut object_8 = object.key("JupyterServerAppSettings").start_object();
        crate::protocol_serde::shape_jupyter_server_app_settings::ser_jupyter_server_app_settings(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.kernel_gateway_app_settings {
        #[allow(unused_mut)]
        let mut object_10 = object.key("KernelGatewayAppSettings").start_object();
        crate::protocol_serde::shape_kernel_gateway_app_settings::ser_kernel_gateway_app_settings(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.tensor_board_app_settings {
        #[allow(unused_mut)]
        let mut object_12 = object.key("TensorBoardAppSettings").start_object();
        crate::protocol_serde::shape_tensor_board_app_settings::ser_tensor_board_app_settings(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.r_studio_server_pro_app_settings {
        #[allow(unused_mut)]
        let mut object_14 = object.key("RStudioServerProAppSettings").start_object();
        crate::protocol_serde::shape_r_studio_server_pro_app_settings::ser_r_studio_server_pro_app_settings(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.r_session_app_settings {
        #[allow(unused_mut)]
        let mut object_16 = object.key("RSessionAppSettings").start_object();
        crate::protocol_serde::shape_r_session_app_settings::ser_r_session_app_settings(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.canvas_app_settings {
        #[allow(unused_mut)]
        let mut object_18 = object.key("CanvasAppSettings").start_object();
        crate::protocol_serde::shape_canvas_app_settings::ser_canvas_app_settings(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.code_editor_app_settings {
        #[allow(unused_mut)]
        let mut object_20 = object.key("CodeEditorAppSettings").start_object();
        crate::protocol_serde::shape_code_editor_app_settings::ser_code_editor_app_settings(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.jupyter_lab_app_settings {
        #[allow(unused_mut)]
        let mut object_22 = object.key("JupyterLabAppSettings").start_object();
        crate::protocol_serde::shape_jupyter_lab_app_settings::ser_jupyter_lab_app_settings(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.space_storage_settings {
        #[allow(unused_mut)]
        let mut object_24 = object.key("SpaceStorageSettings").start_object();
        crate::protocol_serde::shape_default_space_storage_settings::ser_default_space_storage_settings(&mut object_24, var_23)?;
        object_24.finish();
    }
    if let Some(var_25) = &input.default_landing_uri {
        object.key("DefaultLandingUri").string(var_25.as_str());
    }
    if let Some(var_26) = &input.studio_web_portal {
        object.key("StudioWebPortal").string(var_26.as_str());
    }
    if let Some(var_27) = &input.custom_posix_user_config {
        #[allow(unused_mut)]
        let mut object_28 = object.key("CustomPosixUserConfig").start_object();
        crate::protocol_serde::shape_custom_posix_user_config::ser_custom_posix_user_config(&mut object_28, var_27)?;
        object_28.finish();
    }
    if let Some(var_29) = &input.custom_file_system_configs {
        let mut array_30 = object.key("CustomFileSystemConfigs").start_array();
        for item_31 in var_29 {
            {
                #[allow(unused_mut)]
                let mut object_32 = array_30.value().start_object();
                crate::protocol_serde::shape_custom_file_system_config::ser_custom_file_system_config(&mut object_32, item_31)?;
                object_32.finish();
            }
        }
        array_30.finish();
    }
    if let Some(var_33) = &input.studio_web_portal_settings {
        #[allow(unused_mut)]
        let mut object_34 = object.key("StudioWebPortalSettings").start_object();
        crate::protocol_serde::shape_studio_web_portal_settings::ser_studio_web_portal_settings(&mut object_34, var_33)?;
        object_34.finish();
    }
    if let Some(var_35) = &input.auto_mount_home_efs {
        object.key("AutoMountHomeEFS").string(var_35.as_str());
    }
    Ok(())
}

pub(crate) fn de_user_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::UserSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::UserSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ExecutionRole" => {
                            builder = builder.set_execution_role(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "SecurityGroups" => {
                            builder = builder.set_security_groups(crate::protocol_serde::shape_security_group_ids::de_security_group_ids(tokens)?);
                        }
                        "SharingSettings" => {
                            builder = builder.set_sharing_settings(crate::protocol_serde::shape_sharing_settings::de_sharing_settings(tokens)?);
                        }
                        "JupyterServerAppSettings" => {
                            builder = builder.set_jupyter_server_app_settings(
                                crate::protocol_serde::shape_jupyter_server_app_settings::de_jupyter_server_app_settings(tokens)?,
                            );
                        }
                        "KernelGatewayAppSettings" => {
                            builder = builder.set_kernel_gateway_app_settings(
                                crate::protocol_serde::shape_kernel_gateway_app_settings::de_kernel_gateway_app_settings(tokens)?,
                            );
                        }
                        "TensorBoardAppSettings" => {
                            builder = builder.set_tensor_board_app_settings(
                                crate::protocol_serde::shape_tensor_board_app_settings::de_tensor_board_app_settings(tokens)?,
                            );
                        }
                        "RStudioServerProAppSettings" => {
                            builder = builder.set_r_studio_server_pro_app_settings(
                                crate::protocol_serde::shape_r_studio_server_pro_app_settings::de_r_studio_server_pro_app_settings(tokens)?,
                            );
                        }
                        "RSessionAppSettings" => {
                            builder = builder
                                .set_r_session_app_settings(crate::protocol_serde::shape_r_session_app_settings::de_r_session_app_settings(tokens)?);
                        }
                        "CanvasAppSettings" => {
                            builder =
                                builder.set_canvas_app_settings(crate::protocol_serde::shape_canvas_app_settings::de_canvas_app_settings(tokens)?);
                        }
                        "CodeEditorAppSettings" => {
                            builder = builder.set_code_editor_app_settings(
                                crate::protocol_serde::shape_code_editor_app_settings::de_code_editor_app_settings(tokens)?,
                            );
                        }
                        "JupyterLabAppSettings" => {
                            builder = builder.set_jupyter_lab_app_settings(
                                crate::protocol_serde::shape_jupyter_lab_app_settings::de_jupyter_lab_app_settings(tokens)?,
                            );
                        }
                        "SpaceStorageSettings" => {
                            builder = builder.set_space_storage_settings(
                                crate::protocol_serde::shape_default_space_storage_settings::de_default_space_storage_settings(tokens)?,
                            );
                        }
                        "DefaultLandingUri" => {
                            builder = builder.set_default_landing_uri(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "StudioWebPortal" => {
                            builder = builder.set_studio_web_portal(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::StudioWebPortal::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "CustomPosixUserConfig" => {
                            builder = builder.set_custom_posix_user_config(
                                crate::protocol_serde::shape_custom_posix_user_config::de_custom_posix_user_config(tokens)?,
                            );
                        }
                        "CustomFileSystemConfigs" => {
                            builder = builder.set_custom_file_system_configs(
                                crate::protocol_serde::shape_custom_file_system_configs::de_custom_file_system_configs(tokens)?,
                            );
                        }
                        "StudioWebPortalSettings" => {
                            builder = builder.set_studio_web_portal_settings(
                                crate::protocol_serde::shape_studio_web_portal_settings::de_studio_web_portal_settings(tokens)?,
                            );
                        }
                        "AutoMountHomeEFS" => {
                            builder = builder.set_auto_mount_home_efs(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AutoMountHomeEfs::from(u.as_ref())))
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
