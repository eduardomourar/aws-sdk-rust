// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_wireless_gateway_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_wireless_gateway::UpdateWirelessGatewayInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.join_eui_filters {
        let mut array_3 = object.key("JoinEuiFilters").start_array();
        for item_4 in var_2 {
            {
                let mut array_5 = array_3.value().start_array();
                for item_6 in item_4 {
                    {
                        array_5.value().string(item_6.as_str());
                    }
                }
                array_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_7) = &input.max_eirp {
        object.key("MaxEirp").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.net_id_filters {
        let mut array_10 = object.key("NetIdFilters").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    Ok(())
}
