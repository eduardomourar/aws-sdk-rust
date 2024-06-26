// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateCapability`](crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`capability_id(impl Into<String>)`](crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder::capability_id) / [`set_capability_id(Option<String>)`](crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder::set_capability_id):<br>required: **true**<br><p>Specifies a system-assigned unique identifier for the capability.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder::set_name):<br>required: **false**<br><p>Specifies a new name for the capability, to replace the existing name.</p><br>
    ///   - [`configuration(CapabilityConfiguration)`](crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder::configuration) / [`set_configuration(Option<CapabilityConfiguration>)`](crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder::set_configuration):<br>required: **false**<br><p>Specifies a structure that contains the details for a capability.</p><br>
    ///   - [`instructions_documents(S3Location)`](crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder::instructions_documents) / [`set_instructions_documents(Option<Vec::<S3Location>>)`](crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder::set_instructions_documents):<br>required: **false**<br><p>Specifies one or more locations in Amazon S3, each specifying an EDI document that can be used with this capability. Each item contains the name of the bucket and the key, to identify the document's location.</p><br>
    /// - On success, responds with [`UpdateCapabilityOutput`](crate::operation::update_capability::UpdateCapabilityOutput) with field(s):
    ///   - [`capability_id(String)`](crate::operation::update_capability::UpdateCapabilityOutput::capability_id): <p>Returns a system-assigned unique identifier for the capability.</p>
    ///   - [`capability_arn(String)`](crate::operation::update_capability::UpdateCapabilityOutput::capability_arn): <p>Returns an Amazon Resource Name (ARN) for a specific Amazon Web Services resource, such as a capability, partnership, profile, or transformer.</p>
    ///   - [`name(String)`](crate::operation::update_capability::UpdateCapabilityOutput::name): <p>Returns the name of the capability, used to identify it.</p>
    ///   - [`r#type(CapabilityType)`](crate::operation::update_capability::UpdateCapabilityOutput::type): <p>Returns the type of the capability. Currently, only <code>edi</code> is supported.</p>
    ///   - [`configuration(Option<CapabilityConfiguration>)`](crate::operation::update_capability::UpdateCapabilityOutput::configuration): <p>Returns a structure that contains the details for a capability.</p>
    ///   - [`instructions_documents(Option<Vec::<S3Location>>)`](crate::operation::update_capability::UpdateCapabilityOutput::instructions_documents): <p>Returns one or more locations in Amazon S3, each specifying an EDI document that can be used with this capability. Each item contains the name of the bucket and the key, to identify the document's location.</p>
    ///   - [`created_at(DateTime)`](crate::operation::update_capability::UpdateCapabilityOutput::created_at): <p>Returns a timestamp for creation date and time of the capability.</p>
    ///   - [`modified_at(Option<DateTime>)`](crate::operation::update_capability::UpdateCapabilityOutput::modified_at): <p>Returns a timestamp for last time the capability was modified.</p>
    /// - On failure, responds with [`SdkError<UpdateCapabilityError>`](crate::operation::update_capability::UpdateCapabilityError)
    pub fn update_capability(&self) -> crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder {
        crate::operation::update_capability::builders::UpdateCapabilityFluentBuilder::new(self.handle.clone())
    }
}
