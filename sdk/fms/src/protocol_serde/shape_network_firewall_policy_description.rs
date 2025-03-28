// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_network_firewall_policy_description<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::NetworkFirewallPolicyDescription>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::NetworkFirewallPolicyDescriptionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "StatelessRuleGroups" => {
                            builder = builder.set_stateless_rule_groups(
                                crate::protocol_serde::shape_stateless_rule_group_list::de_stateless_rule_group_list(tokens)?,
                            );
                        }
                        "StatelessDefaultActions" => {
                            builder = builder.set_stateless_default_actions(
                                crate::protocol_serde::shape_network_firewall_action_list::de_network_firewall_action_list(tokens)?,
                            );
                        }
                        "StatelessFragmentDefaultActions" => {
                            builder = builder.set_stateless_fragment_default_actions(
                                crate::protocol_serde::shape_network_firewall_action_list::de_network_firewall_action_list(tokens)?,
                            );
                        }
                        "StatelessCustomActions" => {
                            builder = builder.set_stateless_custom_actions(
                                crate::protocol_serde::shape_network_firewall_action_list::de_network_firewall_action_list(tokens)?,
                            );
                        }
                        "StatefulRuleGroups" => {
                            builder = builder.set_stateful_rule_groups(
                                crate::protocol_serde::shape_stateful_rule_group_list::de_stateful_rule_group_list(tokens)?,
                            );
                        }
                        "StatefulDefaultActions" => {
                            builder = builder.set_stateful_default_actions(
                                crate::protocol_serde::shape_network_firewall_action_list::de_network_firewall_action_list(tokens)?,
                            );
                        }
                        "StatefulEngineOptions" => {
                            builder = builder.set_stateful_engine_options(
                                crate::protocol_serde::shape_stateful_engine_options::de_stateful_engine_options(tokens)?,
                            );
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
