// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_route_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_route::UpdateRouteInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.spec {
        #[allow(unused_mut)]
        let mut object_3 = object.key("spec").start_object();
        crate::protocol_serde::shape_route_spec::ser_route_spec(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}
