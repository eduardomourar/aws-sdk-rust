// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_usage_limit_input_input_input(
    input: &crate::operation::modify_usage_limit::ModifyUsageLimitInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyUsageLimit", "2012-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("UsageLimitId");
    if let Some(var_2) = &input.usage_limit_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Amount");
    if let Some(var_4) = &input.amount {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("BreachAction");
    if let Some(var_6) = &input.breach_action {
        scope_5.string(var_6.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
