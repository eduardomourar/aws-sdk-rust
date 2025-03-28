// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_command_execution_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_command_execution::StartCommandExecutionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.command_arn {
        object.key("commandArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.execution_timeout_seconds {
        object.key("executionTimeoutSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_5 = object.key("parameters").start_object();
        for (key_6, value_7) in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_8 = object_5.key(key_6.as_str()).start_object();
                crate::protocol_serde::shape_command_parameter_value::ser_command_parameter_value(&mut object_8, value_7)?;
                object_8.finish();
            }
        }
        object_5.finish();
    }
    if let Some(var_9) = &input.target_arn {
        object.key("targetArn").string(var_9.as_str());
    }
    Ok(())
}
