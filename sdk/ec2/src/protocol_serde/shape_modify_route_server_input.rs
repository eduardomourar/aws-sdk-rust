// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_route_server_input_input_input(
    input: &crate::operation::modify_route_server::ModifyRouteServerInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyRouteServer", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("RouteServerId");
    if let Some(var_2) = &input.route_server_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PersistRoutes");
    if let Some(var_4) = &input.persist_routes {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PersistRoutesDuration");
    if let Some(var_6) = &input.persist_routes_duration {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("SnsNotificationsEnabled");
    if let Some(var_8) = &input.sns_notifications_enabled {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DryRun");
    if let Some(var_10) = &input.dry_run {
        scope_9.boolean(*var_10);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
