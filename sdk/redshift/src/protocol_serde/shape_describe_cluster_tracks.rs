// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_cluster_tracks_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_cluster_tracks::DescribeClusterTracksOutput,
    crate::operation::describe_cluster_tracks::DescribeClusterTracksError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_cluster_tracks::DescribeClusterTracksError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_cluster_tracks::DescribeClusterTracksError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidClusterTrack" => crate::operation::describe_cluster_tracks::DescribeClusterTracksError::InvalidClusterTrackFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidClusterTrackFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_cluster_track_fault::de_invalid_cluster_track_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_cluster_tracks::DescribeClusterTracksError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnauthorizedOperation" => crate::operation::describe_cluster_tracks::DescribeClusterTracksError::UnauthorizedOperation({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnauthorizedOperationBuilder::default();
                output = crate::protocol_serde::shape_unauthorized_operation::de_unauthorized_operation_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_cluster_tracks::DescribeClusterTracksError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_cluster_tracks::DescribeClusterTracksError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_cluster_tracks_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_cluster_tracks::DescribeClusterTracksOutput,
    crate::operation::describe_cluster_tracks::DescribeClusterTracksError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_cluster_tracks::builders::DescribeClusterTracksOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_cluster_tracks::de_describe_cluster_tracks(_response_body, output)
            .map_err(crate::operation::describe_cluster_tracks::DescribeClusterTracksError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_cluster_tracks(
    inp: &[u8],
    mut builder: crate::operation::describe_cluster_tracks::builders::DescribeClusterTracksOutputBuilder,
) -> std::result::Result<
    crate::operation::describe_cluster_tracks::builders::DescribeClusterTracksOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeClusterTracksResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeClusterTracksResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DescribeClusterTracksResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DescribeClusterTracksResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("MaintenanceTracks") /* MaintenanceTracks com.amazonaws.redshift.synthetic#DescribeClusterTracksOutput$MaintenanceTracks */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_track_list::de_track_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_maintenance_tracks(var_1);
            }
            ,
            s if s.matches("Marker") /* Marker com.amazonaws.redshift.synthetic#DescribeClusterTracksOutput$Marker */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_marker(var_2);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected DescribeClusterTracksResult tag",
        ));
    };
    Ok(builder)
}
