// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_image_scan_findings_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_image_scan_findings::DescribeImageScanFindingsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.registry_id {
        object.key("registryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.repository_name {
        object.key("repositoryName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.image_id {
        #[allow(unused_mut)]
        let mut object_4 = object.key("imageId").start_object();
        crate::protocol_serde::shape_image_identifier::ser_image_identifier(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.next_token {
        object.key("nextToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    Ok(())
}
