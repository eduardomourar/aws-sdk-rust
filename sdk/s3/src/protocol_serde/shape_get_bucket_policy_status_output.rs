// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_policy_status_payload(
    body: &[u8],
) -> std::result::Result<::std::option::Option<crate::types::PolicyStatus>, crate::operation::get_bucket_policy_status::GetBucketPolicyStatusError> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_get_bucket_policy_status_output::de_policy_status(body)
                .map_err(crate::operation::get_bucket_policy_status::GetBucketPolicyStatusError::unhandled)
        })
        .transpose()
}

pub fn de_policy_status(inp: &[u8]) -> std::result::Result<crate::types::PolicyStatus, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("PolicyStatus")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected PolicyStatus got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_policy_status::de_policy_status(&mut decoder)
}
