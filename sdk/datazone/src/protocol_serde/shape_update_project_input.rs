// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_project_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_project::UpdateProjectInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.domain_unit_id {
        object.key("domainUnitId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.environment_deployment_details {
        #[allow(unused_mut)]
        let mut object_4 = object.key("environmentDeploymentDetails").start_object();
        crate::protocol_serde::shape_environment_deployment_details::ser_environment_deployment_details(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.glossary_terms {
        let mut array_6 = object.key("glossaryTerms").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.project_profile_version {
        object.key("projectProfileVersion").string(var_9.as_str());
    }
    if let Some(var_10) = &input.user_parameters {
        let mut array_11 = object.key("userParameters").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_environment_configuration_user_parameter::ser_environment_configuration_user_parameter(
                    &mut object_13,
                    item_12,
                )?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
