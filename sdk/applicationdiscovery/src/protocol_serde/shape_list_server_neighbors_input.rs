// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_server_neighbors_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_server_neighbors::ListServerNeighborsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.configuration_id {
        object.key("configurationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.port_information_needed {
        object.key("portInformationNeeded").boolean(*var_2);
    }
    if let Some(var_3) = &input.neighbor_configuration_ids {
        let mut array_4 = object.key("neighborConfigurationIds").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.next_token {
        object.key("nextToken").string(var_7.as_str());
    }
    Ok(())
}
