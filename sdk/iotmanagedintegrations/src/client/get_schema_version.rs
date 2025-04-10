// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSchemaVersion`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`r#type(SchemaVersionType)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::type) / [`set_type(Option<SchemaVersionType>)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::set_type):<br>required: **true**<br><p>The type of schema version.</p><br>
    ///   - [`schema_versioned_id(impl Into<String>)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::schema_versioned_id) / [`set_schema_versioned_id(Option<String>)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::set_schema_versioned_id):<br>required: **true**<br><p>Schema id with a version specified. If the version is missing, it defaults to latest version.</p><br>
    ///   - [`format(SchemaVersionFormat)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::format) / [`set_format(Option<SchemaVersionFormat>)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::set_format):<br>required: **false**<br><p>The format of the schema version.</p><br>
    /// - On success, responds with [`GetSchemaVersionOutput`](crate::operation::get_schema_version::GetSchemaVersionOutput) with field(s):
    ///   - [`schema_id(Option<String>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::schema_id): <p>The id of the schema version.</p>
    ///   - [`r#type(Option<SchemaVersionType>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::type): <p>The type of schema version.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::description): <p>The description of the schema version.</p>
    ///   - [`namespace(Option<String>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::namespace): <p>The name of the schema version.</p>
    ///   - [`semantic_version(Option<String>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::semantic_version): <p>The schema version. If this is left blank, it defaults to the latest version.</p>
    ///   - [`visibility(Option<SchemaVersionVisibility>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::visibility): <p>The visibility of the schema version.</p>
    ///   - [`schema(Option<Document>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::schema): <p>The schema of the schema version.</p>
    /// - On failure, responds with [`SdkError<GetSchemaVersionError>`](crate::operation::get_schema_version::GetSchemaVersionError)
    pub fn get_schema_version(&self) -> crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder {
        crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::new(self.handle.clone())
    }
}
