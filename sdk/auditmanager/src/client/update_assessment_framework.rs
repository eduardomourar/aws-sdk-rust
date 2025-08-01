// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAssessmentFramework`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`framework_id(impl Into<String>)`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::framework_id) / [`set_framework_id(Option<String>)`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::set_framework_id):<br>required: **true**<br><p>The unique identifier for the framework.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::set_name):<br>required: **true**<br><p>The name of the framework to be updated.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::set_description):<br>required: **false**<br><p>The description of the updated framework.</p><br>
    ///   - [`compliance_type(impl Into<String>)`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::compliance_type) / [`set_compliance_type(Option<String>)`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::set_compliance_type):<br>required: **false**<br><p>The compliance type that the new custom framework supports, such as CIS or HIPAA.</p><br>
    ///   - [`control_sets(UpdateAssessmentFrameworkControlSet)`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::control_sets) / [`set_control_sets(Option<Vec::<UpdateAssessmentFrameworkControlSet>>)`](crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::set_control_sets):<br>required: **true**<br><p>The control sets that are associated with the framework.</p><note>  <p>The <code>Controls</code> object returns a partial response when called through Framework APIs. For a complete <code>Controls</code> object, use <code>GetControl</code>.</p> </note><br>
    /// - On success, responds with [`UpdateAssessmentFrameworkOutput`](crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput) with field(s):
    ///   - [`framework(Option<Framework>)`](crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput::framework): <p>The framework object.</p>
    /// - On failure, responds with [`SdkError<UpdateAssessmentFrameworkError>`](crate::operation::update_assessment_framework::UpdateAssessmentFrameworkError)
    pub fn update_assessment_framework(&self) -> crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder {
        crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkFluentBuilder::new(self.handle.clone())
    }
}
