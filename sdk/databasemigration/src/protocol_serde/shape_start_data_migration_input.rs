// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_data_migration_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_data_migration::StartDataMigrationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.data_migration_identifier {
        object.key("DataMigrationIdentifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.start_type {
        object.key("StartType").string(var_2.as_str());
    }
    Ok(())
}
