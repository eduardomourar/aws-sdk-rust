// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_job_queue_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_job_queue::CreateJobQueueInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.compute_environment_order {
        let mut array_2 = object.key("computeEnvironmentOrder").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_compute_environment_order::ser_compute_environment_order(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.job_queue_name {
        object.key("jobQueueName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.job_queue_type {
        object.key("jobQueueType").string(var_6.as_str());
    }
    if let Some(var_7) = &input.job_state_time_limit_actions {
        let mut array_8 = object.key("jobStateTimeLimitActions").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_job_state_time_limit_action::ser_job_state_time_limit_action(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.priority {
        object.key("priority").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.scheduling_policy_arn {
        object.key("schedulingPolicyArn").string(var_12.as_str());
    }
    if let Some(var_13) = &input.service_environment_order {
        let mut array_14 = object.key("serviceEnvironmentOrder").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_service_environment_order::ser_service_environment_order(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.state {
        object.key("state").string(var_17.as_str());
    }
    if let Some(var_18) = &input.tags {
        #[allow(unused_mut)]
        let mut object_19 = object.key("tags").start_object();
        for (key_20, value_21) in var_18 {
            {
                object_19.key(key_20.as_str()).string(value_21.as_str());
            }
        }
        object_19.finish();
    }
    Ok(())
}
