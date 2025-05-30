// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_access_grant_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::delete_access_grant::DeleteAccessGrantOutput, crate::operation::delete_access_grant::DeleteAccessGrantError>
{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_access_grant::DeleteAccessGrantError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::delete_access_grant::DeleteAccessGrantError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_access_grant_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::delete_access_grant::DeleteAccessGrantOutput, crate::operation::delete_access_grant::DeleteAccessGrantError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_access_grant::builders::DeleteAccessGrantOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_delete_access_grant_headers(
    input: &crate::operation::delete_access_grant::DeleteAccessGrantInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.account_id {
        let formatted_2 = inner_1.as_str();
        let header_value = formatted_2;
        let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
            ::aws_smithy_types::error::operation::BuildError::invalid_field(
                "account_id",
                format!("`{}` cannot be used as a header value: {}", &header_value, err),
            )
        })?;
        builder = builder.header("x-amz-account-id", header_value);
    }
    Ok(builder)
}
