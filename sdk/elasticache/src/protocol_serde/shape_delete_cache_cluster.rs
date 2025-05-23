// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_cache_cluster_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_cache_cluster::DeleteCacheClusterOutput,
    crate::operation::delete_cache_cluster::DeleteCacheClusterError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_cache_cluster::DeleteCacheClusterError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_cache_cluster::DeleteCacheClusterError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CacheClusterNotFound" => crate::operation::delete_cache_cluster::DeleteCacheClusterError::CacheClusterNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CacheClusterNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cache_cluster_not_found_fault::de_cache_cluster_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::delete_cache_cluster::DeleteCacheClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidCacheClusterState" => crate::operation::delete_cache_cluster::DeleteCacheClusterError::InvalidCacheClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidCacheClusterStateFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_cache_cluster_state_fault::de_invalid_cache_cluster_state_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_cache_cluster::DeleteCacheClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterCombination" => crate::operation::delete_cache_cluster::DeleteCacheClusterError::InvalidParameterCombinationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterCombinationExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_combination_exception::de_invalid_parameter_combination_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_cache_cluster::DeleteCacheClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterValue" => crate::operation::delete_cache_cluster::DeleteCacheClusterError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_cache_cluster::DeleteCacheClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SnapshotAlreadyExistsFault" => crate::operation::delete_cache_cluster::DeleteCacheClusterError::SnapshotAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SnapshotAlreadyExistsFaultBuilder::default();
                output = crate::protocol_serde::shape_snapshot_already_exists_fault::de_snapshot_already_exists_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::delete_cache_cluster::DeleteCacheClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SnapshotFeatureNotSupportedFault" => crate::operation::delete_cache_cluster::DeleteCacheClusterError::SnapshotFeatureNotSupportedFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SnapshotFeatureNotSupportedFaultBuilder::default();
                output = crate::protocol_serde::shape_snapshot_feature_not_supported_fault::de_snapshot_feature_not_supported_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_cache_cluster::DeleteCacheClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SnapshotQuotaExceededFault" => crate::operation::delete_cache_cluster::DeleteCacheClusterError::SnapshotQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SnapshotQuotaExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_snapshot_quota_exceeded_fault::de_snapshot_quota_exceeded_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::delete_cache_cluster::DeleteCacheClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_cache_cluster::DeleteCacheClusterError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_cache_cluster_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_cache_cluster::DeleteCacheClusterOutput,
    crate::operation::delete_cache_cluster::DeleteCacheClusterError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_cache_cluster::builders::DeleteCacheClusterOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_cache_cluster::de_delete_cache_cluster(_response_body, output)
            .map_err(crate::operation::delete_cache_cluster::DeleteCacheClusterError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_cache_cluster(
    inp: &[u8],
    mut builder: crate::operation::delete_cache_cluster::builders::DeleteCacheClusterOutputBuilder,
) -> std::result::Result<crate::operation::delete_cache_cluster::builders::DeleteCacheClusterOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteCacheClusterResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteCacheClusterResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DeleteCacheClusterResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DeleteCacheClusterResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("CacheCluster") /* CacheCluster com.amazonaws.elasticache.synthetic#DeleteCacheClusterOutput$CacheCluster */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_cache_cluster::de_cache_cluster(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cache_cluster(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected DeleteCacheClusterResult tag"));
    };
    Ok(builder)
}
