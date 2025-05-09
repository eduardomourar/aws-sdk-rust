// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteSdiSource`](crate::operation::delete_sdi_source::builders::DeleteSdiSourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`sdi_source_id(impl Into<String>)`](crate::operation::delete_sdi_source::builders::DeleteSdiSourceFluentBuilder::sdi_source_id) / [`set_sdi_source_id(Option<String>)`](crate::operation::delete_sdi_source::builders::DeleteSdiSourceFluentBuilder::set_sdi_source_id):<br>required: **true**<br>The ID of the SdiSource.<br>
    /// - On success, responds with [`DeleteSdiSourceOutput`](crate::operation::delete_sdi_source::DeleteSdiSourceOutput) with field(s):
    ///   - [`sdi_source(Option<SdiSource>)`](crate::operation::delete_sdi_source::DeleteSdiSourceOutput::sdi_source): Settings for the SDI source.
    /// - On failure, responds with [`SdkError<DeleteSdiSourceError>`](crate::operation::delete_sdi_source::DeleteSdiSourceError)
    pub fn delete_sdi_source(&self) -> crate::operation::delete_sdi_source::builders::DeleteSdiSourceFluentBuilder {
        crate::operation::delete_sdi_source::builders::DeleteSdiSourceFluentBuilder::new(self.handle.clone())
    }
}
