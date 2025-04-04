// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_service_software_update_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_service_software_update::StartServiceSoftwareUpdateInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.desired_start_time {
        object.key("DesiredStartTime").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.domain_name {
        object.key("DomainName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.schedule_at {
        object.key("ScheduleAt").string(var_3.as_str());
    }
    Ok(())
}
