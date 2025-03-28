// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_hub_content_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_hub_content::UpdateHubContentInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.hub_name {
        object.key("HubName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.hub_content_name {
        object.key("HubContentName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.hub_content_type {
        object.key("HubContentType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.hub_content_version {
        object.key("HubContentVersion").string(var_4.as_str());
    }
    if let Some(var_5) = &input.hub_content_display_name {
        object.key("HubContentDisplayName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.hub_content_description {
        object.key("HubContentDescription").string(var_6.as_str());
    }
    if let Some(var_7) = &input.hub_content_markdown {
        object.key("HubContentMarkdown").string(var_7.as_str());
    }
    if let Some(var_8) = &input.hub_content_search_keywords {
        let mut array_9 = object.key("HubContentSearchKeywords").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.support_status {
        object.key("SupportStatus").string(var_11.as_str());
    }
    Ok(())
}
