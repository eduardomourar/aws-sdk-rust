// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_network_acl_entry(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::NetworkAclEntry,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.icmp_type_code {
        #[allow(unused_mut)]
        let mut object_2 = object.key("IcmpTypeCode").start_object();
        crate::protocol_serde::shape_network_acl_icmp_type_code::ser_network_acl_icmp_type_code(&mut object_2, var_1)?;
        object_2.finish();
    }
    {
        object.key("Protocol").string(input.protocol.as_str());
    }
    if let Some(var_3) = &input.port_range {
        #[allow(unused_mut)]
        let mut object_4 = object.key("PortRange").start_object();
        crate::protocol_serde::shape_network_acl_port_range::ser_network_acl_port_range(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.cidr_block {
        object.key("CidrBlock").string(var_5.as_str());
    }
    if let Some(var_6) = &input.ipv6_cidr_block {
        object.key("Ipv6CidrBlock").string(var_6.as_str());
    }
    {
        object.key("RuleAction").string(input.rule_action.as_str());
    }
    {
        object.key("Egress").boolean(input.egress);
    }
    Ok(())
}

pub(crate) fn de_network_acl_entry<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::NetworkAclEntry>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::NetworkAclEntryBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "IcmpTypeCode" => {
                            builder = builder.set_icmp_type_code(
                                crate::protocol_serde::shape_network_acl_icmp_type_code::de_network_acl_icmp_type_code(tokens)?,
                            );
                        }
                        "Protocol" => {
                            builder = builder.set_protocol(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PortRange" => {
                            builder = builder.set_port_range(crate::protocol_serde::shape_network_acl_port_range::de_network_acl_port_range(tokens)?);
                        }
                        "CidrBlock" => {
                            builder = builder.set_cidr_block(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Ipv6CidrBlock" => {
                            builder = builder.set_ipv6_cidr_block(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "RuleAction" => {
                            builder = builder.set_rule_action(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::NetworkAclRuleAction::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "Egress" => {
                            builder = builder.set_egress(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(crate::serde_util::network_acl_entry_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
