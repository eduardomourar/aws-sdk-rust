// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_verified_access_group_input_input_input(
    input: &crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyVerifiedAccessGroup", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("VerifiedAccessGroupId");
    if let Some(var_2) = &input.verified_access_group_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VerifiedAccessInstanceId");
    if let Some(var_4) = &input.verified_access_instance_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Description");
    if let Some(var_6) = &input.description {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("ClientToken");
    if let Some(var_8) = &input.client_token {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DryRun");
    if let Some(var_10) = &input.dry_run {
        scope_9.boolean(*var_10);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
