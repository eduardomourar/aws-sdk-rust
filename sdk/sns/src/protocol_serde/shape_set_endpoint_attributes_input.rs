// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_set_endpoint_attributes_input_input_input(
    input: &crate::operation::set_endpoint_attributes::SetEndpointAttributesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "SetEndpointAttributes", "2010-03-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EndpointArn");
    if let Some(var_2) = &input.endpoint_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Attributes");
    if let Some(var_4) = &input.attributes {
        let mut map_5 = scope_3.start_map(false, "key", "value");
        for (key_6, value_7) in var_4 {
            #[allow(unused_mut)]
            let mut entry_8 = map_5.entry(key_6);
            {
                entry_8.string(value_7);
            }
        }
        map_5.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
