// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_generated_template_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_generated_template::UpdateGeneratedTemplateOutput,
    crate::operation::update_generated_template::UpdateGeneratedTemplateError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_generated_template::UpdateGeneratedTemplateError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::update_generated_template::UpdateGeneratedTemplateError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AlreadyExistsException" => crate::operation::update_generated_template::UpdateGeneratedTemplateError::AlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AlreadyExistsExceptionBuilder::default();
                output = crate::protocol_serde::shape_already_exists_exception::de_already_exists_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::update_generated_template::UpdateGeneratedTemplateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "GeneratedTemplateNotFound" => {
            crate::operation::update_generated_template::UpdateGeneratedTemplateError::GeneratedTemplateNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::GeneratedTemplateNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_generated_template_not_found_exception::de_generated_template_not_found_exception_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::update_generated_template::UpdateGeneratedTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "LimitExceededException" => crate::operation::update_generated_template::UpdateGeneratedTemplateError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::update_generated_template::UpdateGeneratedTemplateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::update_generated_template::UpdateGeneratedTemplateError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_generated_template_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_generated_template::UpdateGeneratedTemplateOutput,
    crate::operation::update_generated_template::UpdateGeneratedTemplateError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_generated_template::builders::UpdateGeneratedTemplateOutputBuilder::default();
        output = crate::protocol_serde::shape_update_generated_template::de_update_generated_template(_response_body, output)
            .map_err(crate::operation::update_generated_template::UpdateGeneratedTemplateError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_update_generated_template(
    inp: &[u8],
    mut builder: crate::operation::update_generated_template::builders::UpdateGeneratedTemplateOutputBuilder,
) -> std::result::Result<
    crate::operation::update_generated_template::builders::UpdateGeneratedTemplateOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("UpdateGeneratedTemplateResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected UpdateGeneratedTemplateResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("UpdateGeneratedTemplateResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected UpdateGeneratedTemplateResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("GeneratedTemplateId") /* GeneratedTemplateId com.amazonaws.cloudformation.synthetic#UpdateGeneratedTemplateOutput$GeneratedTemplateId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_generated_template_id(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected UpdateGeneratedTemplateResult tag",
        ));
    };
    Ok(builder)
}
