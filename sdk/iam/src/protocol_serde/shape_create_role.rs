// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_role_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::create_role::CreateRoleOutput, crate::operation::create_role::CreateRoleError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_role::CreateRoleError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::create_role::CreateRoleError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModification" => crate::operation::create_role::CreateRoleError::ConcurrentModificationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConcurrentModificationExceptionBuilder::default();
                output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_role::CreateRoleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EntityAlreadyExists" => crate::operation::create_role::CreateRoleError::EntityAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EntityAlreadyExistsExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_entity_already_exists_exception::de_entity_already_exists_exception_xml_err(_response_body, output)
                        .map_err(crate::operation::create_role::CreateRoleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidInput" => crate::operation::create_role::CreateRoleError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::create_role::CreateRoleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "LimitExceeded" => crate::operation::create_role::CreateRoleError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::create_role::CreateRoleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "MalformedPolicyDocument" => crate::operation::create_role::CreateRoleError::MalformedPolicyDocumentException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
                output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_role::CreateRoleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceFailure" => crate::operation::create_role::CreateRoleError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::create_role::CreateRoleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::create_role::CreateRoleError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_role_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::create_role::CreateRoleOutput, crate::operation::create_role::CreateRoleError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_role::builders::CreateRoleOutputBuilder::default();
        output = crate::protocol_serde::shape_create_role::de_create_role(_response_body, output)
            .map_err(crate::operation::create_role::CreateRoleError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::create_role_output_output_correct_errors(output).build()
    })
}

#[allow(unused_mut)]
pub fn de_create_role(
    inp: &[u8],
    mut builder: crate::operation::create_role::builders::CreateRoleOutputBuilder,
) -> std::result::Result<crate::operation::create_role::builders::CreateRoleOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateRoleResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateRoleResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("CreateRoleResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected CreateRoleResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Role") /* Role com.amazonaws.iam.synthetic#CreateRoleOutput$Role */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_role::de_role(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_role(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected CreateRoleResult tag"));
    };
    Ok(builder)
}
