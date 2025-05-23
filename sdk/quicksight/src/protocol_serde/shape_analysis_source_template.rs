// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_analysis_source_template(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AnalysisSourceTemplate,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("DataSetReferences").start_array();
        for item_2 in &input.data_set_references {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_data_set_reference::ser_data_set_reference(&mut object_3, item_2)?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    {
        object.key("Arn").string(input.arn.as_str());
    }
    Ok(())
}
