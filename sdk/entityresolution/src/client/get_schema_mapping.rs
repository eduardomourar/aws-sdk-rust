// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSchemaMapping`](crate::operation::get_schema_mapping::builders::GetSchemaMappingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`schema_name(impl Into<String>)`](crate::operation::get_schema_mapping::builders::GetSchemaMappingFluentBuilder::schema_name) / [`set_schema_name(Option<String>)`](crate::operation::get_schema_mapping::builders::GetSchemaMappingFluentBuilder::set_schema_name):<br>required: **true**<br><p>The name of the schema to be retrieved.</p><br>
    /// - On success, responds with [`GetSchemaMappingOutput`](crate::operation::get_schema_mapping::GetSchemaMappingOutput) with field(s):
    ///   - [`schema_name(String)`](crate::operation::get_schema_mapping::GetSchemaMappingOutput::schema_name): <p>The name of the schema.</p>
    ///   - [`schema_arn(String)`](crate::operation::get_schema_mapping::GetSchemaMappingOutput::schema_arn): <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the SchemaMapping.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_schema_mapping::GetSchemaMappingOutput::description): <p>A description of the schema.</p>
    ///   - [`mapped_input_fields(Vec::<SchemaInputAttribute>)`](crate::operation::get_schema_mapping::GetSchemaMappingOutput::mapped_input_fields): <p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information Entity Resolution uses for matching.</p>
    ///   - [`created_at(DateTime)`](crate::operation::get_schema_mapping::GetSchemaMappingOutput::created_at): <p>The timestamp of when the <code>SchemaMapping</code> was created.</p>
    ///   - [`updated_at(DateTime)`](crate::operation::get_schema_mapping::GetSchemaMappingOutput::updated_at): <p>The timestamp of when the <code>SchemaMapping</code> was last updated.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::get_schema_mapping::GetSchemaMappingOutput::tags): <p>The tags used to organize, track, or control access for this resource.</p>
    ///   - [`has_workflows(bool)`](crate::operation::get_schema_mapping::GetSchemaMappingOutput::has_workflows): <p>Specifies whether the schema mapping has been applied to a workflow.</p>
    /// - On failure, responds with [`SdkError<GetSchemaMappingError>`](crate::operation::get_schema_mapping::GetSchemaMappingError)
    pub fn get_schema_mapping(&self) -> crate::operation::get_schema_mapping::builders::GetSchemaMappingFluentBuilder {
        crate::operation::get_schema_mapping::builders::GetSchemaMappingFluentBuilder::new(self.handle.clone())
    }
}
