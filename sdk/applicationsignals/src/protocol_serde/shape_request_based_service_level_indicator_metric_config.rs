// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_request_based_service_level_indicator_metric_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RequestBasedServiceLevelIndicatorMetricConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.key_attributes {
        #[allow(unused_mut)]
        let mut object_2 = object.key("KeyAttributes").start_object();
        for (key_3, value_4) in var_1 {
            {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.operation_name {
        object.key("OperationName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.metric_type {
        object.key("MetricType").string(var_6.as_str());
    }
    if let Some(var_7) = &input.total_request_count_metric {
        let mut array_8 = object.key("TotalRequestCountMetric").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_metric_data_query::ser_metric_data_query(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.monitored_request_count_metric {
        #[allow(unused_mut)]
        let mut object_12 = object.key("MonitoredRequestCountMetric").start_object();
        crate::protocol_serde::shape_monitored_request_count_metric_data_queries::ser_monitored_request_count_metric_data_queries(
            &mut object_12,
            var_11,
        )?;
        object_12.finish();
    }
    if let Some(var_13) = &input.dependency_config {
        #[allow(unused_mut)]
        let mut object_14 = object.key("DependencyConfig").start_object();
        crate::protocol_serde::shape_dependency_config::ser_dependency_config(&mut object_14, var_13)?;
        object_14.finish();
    }
    Ok(())
}
