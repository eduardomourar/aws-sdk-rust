// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_reserved_cache_nodes_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesOutput,
    crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidParameterCombination" => {
            crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError::InvalidParameterCombinationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterCombinationExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_parameter_combination_exception::de_invalid_parameter_combination_exception_xml_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterValue" => {
            crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError::InvalidParameterValueException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ReservedCacheNodeNotFound" => {
            crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError::ReservedCacheNodeNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ReservedCacheNodeNotFoundFaultBuilder::default();
                    output = crate::protocol_serde::shape_reserved_cache_node_not_found_fault::de_reserved_cache_node_not_found_fault_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_reserved_cache_nodes_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesOutput,
    crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_reserved_cache_nodes::builders::DescribeReservedCacheNodesOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_reserved_cache_nodes::de_describe_reserved_cache_nodes(_response_body, output)
            .map_err(crate::operation::describe_reserved_cache_nodes::DescribeReservedCacheNodesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_reserved_cache_nodes(
    inp: &[u8],
    mut builder: crate::operation::describe_reserved_cache_nodes::builders::DescribeReservedCacheNodesOutputBuilder,
) -> std::result::Result<
    crate::operation::describe_reserved_cache_nodes::builders::DescribeReservedCacheNodesOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeReservedCacheNodesResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeReservedCacheNodesResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DescribeReservedCacheNodesResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DescribeReservedCacheNodesResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Marker") /* Marker com.amazonaws.elasticache.synthetic#DescribeReservedCacheNodesOutput$Marker */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_marker(var_1);
            }
            ,
            s if s.matches("ReservedCacheNodes") /* ReservedCacheNodes com.amazonaws.elasticache.synthetic#DescribeReservedCacheNodesOutput$ReservedCacheNodes */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_reserved_cache_node_list::de_reserved_cache_node_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_reserved_cache_nodes(var_2);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected DescribeReservedCacheNodesResult tag",
        ));
    };
    Ok(builder)
}
