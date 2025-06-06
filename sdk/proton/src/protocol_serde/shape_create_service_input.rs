// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_service_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_service::CreateServiceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.template_name {
        object.key("templateName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.template_major_version {
        object.key("templateMajorVersion").string(var_4.as_str());
    }
    if let Some(var_5) = &input.template_minor_version {
        object.key("templateMinorVersion").string(var_5.as_str());
    }
    if let Some(var_6) = &input.spec {
        object.key("spec").string(var_6.as_str());
    }
    if let Some(var_7) = &input.repository_connection_arn {
        object.key("repositoryConnectionArn").string(var_7.as_str());
    }
    if let Some(var_8) = &input.repository_id {
        object.key("repositoryId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.branch_name {
        object.key("branchName").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags {
        let mut array_11 = object.key("tags").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
