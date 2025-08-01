// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_reputation_entity_customer_managed_status_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusOutput,
    crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled(
                    generic,
                ),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "ConflictException" => crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output).map_err(crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        _ => crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_reputation_entity_customer_managed_status_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusOutput,
    crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_reputation_entity_customer_managed_status::builders::UpdateReputationEntityCustomerManagedStatusOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_update_reputation_entity_customer_managed_status_input(
    input: &crate::operation::update_reputation_entity_customer_managed_status::UpdateReputationEntityCustomerManagedStatusInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_reputation_entity_customer_managed_status_input::ser_update_reputation_entity_customer_managed_status_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
