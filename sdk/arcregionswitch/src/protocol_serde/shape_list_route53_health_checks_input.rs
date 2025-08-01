// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_route53_health_checks_input_input(
    encoder: &mut ::aws_smithy_cbor::Encoder,
    #[allow(unused)] input: &crate::operation::list_route53_health_checks::ListRoute53HealthChecksInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    encoder.begin_map();
    if let Some(var_1) = &input.arn {
        encoder.str("arn").str(var_1.as_str());
    }
    if let Some(var_2) = &input.hosted_zone_id {
        encoder.str("hostedZoneId").str(var_2.as_str());
    }
    if let Some(var_3) = &input.record_name {
        encoder.str("recordName").str(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        encoder.str("maxResults").integer(*var_4);
    }
    if let Some(var_5) = &input.next_token {
        encoder.str("nextToken").str(var_5.as_str());
    }
    encoder.end();
    Ok(())
}
