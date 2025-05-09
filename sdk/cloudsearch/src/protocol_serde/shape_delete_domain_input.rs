// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_domain_input_input_input(
    input: &crate::operation::delete_domain::DeleteDomainInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DeleteDomain", "2013-01-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DomainName");
    if let Some(var_2) = &input.domain_name {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
