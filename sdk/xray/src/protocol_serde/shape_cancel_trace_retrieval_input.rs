// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cancel_trace_retrieval_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::cancel_trace_retrieval::CancelTraceRetrievalInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.retrieval_token {
        object.key("RetrievalToken").string(var_1.as_str());
    }
    Ok(())
}
