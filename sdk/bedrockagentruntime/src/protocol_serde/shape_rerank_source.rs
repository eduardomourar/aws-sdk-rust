// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rerank_source(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RerankSource,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("type").string(input.r#type.as_str());
    }
    if let Some(var_1) = &input.inline_document_source {
        #[allow(unused_mut)]
        let mut object_2 = object.key("inlineDocumentSource").start_object();
        crate::protocol_serde::shape_rerank_document::ser_rerank_document(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
