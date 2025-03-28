// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_reboot_db_instance_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::reboot_db_instance::RebootDbInstanceOutput, crate::operation::reboot_db_instance::RebootDBInstanceError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::reboot_db_instance::RebootDBInstanceError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::reboot_db_instance::RebootDBInstanceError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DBInstanceNotFound" => crate::operation::reboot_db_instance::RebootDBInstanceError::DbInstanceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DbInstanceNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_db_instance_not_found_fault::de_db_instance_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::reboot_db_instance::RebootDBInstanceError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidDBInstanceState" => crate::operation::reboot_db_instance::RebootDBInstanceError::InvalidDbInstanceStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidDbInstanceStateFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_db_instance_state_fault::de_invalid_db_instance_state_fault_xml_err(_response_body, output)
                        .map_err(crate::operation::reboot_db_instance::RebootDBInstanceError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::reboot_db_instance::RebootDBInstanceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_reboot_db_instance_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::reboot_db_instance::RebootDbInstanceOutput, crate::operation::reboot_db_instance::RebootDBInstanceError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::reboot_db_instance::builders::RebootDbInstanceOutputBuilder::default();
        output = crate::protocol_serde::shape_reboot_db_instance::de_reboot_db_instance(_response_body, output)
            .map_err(crate::operation::reboot_db_instance::RebootDBInstanceError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_reboot_db_instance(
    inp: &[u8],
    mut builder: crate::operation::reboot_db_instance::builders::RebootDbInstanceOutputBuilder,
) -> std::result::Result<crate::operation::reboot_db_instance::builders::RebootDbInstanceOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("RebootDBInstanceResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected RebootDBInstanceResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("RebootDBInstanceResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected RebootDBInstanceResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("DBInstance") /* DBInstance com.amazonaws.docdb.synthetic#RebootDBInstanceOutput$DBInstance */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_db_instance::de_db_instance(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_instance(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected RebootDBInstanceResult tag"));
    };
    Ok(builder)
}
