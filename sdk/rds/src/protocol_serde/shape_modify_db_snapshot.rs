// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_db_snapshot_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::modify_db_snapshot::ModifyDbSnapshotOutput, crate::operation::modify_db_snapshot::ModifyDBSnapshotError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_db_snapshot::ModifyDBSnapshotError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::modify_db_snapshot::ModifyDBSnapshotError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DBSnapshotNotFound" => crate::operation::modify_db_snapshot::ModifyDBSnapshotError::DbSnapshotNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DbSnapshotNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_db_snapshot_not_found_fault::de_db_snapshot_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::modify_db_snapshot::ModifyDBSnapshotError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::modify_db_snapshot::ModifyDBSnapshotError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_db_snapshot_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::modify_db_snapshot::ModifyDbSnapshotOutput, crate::operation::modify_db_snapshot::ModifyDBSnapshotError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_db_snapshot::builders::ModifyDbSnapshotOutputBuilder::default();
        output = crate::protocol_serde::shape_modify_db_snapshot::de_modify_db_snapshot(_response_body, output)
            .map_err(crate::operation::modify_db_snapshot::ModifyDBSnapshotError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_db_snapshot(
    inp: &[u8],
    mut builder: crate::operation::modify_db_snapshot::builders::ModifyDbSnapshotOutputBuilder,
) -> std::result::Result<crate::operation::modify_db_snapshot::builders::ModifyDbSnapshotOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyDBSnapshotResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyDBSnapshotResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ModifyDBSnapshotResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ModifyDBSnapshotResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("DBSnapshot") /* DBSnapshot com.amazonaws.rds.synthetic#ModifyDBSnapshotOutput$DBSnapshot */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_db_snapshot::de_db_snapshot(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_snapshot(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected ModifyDBSnapshotResult tag"));
    };
    Ok(builder)
}
