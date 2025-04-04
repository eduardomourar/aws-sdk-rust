// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_suggester_input_input_input(
    input: &crate::operation::delete_suggester::DeleteSuggesterInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DeleteSuggester", "2013-01-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DomainName");
    if let Some(var_2) = &input.domain_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SuggesterName");
    if let Some(var_4) = &input.suggester_name {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
