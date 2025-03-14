// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_permission_associations_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_permission_associations::ListPermissionAssociationsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.association_status {
        object.key("associationStatus").string(var_1.as_str());
    }
    if let Some(var_2) = &input.default_version {
        object.key("defaultVersion").boolean(*var_2);
    }
    if let Some(var_3) = &input.feature_set {
        object.key("featureSet").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.next_token {
        object.key("nextToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.permission_arn {
        object.key("permissionArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.permission_version {
        object.key("permissionVersion").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.resource_type {
        object.key("resourceType").string(var_8.as_str());
    }
    Ok(())
}
