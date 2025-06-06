// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeHoursOfOperationOverride`](crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideFluentBuilder::set_instance_id):<br>required: **true**<br><p>The identifier of the Amazon Connect instance.</p><br>
    ///   - [`hours_of_operation_id(impl Into<String>)`](crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideFluentBuilder::hours_of_operation_id) / [`set_hours_of_operation_id(Option<String>)`](crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideFluentBuilder::set_hours_of_operation_id):<br>required: **true**<br><p>The identifier for the hours of operation.</p><br>
    ///   - [`hours_of_operation_override_id(impl Into<String>)`](crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideFluentBuilder::hours_of_operation_override_id) / [`set_hours_of_operation_override_id(Option<String>)`](crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideFluentBuilder::set_hours_of_operation_override_id):<br>required: **true**<br><p>The identifier for the hours of operation override.</p><br>
    /// - On success, responds with [`DescribeHoursOfOperationOverrideOutput`](crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideOutput) with field(s):
    ///   - [`hours_of_operation_override(Option<HoursOfOperationOverride>)`](crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideOutput::hours_of_operation_override): <p>Information about the hours of operations override.</p>
    /// - On failure, responds with [`SdkError<DescribeHoursOfOperationOverrideError>`](crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError)
    pub fn describe_hours_of_operation_override(
        &self,
    ) -> crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideFluentBuilder {
        crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideFluentBuilder::new(self.handle.clone())
    }
}
