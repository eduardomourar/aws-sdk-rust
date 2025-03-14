// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_bucket_logging_status(
    input: &crate::types::BucketLoggingStatus,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.logging_enabled {
        let inner_writer = scope.start_el("LoggingEnabled");
        crate::protocol_serde::shape_logging_enabled::ser_logging_enabled(var_1, inner_writer)?
    }
    scope.finish();
    Ok(())
}
