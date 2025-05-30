// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_availability_options_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_availability_options::DescribeAvailabilityOptionsOutput,
    crate::operation::describe_availability_options::DescribeAvailabilityOptionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BaseException" => crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::BaseException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BaseExceptionBuilder::default();
                output = crate::protocol_serde::shape_base_exception::de_base_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DisabledAction" => crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::DisabledOperationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DisabledOperationExceptionBuilder::default();
                output = crate::protocol_serde::shape_disabled_operation_exception::de_disabled_operation_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalException" => crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::InternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_exception::de_internal_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidType" => crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::InvalidTypeException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidTypeExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_type_exception::de_invalid_type_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "LimitExceeded" => crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFound" => crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_availability_options_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_availability_options::DescribeAvailabilityOptionsOutput,
    crate::operation::describe_availability_options::DescribeAvailabilityOptionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_availability_options::builders::DescribeAvailabilityOptionsOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_availability_options::de_describe_availability_options(_response_body, output)
            .map_err(crate::operation::describe_availability_options::DescribeAvailabilityOptionsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_availability_options(
    inp: &[u8],
    mut builder: crate::operation::describe_availability_options::builders::DescribeAvailabilityOptionsOutputBuilder,
) -> std::result::Result<
    crate::operation::describe_availability_options::builders::DescribeAvailabilityOptionsOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeAvailabilityOptionsResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeAvailabilityOptionsResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DescribeAvailabilityOptionsResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DescribeAvailabilityOptionsResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("AvailabilityOptions") /* AvailabilityOptions com.amazonaws.cloudsearch.synthetic#DescribeAvailabilityOptionsOutput$AvailabilityOptions */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_availability_options_status::de_availability_options_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_options(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected DescribeAvailabilityOptionsResult tag",
        ));
    };
    Ok(builder)
}
