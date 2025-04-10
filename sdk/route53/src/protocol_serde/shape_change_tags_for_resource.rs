// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_change_tags_for_resource_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::change_tags_for_resource::ChangeTagsForResourceOutput,
    crate::operation::change_tags_for_resource::ChangeTagsForResourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::change_tags_for_resource::ChangeTagsForResourceError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::change_tags_for_resource::ChangeTagsForResourceError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::change_tags_for_resource::ChangeTagsForResourceError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(_response_body, output)
                    .map_err(crate::operation::change_tags_for_resource::ChangeTagsForResourceError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoSuchHealthCheck" => crate::operation::change_tags_for_resource::ChangeTagsForResourceError::NoSuchHealthCheck({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoSuchHealthCheckBuilder::default();
                output = crate::protocol_serde::shape_no_such_health_check::de_no_such_health_check_xml_err(_response_body, output)
                    .map_err(crate::operation::change_tags_for_resource::ChangeTagsForResourceError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoSuchHostedZone" => crate::operation::change_tags_for_resource::ChangeTagsForResourceError::NoSuchHostedZone({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoSuchHostedZoneBuilder::default();
                output = crate::protocol_serde::shape_no_such_hosted_zone::de_no_such_hosted_zone_xml_err(_response_body, output)
                    .map_err(crate::operation::change_tags_for_resource::ChangeTagsForResourceError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "PriorRequestNotComplete" => crate::operation::change_tags_for_resource::ChangeTagsForResourceError::PriorRequestNotComplete({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::PriorRequestNotCompleteBuilder::default();
                output = crate::protocol_serde::shape_prior_request_not_complete::de_prior_request_not_complete_xml_err(_response_body, output)
                    .map_err(crate::operation::change_tags_for_resource::ChangeTagsForResourceError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottlingException" => crate::operation::change_tags_for_resource::ChangeTagsForResourceError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::change_tags_for_resource::ChangeTagsForResourceError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::change_tags_for_resource::ChangeTagsForResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_change_tags_for_resource_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::change_tags_for_resource::ChangeTagsForResourceOutput,
    crate::operation::change_tags_for_resource::ChangeTagsForResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::change_tags_for_resource::builders::ChangeTagsForResourceOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_change_tags_for_resource_op_input(
    input: &crate::operation::change_tags_for_resource::ChangeTagsForResourceInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("ChangeTagsForResourceRequest")
            .write_ns("https://route53.amazonaws.com/doc/2013-04-01/", None);
        crate::protocol_serde::shape_change_tags_for_resource_input::ser_change_tags_for_resource_input_input_input(input, root)?
    }
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
