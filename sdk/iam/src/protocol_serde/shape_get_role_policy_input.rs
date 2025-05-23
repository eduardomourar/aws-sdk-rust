// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_role_policy_input_input_input(
    input: &crate::operation::get_role_policy::GetRolePolicyInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetRolePolicy", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("RoleName");
    if let Some(var_2) = &input.role_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PolicyName");
    if let Some(var_4) = &input.policy_name {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
