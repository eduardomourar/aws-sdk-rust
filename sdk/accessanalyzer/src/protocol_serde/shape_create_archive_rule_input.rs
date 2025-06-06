// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_archive_rule_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_archive_rule::CreateArchiveRuleInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filter {
        #[allow(unused_mut)]
        let mut object_3 = object.key("filter").start_object();
        for (key_4, value_5) in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_6 = object_3.key(key_4.as_str()).start_object();
                crate::protocol_serde::shape_criterion::ser_criterion(&mut object_6, value_5)?;
                object_6.finish();
            }
        }
        object_3.finish();
    }
    if let Some(var_7) = &input.rule_name {
        object.key("ruleName").string(var_7.as_str());
    }
    Ok(())
}
