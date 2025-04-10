// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_traffic_policy_instance_input_input_input(
    input: &crate::operation::create_traffic_policy_instance::CreateTrafficPolicyInstanceInput,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.hosted_zone_id {
        let mut inner_writer = scope.start_el("HostedZoneId").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        let mut inner_writer = scope.start_el("Name").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.ttl {
        let mut inner_writer = scope.start_el("TTL").finish();
        inner_writer.data(::aws_smithy_types::primitive::Encoder::from(*var_3).encode());
    }
    if let Some(var_4) = &input.traffic_policy_id {
        let mut inner_writer = scope.start_el("TrafficPolicyId").finish();
        inner_writer.data(var_4.as_str());
    }
    if let Some(var_5) = &input.traffic_policy_version {
        let mut inner_writer = scope.start_el("TrafficPolicyVersion").finish();
        inner_writer.data(::aws_smithy_types::primitive::Encoder::from(*var_5).encode());
    }
    scope.finish();
    Ok(())
}
