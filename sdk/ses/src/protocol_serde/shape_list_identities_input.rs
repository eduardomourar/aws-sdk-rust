// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_identities_input_input_input(
    input: &crate::operation::list_identities::ListIdentitiesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ListIdentities", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("IdentityType");
    if let Some(var_2) = &input.identity_type {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("NextToken");
    if let Some(var_4) = &input.next_token {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MaxItems");
    if let Some(var_6) = &input.max_items {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
