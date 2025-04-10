// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_evaluation_form_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_evaluation_form::CreateEvaluationFormInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.items {
        let mut array_4 = object.key("Items").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_evaluation_form_item::ser_evaluation_form_item(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.scoring_strategy {
        #[allow(unused_mut)]
        let mut object_8 = object.key("ScoringStrategy").start_object();
        crate::protocol_serde::shape_evaluation_form_scoring_strategy::ser_evaluation_form_scoring_strategy(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.title {
        object.key("Title").string(var_9.as_str());
    }
    Ok(())
}
