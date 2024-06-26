// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBlueprint`](crate::operation::get_blueprint::builders::GetBlueprintFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::get_blueprint::builders::GetBlueprintFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::get_blueprint::builders::GetBlueprintFluentBuilder::set_name):<br>required: **true**<br><p>The name of the blueprint.</p><br>
    ///   - [`include_blueprint(bool)`](crate::operation::get_blueprint::builders::GetBlueprintFluentBuilder::include_blueprint) / [`set_include_blueprint(Option<bool>)`](crate::operation::get_blueprint::builders::GetBlueprintFluentBuilder::set_include_blueprint):<br>required: **false**<br><p>Specifies whether or not to include the blueprint in the response.</p><br>
    ///   - [`include_parameter_spec(bool)`](crate::operation::get_blueprint::builders::GetBlueprintFluentBuilder::include_parameter_spec) / [`set_include_parameter_spec(Option<bool>)`](crate::operation::get_blueprint::builders::GetBlueprintFluentBuilder::set_include_parameter_spec):<br>required: **false**<br><p>Specifies whether or not to include the parameter specification.</p><br>
    /// - On success, responds with [`GetBlueprintOutput`](crate::operation::get_blueprint::GetBlueprintOutput) with field(s):
    ///   - [`blueprint(Option<Blueprint>)`](crate::operation::get_blueprint::GetBlueprintOutput::blueprint): <p>Returns a <code>Blueprint</code> object.</p>
    /// - On failure, responds with [`SdkError<GetBlueprintError>`](crate::operation::get_blueprint::GetBlueprintError)
    pub fn get_blueprint(&self) -> crate::operation::get_blueprint::builders::GetBlueprintFluentBuilder {
        crate::operation::get_blueprint::builders::GetBlueprintFluentBuilder::new(self.handle.clone())
    }
}
