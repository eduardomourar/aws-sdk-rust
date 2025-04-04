// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_export_job_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ExportJobRequest,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.role_arn {
        object.key("RoleArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.s3_url_prefix {
        object.key("S3UrlPrefix").string(var_2.as_str());
    }
    if let Some(var_3) = &input.segment_id {
        object.key("SegmentId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.segment_version {
        object.key("SegmentVersion").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    Ok(())
}
