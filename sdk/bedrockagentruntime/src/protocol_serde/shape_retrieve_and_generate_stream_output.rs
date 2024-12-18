// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_session_id_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amzn-bedrock-knowledge-base-session-id");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub fn de_stream_payload(
    body: &mut ::aws_smithy_types::body::SdkBody,
) -> std::result::Result<
    crate::event_receiver::EventReceiver<
        crate::types::RetrieveAndGenerateStreamResponseOutput,
        crate::types::error::RetrieveAndGenerateStreamResponseOutputError,
    >,
    crate::operation::retrieve_and_generate_stream::RetrieveAndGenerateStreamError,
> {
    let unmarshaller = crate::event_stream_serde::RetrieveAndGenerateStreamResponseOutputUnmarshaller::new();
    let body = std::mem::replace(body, ::aws_smithy_types::body::SdkBody::taken());
    Ok(crate::event_receiver::EventReceiver::new(::aws_smithy_http::event_stream::Receiver::new(
        unmarshaller,
        body,
    )))
}
