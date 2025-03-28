// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_theme_data(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CreateThemeData,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    {
        let mut array_1 = object.key("values").start_array();
        for item_2 in &input.values {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_theme_values::ser_theme_values(&mut object_3, item_2)?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    if let Some(var_4) = &input.overrides {
        let mut array_5 = object.key("overrides").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_theme_values::ser_theme_values(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.tags {
        #[allow(unused_mut)]
        let mut object_9 = object.key("tags").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}
