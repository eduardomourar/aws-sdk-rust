// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_resolver_endpoint_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_resolver_endpoint::CreateResolverEndpointInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.security_group_ids {
        let mut array_4 = object.key("SecurityGroupIds").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.direction {
        object.key("Direction").string(var_6.as_str());
    }
    if let Some(var_7) = &input.ip_addresses {
        let mut array_8 = object.key("IpAddresses").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_ip_address_request::ser_ip_address_request(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.outpost_arn {
        object.key("OutpostArn").string(var_11.as_str());
    }
    if let Some(var_12) = &input.preferred_instance_type {
        object.key("PreferredInstanceType").string(var_12.as_str());
    }
    if let Some(var_13) = &input.tags {
        let mut array_14 = object.key("Tags").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.resolver_endpoint_type {
        object.key("ResolverEndpointType").string(var_17.as_str());
    }
    if let Some(var_18) = &input.protocols {
        let mut array_19 = object.key("Protocols").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    Ok(())
}
