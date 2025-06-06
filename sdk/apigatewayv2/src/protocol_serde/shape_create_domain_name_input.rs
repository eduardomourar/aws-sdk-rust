// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_domain_name_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_domain_name::CreateDomainNameInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.domain_name {
        object.key("domainName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.domain_name_configurations {
        let mut array_3 = object.key("domainNameConfigurations").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_domain_name_configuration::ser_domain_name_configuration(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.mutual_tls_authentication {
        #[allow(unused_mut)]
        let mut object_7 = object.key("mutualTlsAuthentication").start_object();
        crate::protocol_serde::shape_mutual_tls_authentication_input::ser_mutual_tls_authentication_input(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.routing_mode {
        object.key("routingMode").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    Ok(())
}
