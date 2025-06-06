// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_search_routing_profiles_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::search_routing_profiles::SearchRoutingProfilesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.instance_id {
        object.key("InstanceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.search_criteria {
        #[allow(unused_mut)]
        let mut object_5 = object.key("SearchCriteria").start_object();
        crate::protocol_serde::shape_routing_profile_search_criteria::ser_routing_profile_search_criteria(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.search_filter {
        #[allow(unused_mut)]
        let mut object_7 = object.key("SearchFilter").start_object();
        crate::protocol_serde::shape_routing_profile_search_filter::ser_routing_profile_search_filter(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}
