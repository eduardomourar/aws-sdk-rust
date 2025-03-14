// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_location_hdfs_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_location_hdfs::CreateLocationHdfsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.subdirectory {
        object.key("Subdirectory").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name_nodes {
        let mut array_3 = object.key("NameNodes").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_hdfs_name_node::ser_hdfs_name_node(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.block_size {
        object.key("BlockSize").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.replication_factor {
        object.key("ReplicationFactor").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.kms_key_provider_uri {
        object.key("KmsKeyProviderUri").string(var_8.as_str());
    }
    if let Some(var_9) = &input.qop_configuration {
        #[allow(unused_mut)]
        let mut object_10 = object.key("QopConfiguration").start_object();
        crate::protocol_serde::shape_qop_configuration::ser_qop_configuration(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.authentication_type {
        object.key("AuthenticationType").string(var_11.as_str());
    }
    if let Some(var_12) = &input.simple_user {
        object.key("SimpleUser").string(var_12.as_str());
    }
    if let Some(var_13) = &input.kerberos_principal {
        object.key("KerberosPrincipal").string(var_13.as_str());
    }
    if let Some(var_14) = &input.kerberos_keytab {
        object.key("KerberosKeytab").string_unchecked(&::aws_smithy_types::base64::encode(var_14));
    }
    if let Some(var_15) = &input.kerberos_krb5_conf {
        object
            .key("KerberosKrb5Conf")
            .string_unchecked(&::aws_smithy_types::base64::encode(var_15));
    }
    if let Some(var_16) = &input.agent_arns {
        let mut array_17 = object.key("AgentArns").start_array();
        for item_18 in var_16 {
            {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    if let Some(var_19) = &input.tags {
        let mut array_20 = object.key("Tags").start_array();
        for item_21 in var_19 {
            {
                #[allow(unused_mut)]
                let mut object_22 = array_20.value().start_object();
                crate::protocol_serde::shape_tag_list_entry::ser_tag_list_entry(&mut object_22, item_21)?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    Ok(())
}
