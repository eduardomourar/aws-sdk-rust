// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_ip_pools_input_input_input(
    input: &crate::operation::modify_ip_pools::ModifyIpPoolsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyIpPools", "2015-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LoadBalancerArn");
    if let Some(var_2) = &input.load_balancer_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("IpamPools");
    if let Some(var_4) = &input.ipam_pools {
        crate::protocol_serde::shape_ipam_pools::ser_ipam_pools(scope_3, var_4)?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("RemoveIpamPools");
    if let Some(var_6) = &input.remove_ipam_pools {
        let mut list_8 = scope_5.start_list(false, None);
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            entry_9.string(item_7.as_str());
        }
        list_8.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
