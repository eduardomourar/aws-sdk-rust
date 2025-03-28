// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sessions_statistics_resources(
    object_7: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SessionsStatisticsResources,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::SessionsStatisticsResources::QueueIds(inner) => {
            let mut array_1 = object_7.key("queueIds").start_array();
            for item_2 in inner {
                {
                    array_1.value().string(item_2.as_str());
                }
            }
            array_1.finish();
        }
        crate::types::SessionsStatisticsResources::FleetIds(inner) => {
            let mut array_3 = object_7.key("fleetIds").start_array();
            for item_4 in inner {
                {
                    array_3.value().string(item_4.as_str());
                }
            }
            array_3.finish();
        }
        crate::types::SessionsStatisticsResources::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "SessionsStatisticsResources",
            ))
        }
    }
    Ok(())
}
