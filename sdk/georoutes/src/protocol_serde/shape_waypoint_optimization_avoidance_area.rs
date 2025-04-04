// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_waypoint_optimization_avoidance_area(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::WaypointOptimizationAvoidanceArea,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.geometry {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Geometry").start_object();
        crate::protocol_serde::shape_waypoint_optimization_avoidance_area_geometry::ser_waypoint_optimization_avoidance_area_geometry(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    Ok(())
}
