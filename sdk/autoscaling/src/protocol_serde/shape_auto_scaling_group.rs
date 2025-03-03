// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_auto_scaling_group(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::AutoScalingGroup, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AutoScalingGroup::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AutoScalingGroupName") /* AutoScalingGroupName com.amazonaws.autoscaling#AutoScalingGroup$AutoScalingGroupName */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_auto_scaling_group_name(var_1);
            }
            ,
            s if s.matches("AutoScalingGroupARN") /* AutoScalingGroupARN com.amazonaws.autoscaling#AutoScalingGroup$AutoScalingGroupARN */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_auto_scaling_group_arn(var_2);
            }
            ,
            s if s.matches("LaunchConfigurationName") /* LaunchConfigurationName com.amazonaws.autoscaling#AutoScalingGroup$LaunchConfigurationName */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_launch_configuration_name(var_3);
            }
            ,
            s if s.matches("LaunchTemplate") /* LaunchTemplate com.amazonaws.autoscaling#AutoScalingGroup$LaunchTemplate */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_launch_template_specification::de_launch_template_specification(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_launch_template(var_4);
            }
            ,
            s if s.matches("MixedInstancesPolicy") /* MixedInstancesPolicy com.amazonaws.autoscaling#AutoScalingGroup$MixedInstancesPolicy */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_mixed_instances_policy::de_mixed_instances_policy(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_mixed_instances_policy(var_5);
            }
            ,
            s if s.matches("MinSize") /* MinSize com.amazonaws.autoscaling#AutoScalingGroup$MinSize */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#AutoScalingGroupMinSize`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_min_size(var_6);
            }
            ,
            s if s.matches("MaxSize") /* MaxSize com.amazonaws.autoscaling#AutoScalingGroup$MaxSize */ =>  {
                let var_7 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#AutoScalingGroupMaxSize`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_size(var_7);
            }
            ,
            s if s.matches("DesiredCapacity") /* DesiredCapacity com.amazonaws.autoscaling#AutoScalingGroup$DesiredCapacity */ =>  {
                let var_8 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#AutoScalingGroupDesiredCapacity`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_desired_capacity(var_8);
            }
            ,
            s if s.matches("PredictedCapacity") /* PredictedCapacity com.amazonaws.autoscaling#AutoScalingGroup$PredictedCapacity */ =>  {
                let var_9 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#AutoScalingGroupPredictedCapacity`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_predicted_capacity(var_9);
            }
            ,
            s if s.matches("DefaultCooldown") /* DefaultCooldown com.amazonaws.autoscaling#AutoScalingGroup$DefaultCooldown */ =>  {
                let var_10 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#Cooldown`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_default_cooldown(var_10);
            }
            ,
            s if s.matches("AvailabilityZones") /* AvailabilityZones com.amazonaws.autoscaling#AutoScalingGroup$AvailabilityZones */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_availability_zones::de_availability_zones(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zones(var_11);
            }
            ,
            s if s.matches("LoadBalancerNames") /* LoadBalancerNames com.amazonaws.autoscaling#AutoScalingGroup$LoadBalancerNames */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_load_balancer_names::de_load_balancer_names(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_load_balancer_names(var_12);
            }
            ,
            s if s.matches("TargetGroupARNs") /* TargetGroupARNs com.amazonaws.autoscaling#AutoScalingGroup$TargetGroupARNs */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_target_group_arns::de_target_group_arns(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_target_group_arns(var_13);
            }
            ,
            s if s.matches("HealthCheckType") /* HealthCheckType com.amazonaws.autoscaling#AutoScalingGroup$HealthCheckType */ =>  {
                let var_14 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_health_check_type(var_14);
            }
            ,
            s if s.matches("HealthCheckGracePeriod") /* HealthCheckGracePeriod com.amazonaws.autoscaling#AutoScalingGroup$HealthCheckGracePeriod */ =>  {
                let var_15 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#HealthCheckGracePeriod`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_health_check_grace_period(var_15);
            }
            ,
            s if s.matches("Instances") /* Instances com.amazonaws.autoscaling#AutoScalingGroup$Instances */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_instances::de_instances(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instances(var_16);
            }
            ,
            s if s.matches("CreatedTime") /* CreatedTime com.amazonaws.autoscaling#AutoScalingGroup$CreatedTime */ =>  {
                let var_17 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.autoscaling#TimestampType`)"))
                        ?
                    )
                ;
                builder = builder.set_created_time(var_17);
            }
            ,
            s if s.matches("SuspendedProcesses") /* SuspendedProcesses com.amazonaws.autoscaling#AutoScalingGroup$SuspendedProcesses */ =>  {
                let var_18 =
                    Some(
                        crate::protocol_serde::shape_suspended_processes::de_suspended_processes(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_suspended_processes(var_18);
            }
            ,
            s if s.matches("PlacementGroup") /* PlacementGroup com.amazonaws.autoscaling#AutoScalingGroup$PlacementGroup */ =>  {
                let var_19 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_placement_group(var_19);
            }
            ,
            s if s.matches("VPCZoneIdentifier") /* VPCZoneIdentifier com.amazonaws.autoscaling#AutoScalingGroup$VPCZoneIdentifier */ =>  {
                let var_20 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_zone_identifier(var_20);
            }
            ,
            s if s.matches("EnabledMetrics") /* EnabledMetrics com.amazonaws.autoscaling#AutoScalingGroup$EnabledMetrics */ =>  {
                let var_21 =
                    Some(
                        crate::protocol_serde::shape_enabled_metrics::de_enabled_metrics(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_enabled_metrics(var_21);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.autoscaling#AutoScalingGroup$Status */ =>  {
                let var_22 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_22);
            }
            ,
            s if s.matches("Tags") /* Tags com.amazonaws.autoscaling#AutoScalingGroup$Tags */ =>  {
                let var_23 =
                    Some(
                        crate::protocol_serde::shape_tag_description_list::de_tag_description_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_23);
            }
            ,
            s if s.matches("TerminationPolicies") /* TerminationPolicies com.amazonaws.autoscaling#AutoScalingGroup$TerminationPolicies */ =>  {
                let var_24 =
                    Some(
                        crate::protocol_serde::shape_termination_policies::de_termination_policies(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_termination_policies(var_24);
            }
            ,
            s if s.matches("NewInstancesProtectedFromScaleIn") /* NewInstancesProtectedFromScaleIn com.amazonaws.autoscaling#AutoScalingGroup$NewInstancesProtectedFromScaleIn */ =>  {
                let var_25 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.autoscaling#InstanceProtected`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_new_instances_protected_from_scale_in(var_25);
            }
            ,
            s if s.matches("ServiceLinkedRoleARN") /* ServiceLinkedRoleARN com.amazonaws.autoscaling#AutoScalingGroup$ServiceLinkedRoleARN */ =>  {
                let var_26 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_service_linked_role_arn(var_26);
            }
            ,
            s if s.matches("MaxInstanceLifetime") /* MaxInstanceLifetime com.amazonaws.autoscaling#AutoScalingGroup$MaxInstanceLifetime */ =>  {
                let var_27 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#MaxInstanceLifetime`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_instance_lifetime(var_27);
            }
            ,
            s if s.matches("CapacityRebalance") /* CapacityRebalance com.amazonaws.autoscaling#AutoScalingGroup$CapacityRebalance */ =>  {
                let var_28 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.autoscaling#CapacityRebalanceEnabled`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_capacity_rebalance(var_28);
            }
            ,
            s if s.matches("WarmPoolConfiguration") /* WarmPoolConfiguration com.amazonaws.autoscaling#AutoScalingGroup$WarmPoolConfiguration */ =>  {
                let var_29 =
                    Some(
                        crate::protocol_serde::shape_warm_pool_configuration::de_warm_pool_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_warm_pool_configuration(var_29);
            }
            ,
            s if s.matches("WarmPoolSize") /* WarmPoolSize com.amazonaws.autoscaling#AutoScalingGroup$WarmPoolSize */ =>  {
                let var_30 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#WarmPoolSize`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_warm_pool_size(var_30);
            }
            ,
            s if s.matches("Context") /* Context com.amazonaws.autoscaling#AutoScalingGroup$Context */ =>  {
                let var_31 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_context(var_31);
            }
            ,
            s if s.matches("DesiredCapacityType") /* DesiredCapacityType com.amazonaws.autoscaling#AutoScalingGroup$DesiredCapacityType */ =>  {
                let var_32 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_desired_capacity_type(var_32);
            }
            ,
            s if s.matches("DefaultInstanceWarmup") /* DefaultInstanceWarmup com.amazonaws.autoscaling#AutoScalingGroup$DefaultInstanceWarmup */ =>  {
                let var_33 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#DefaultInstanceWarmup`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_default_instance_warmup(var_33);
            }
            ,
            s if s.matches("TrafficSources") /* TrafficSources com.amazonaws.autoscaling#AutoScalingGroup$TrafficSources */ =>  {
                let var_34 =
                    Some(
                        crate::protocol_serde::shape_traffic_sources::de_traffic_sources(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_traffic_sources(var_34);
            }
            ,
            s if s.matches("InstanceMaintenancePolicy") /* InstanceMaintenancePolicy com.amazonaws.autoscaling#AutoScalingGroup$InstanceMaintenancePolicy */ =>  {
                let var_35 =
                    Some(
                        crate::protocol_serde::shape_instance_maintenance_policy::de_instance_maintenance_policy(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_maintenance_policy(var_35);
            }
            ,
            s if s.matches("AvailabilityZoneDistribution") /* AvailabilityZoneDistribution com.amazonaws.autoscaling#AutoScalingGroup$AvailabilityZoneDistribution */ =>  {
                let var_36 =
                    Some(
                        crate::protocol_serde::shape_availability_zone_distribution::de_availability_zone_distribution(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zone_distribution(var_36);
            }
            ,
            s if s.matches("AvailabilityZoneImpairmentPolicy") /* AvailabilityZoneImpairmentPolicy com.amazonaws.autoscaling#AutoScalingGroup$AvailabilityZoneImpairmentPolicy */ =>  {
                let var_37 =
                    Some(
                        crate::protocol_serde::shape_availability_zone_impairment_policy::de_availability_zone_impairment_policy(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zone_impairment_policy(var_37);
            }
            ,
            s if s.matches("CapacityReservationSpecification") /* CapacityReservationSpecification com.amazonaws.autoscaling#AutoScalingGroup$CapacityReservationSpecification */ =>  {
                let var_38 =
                    Some(
                        crate::protocol_serde::shape_capacity_reservation_specification::de_capacity_reservation_specification(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_capacity_reservation_specification(var_38);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::auto_scaling_group_correct_errors(builder).build())
}
