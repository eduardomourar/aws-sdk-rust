// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_continuous_deployment_policy_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::ContinuousDeploymentPolicy>,
    crate::operation::get_continuous_deployment_policy::GetContinuousDeploymentPolicyError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_get_continuous_deployment_policy_output::de_continuous_deployment_policy(body)
                .map_err(crate::operation::get_continuous_deployment_policy::GetContinuousDeploymentPolicyError::unhandled)
        })
        .transpose()
}

pub(crate) fn de_e_tag_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("ETag");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub fn de_continuous_deployment_policy(
    inp: &[u8],
) -> std::result::Result<crate::types::ContinuousDeploymentPolicy, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("ContinuousDeploymentPolicy")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ContinuousDeploymentPolicy got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_continuous_deployment_policy::de_continuous_deployment_policy(&mut decoder)
}
