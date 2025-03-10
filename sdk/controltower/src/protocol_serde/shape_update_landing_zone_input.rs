// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_landing_zone_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_landing_zone::UpdateLandingZoneInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.landing_zone_identifier {
        object.key("landingZoneIdentifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.manifest {
        object.key("manifest").document(var_2);
    }
    if let Some(var_3) = &input.version {
        object.key("version").string(var_3.as_str());
    }
    Ok(())
}
