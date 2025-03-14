// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_realtime_log_config_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigOutput,
    crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedBuilder::default();
                output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(_response_body, output)
                    .map_err(crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArgument" => crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArgumentBuilder::default();
                output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(_response_body, output)
                    .map_err(crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoSuchRealtimeLogConfig" => crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError::NoSuchRealtimeLogConfig({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoSuchRealtimeLogConfigBuilder::default();
                output = crate::protocol_serde::shape_no_such_realtime_log_config::de_no_such_realtime_log_config_xml_err(_response_body, output)
                    .map_err(crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_realtime_log_config_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigOutput,
    crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigOutputBuilder::default();
        output = crate::protocol_serde::shape_update_realtime_log_config::de_update_realtime_log_config(_response_body, output)
            .map_err(crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_update_realtime_log_config_op_input(
    input: &crate::operation::update_realtime_log_config::UpdateRealtimeLogConfigInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("UpdateRealtimeLogConfigRequest")
            .write_ns("http://cloudfront.amazonaws.com/doc/2020-05-31/", None);
        crate::protocol_serde::shape_update_realtime_log_config_input::ser_update_realtime_log_config_input_input_input(input, root)?
    }
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

#[allow(unused_mut)]
pub fn de_update_realtime_log_config(
    inp: &[u8],
    mut builder: crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigOutputBuilder,
) -> std::result::Result<
    crate::operation::update_realtime_log_config::builders::UpdateRealtimeLogConfigOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("UpdateRealtimeLogConfigResult") {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "encountered invalid XML root: expected UpdateRealtimeLogConfigResult but got {:?}. This is likely a bug in the SDK.",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("RealtimeLogConfig") /* RealtimeLogConfig com.amazonaws.cloudfront.synthetic#UpdateRealtimeLogConfigOutput$RealtimeLogConfig */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_realtime_log_config::de_realtime_log_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_realtime_log_config(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
