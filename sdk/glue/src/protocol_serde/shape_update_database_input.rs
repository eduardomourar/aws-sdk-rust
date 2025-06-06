// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_database_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_database::UpdateDatabaseInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.catalog_id {
        object.key("CatalogId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.database_input {
        #[allow(unused_mut)]
        let mut object_4 = object.key("DatabaseInput").start_object();
        crate::protocol_serde::shape_database_input::ser_database_input(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
