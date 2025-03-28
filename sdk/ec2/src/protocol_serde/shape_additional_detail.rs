// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_additional_detail(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::AdditionalDetail, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AdditionalDetail::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("additionalDetailType") /* AdditionalDetailType com.amazonaws.ec2#AdditionalDetail$AdditionalDetailType */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_additional_detail_type(var_1);
            }
            ,
            s if s.matches("component") /* Component com.amazonaws.ec2#AdditionalDetail$Component */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_component(var_2);
            }
            ,
            s if s.matches("vpcEndpointService") /* VpcEndpointService com.amazonaws.ec2#AdditionalDetail$VpcEndpointService */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpc_endpoint_service(var_3);
            }
            ,
            s if s.matches("ruleOptionSet") /* RuleOptions com.amazonaws.ec2#AdditionalDetail$RuleOptions */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_rule_option_list::de_rule_option_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_rule_options(var_4);
            }
            ,
            s if s.matches("ruleGroupTypePairSet") /* RuleGroupTypePairs com.amazonaws.ec2#AdditionalDetail$RuleGroupTypePairs */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_rule_group_type_pair_list::de_rule_group_type_pair_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_rule_group_type_pairs(var_5);
            }
            ,
            s if s.matches("ruleGroupRuleOptionsPairSet") /* RuleGroupRuleOptionsPairs com.amazonaws.ec2#AdditionalDetail$RuleGroupRuleOptionsPairs */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_rule_group_rule_options_pair_list::de_rule_group_rule_options_pair_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_rule_group_rule_options_pairs(var_6);
            }
            ,
            s if s.matches("serviceName") /* ServiceName com.amazonaws.ec2#AdditionalDetail$ServiceName */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_service_name(var_7);
            }
            ,
            s if s.matches("loadBalancerSet") /* LoadBalancers com.amazonaws.ec2#AdditionalDetail$LoadBalancers */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_analysis_component_list::de_analysis_component_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_load_balancers(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
