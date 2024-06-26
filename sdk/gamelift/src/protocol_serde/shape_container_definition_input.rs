// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_container_definition_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ContainerDefinitionInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.container_name {
        object.key("ContainerName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.image_uri {
        object.key("ImageUri").string(var_2.as_str());
    }
    if let Some(var_3) = &input.memory_limits {
        #[allow(unused_mut)]
        let mut object_4 = object.key("MemoryLimits").start_object();
        crate::protocol_serde::shape_container_memory_limits::ser_container_memory_limits(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.port_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("PortConfiguration").start_object();
        crate::protocol_serde::shape_container_port_configuration::ser_container_port_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.cpu {
        object.key("Cpu").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.health_check {
        #[allow(unused_mut)]
        let mut object_9 = object.key("HealthCheck").start_object();
        crate::protocol_serde::shape_container_health_check::ser_container_health_check(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.command {
        let mut array_11 = object.key("Command").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.essential {
        object.key("Essential").boolean(*var_13);
    }
    if let Some(var_14) = &input.entry_point {
        let mut array_15 = object.key("EntryPoint").start_array();
        for item_16 in var_14 {
            {
                array_15.value().string(item_16.as_str());
            }
        }
        array_15.finish();
    }
    if let Some(var_17) = &input.working_directory {
        object.key("WorkingDirectory").string(var_17.as_str());
    }
    if let Some(var_18) = &input.environment {
        let mut array_19 = object.key("Environment").start_array();
        for item_20 in var_18 {
            {
                #[allow(unused_mut)]
                let mut object_21 = array_19.value().start_object();
                crate::protocol_serde::shape_container_environment::ser_container_environment(&mut object_21, item_20)?;
                object_21.finish();
            }
        }
        array_19.finish();
    }
    if let Some(var_22) = &input.depends_on {
        let mut array_23 = object.key("DependsOn").start_array();
        for item_24 in var_22 {
            {
                #[allow(unused_mut)]
                let mut object_25 = array_23.value().start_object();
                crate::protocol_serde::shape_container_dependency::ser_container_dependency(&mut object_25, item_24)?;
                object_25.finish();
            }
        }
        array_23.finish();
    }
    Ok(())
}
