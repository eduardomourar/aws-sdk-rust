// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_direct_query_data_source_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceOutput,
    crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BaseException" => crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::BaseException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BaseExceptionBuilder::default();
                output = crate::protocol_serde::shape_base_exception::de_base_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DisabledOperationException" => {
            crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::DisabledOperationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DisabledOperationExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_disabled_operation_exception::de_disabled_operation_exception_json_err(_response_body, output)
                            .map_err(crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalException" => crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::InternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ValidationException" => crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_direct_query_data_source_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceOutput,
    crate::operation::delete_direct_query_data_source::DeleteDirectQueryDataSourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_direct_query_data_source::builders::DeleteDirectQueryDataSourceOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
