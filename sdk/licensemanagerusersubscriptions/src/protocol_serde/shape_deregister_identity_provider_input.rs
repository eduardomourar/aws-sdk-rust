// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deregister_identity_provider_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::deregister_identity_provider::DeregisterIdentityProviderInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.identity_provider {
        #[allow(unused_mut)]
        let mut object_2 = object.key("IdentityProvider").start_object();
        crate::protocol_serde::shape_identity_provider::ser_identity_provider(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.identity_provider_arn {
        object.key("IdentityProviderArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.product {
        object.key("Product").string(var_4.as_str());
    }
    Ok(())
}
