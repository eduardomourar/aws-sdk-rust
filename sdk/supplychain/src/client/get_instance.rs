// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetInstance`](crate::operation::get_instance::builders::GetInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::get_instance::builders::GetInstanceFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::get_instance::builders::GetInstanceFluentBuilder::set_instance_id):<br>required: **true**<br><p>The AWS Supply Chain instance identifier</p><br>
    /// - On success, responds with [`GetInstanceOutput`](crate::operation::get_instance::GetInstanceOutput) with field(s):
    ///   - [`instance(Option<Instance>)`](crate::operation::get_instance::GetInstanceOutput::instance): <p>The instance resource data details.</p>
    /// - On failure, responds with [`SdkError<GetInstanceError>`](crate::operation::get_instance::GetInstanceError)
    pub fn get_instance(&self) -> crate::operation::get_instance::builders::GetInstanceFluentBuilder {
        crate::operation::get_instance::builders::GetInstanceFluentBuilder::new(self.handle.clone())
    }
}
