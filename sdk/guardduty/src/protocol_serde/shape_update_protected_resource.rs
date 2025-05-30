// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_protected_resource(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateProtectedResource,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.s3_bucket {
        #[allow(unused_mut)]
        let mut object_2 = object.key("s3Bucket").start_object();
        crate::protocol_serde::shape_update_s3_bucket_resource::ser_update_s3_bucket_resource(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
