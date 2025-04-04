// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_key_signing_key_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_key_signing_key::CreateKeySigningKeyOutput,
    crate::operation::create_key_signing_key::CreateKeySigningKeyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModification" => crate::operation::create_key_signing_key::CreateKeySigningKeyError::ConcurrentModification({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConcurrentModificationBuilder::default();
                output = crate::protocol_serde::shape_concurrent_modification::de_concurrent_modification_xml_err(_response_body, output)
                    .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArgument" => crate::operation::create_key_signing_key::CreateKeySigningKeyError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArgumentBuilder::default();
                output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(_response_body, output)
                    .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidInput" => crate::operation::create_key_signing_key::CreateKeySigningKeyError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(_response_body, output)
                    .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidKeySigningKeyName" => crate::operation::create_key_signing_key::CreateKeySigningKeyError::InvalidKeySigningKeyName({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidKeySigningKeyNameBuilder::default();
                output = crate::protocol_serde::shape_invalid_key_signing_key_name::de_invalid_key_signing_key_name_xml_err(_response_body, output)
                    .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidKeySigningKeyStatus" => crate::operation::create_key_signing_key::CreateKeySigningKeyError::InvalidKeySigningKeyStatus({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidKeySigningKeyStatusBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_key_signing_key_status::de_invalid_key_signing_key_status_xml_err(_response_body, output)
                        .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidKMSArn" => crate::operation::create_key_signing_key::CreateKeySigningKeyError::InvalidKmsArn({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidKmsArnBuilder::default();
                output = crate::protocol_serde::shape_invalid_kms_arn::de_invalid_kms_arn_xml_err(_response_body, output)
                    .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidSigningStatus" => crate::operation::create_key_signing_key::CreateKeySigningKeyError::InvalidSigningStatus({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidSigningStatusBuilder::default();
                output = crate::protocol_serde::shape_invalid_signing_status::de_invalid_signing_status_xml_err(_response_body, output)
                    .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KeySigningKeyAlreadyExists" => crate::operation::create_key_signing_key::CreateKeySigningKeyError::KeySigningKeyAlreadyExists({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KeySigningKeyAlreadyExistsBuilder::default();
                output =
                    crate::protocol_serde::shape_key_signing_key_already_exists::de_key_signing_key_already_exists_xml_err(_response_body, output)
                        .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoSuchHostedZone" => crate::operation::create_key_signing_key::CreateKeySigningKeyError::NoSuchHostedZone({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoSuchHostedZoneBuilder::default();
                output = crate::protocol_serde::shape_no_such_hosted_zone::de_no_such_hosted_zone_xml_err(_response_body, output)
                    .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TooManyKeySigningKeys" => crate::operation::create_key_signing_key::CreateKeySigningKeyError::TooManyKeySigningKeys({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TooManyKeySigningKeysBuilder::default();
                output = crate::protocol_serde::shape_too_many_key_signing_keys::de_too_many_key_signing_keys_xml_err(_response_body, output)
                    .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::create_key_signing_key::CreateKeySigningKeyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_key_signing_key_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_key_signing_key::CreateKeySigningKeyOutput,
    crate::operation::create_key_signing_key::CreateKeySigningKeyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_key_signing_key::builders::CreateKeySigningKeyOutputBuilder::default();
        output = crate::protocol_serde::shape_create_key_signing_key::de_create_key_signing_key(_response_body, output)
            .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?;
        output = output.set_location(
            crate::protocol_serde::shape_create_key_signing_key_output::de_location_header(_response_headers).map_err(|_| {
                crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled("Failed to parse Location from header `Location")
            })?,
        );
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::create_key_signing_key_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::create_key_signing_key::CreateKeySigningKeyError::unhandled)?
    })
}

pub fn ser_create_key_signing_key_op_input(
    input: &crate::operation::create_key_signing_key::CreateKeySigningKeyInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("CreateKeySigningKeyRequest")
            .write_ns("https://route53.amazonaws.com/doc/2013-04-01/", None);
        crate::protocol_serde::shape_create_key_signing_key_input::ser_create_key_signing_key_input_input_input(input, root)?
    }
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

#[allow(unused_mut)]
pub fn de_create_key_signing_key(
    inp: &[u8],
    mut builder: crate::operation::create_key_signing_key::builders::CreateKeySigningKeyOutputBuilder,
) -> std::result::Result<crate::operation::create_key_signing_key::builders::CreateKeySigningKeyOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("CreateKeySigningKeyResponse") {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "encountered invalid XML root: expected CreateKeySigningKeyResponse but got {:?}. This is likely a bug in the SDK.",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ChangeInfo") /* ChangeInfo com.amazonaws.route53.synthetic#CreateKeySigningKeyOutput$ChangeInfo */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_change_info::de_change_info(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_change_info(var_1);
            }
            ,
            s if s.matches("KeySigningKey") /* KeySigningKey com.amazonaws.route53.synthetic#CreateKeySigningKeyOutput$KeySigningKey */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_key_signing_key::de_key_signing_key(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_key_signing_key(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
