// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_resource_policy_input_input_input(
    input: &crate::operation::put_resource_policy::PutResourcePolicyInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "PutResourcePolicy", "2012-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ResourceArn");
    if let Some(var_2) = &input.resource_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Policy");
    if let Some(var_4) = &input.policy {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
