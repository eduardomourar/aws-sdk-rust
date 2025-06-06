// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_trust_store_associations_input_input_input(
    input: &crate::operation::describe_trust_store_associations::DescribeTrustStoreAssociationsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeTrustStoreAssociations", "2015-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TrustStoreArn");
    if let Some(var_2) = &input.trust_store_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Marker");
    if let Some(var_4) = &input.marker {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PageSize");
    if let Some(var_6) = &input.page_size {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
