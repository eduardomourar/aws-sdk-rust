// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_model_customization_job_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_model_customization_job::CreateModelCustomizationJobInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.base_model_identifier {
        object.key("baseModelIdentifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_request_token {
        object.key("clientRequestToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.custom_model_kms_key_id {
        object.key("customModelKmsKeyId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.custom_model_name {
        object.key("customModelName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.custom_model_tags {
        let mut array_6 = object.key("customModelTags").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.customization_config {
        #[allow(unused_mut)]
        let mut object_10 = object.key("customizationConfig").start_object();
        crate::protocol_serde::shape_customization_config::ser_customization_config(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.customization_type {
        object.key("customizationType").string(var_11.as_str());
    }
    if let Some(var_12) = &input.hyper_parameters {
        #[allow(unused_mut)]
        let mut object_13 = object.key("hyperParameters").start_object();
        for (key_14, value_15) in var_12 {
            {
                object_13.key(key_14.as_str()).string(value_15.as_str());
            }
        }
        object_13.finish();
    }
    if let Some(var_16) = &input.job_name {
        object.key("jobName").string(var_16.as_str());
    }
    if let Some(var_17) = &input.job_tags {
        let mut array_18 = object.key("jobTags").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.output_data_config {
        #[allow(unused_mut)]
        let mut object_22 = object.key("outputDataConfig").start_object();
        crate::protocol_serde::shape_output_data_config::ser_output_data_config(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.role_arn {
        object.key("roleArn").string(var_23.as_str());
    }
    if let Some(var_24) = &input.training_data_config {
        #[allow(unused_mut)]
        let mut object_25 = object.key("trainingDataConfig").start_object();
        crate::protocol_serde::shape_training_data_config::ser_training_data_config(&mut object_25, var_24)?;
        object_25.finish();
    }
    if let Some(var_26) = &input.validation_data_config {
        #[allow(unused_mut)]
        let mut object_27 = object.key("validationDataConfig").start_object();
        crate::protocol_serde::shape_validation_data_config::ser_validation_data_config(&mut object_27, var_26)?;
        object_27.finish();
    }
    if let Some(var_28) = &input.vpc_config {
        #[allow(unused_mut)]
        let mut object_29 = object.key("vpcConfig").start_object();
        crate::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_29, var_28)?;
        object_29.finish();
    }
    Ok(())
}
