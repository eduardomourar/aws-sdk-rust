// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDataLakeExceptionSubscription`](crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`subscription_protocol(impl Into<String>)`](crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionFluentBuilder::subscription_protocol) / [`set_subscription_protocol(Option<String>)`](crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionFluentBuilder::set_subscription_protocol):<br>required: **true**<br><p>The subscription protocol to which exception messages are posted.</p><br>
    ///   - [`notification_endpoint(impl Into<String>)`](crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionFluentBuilder::notification_endpoint) / [`set_notification_endpoint(Option<String>)`](crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionFluentBuilder::set_notification_endpoint):<br>required: **true**<br><p>The account that is subscribed to receive exception notifications.</p><br>
    ///   - [`exception_time_to_live(i64)`](crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionFluentBuilder::exception_time_to_live) / [`set_exception_time_to_live(Option<i64>)`](crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionFluentBuilder::set_exception_time_to_live):<br>required: **false**<br><p>The time-to-live (TTL) for the exception message to remain. It is the duration of time until which the exception message remains.</p><br>
    /// - On success, responds with [`UpdateDataLakeExceptionSubscriptionOutput`](crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionOutput)
    /// - On failure, responds with [`SdkError<UpdateDataLakeExceptionSubscriptionError>`](crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionError)
    pub fn update_data_lake_exception_subscription(
        &self,
    ) -> crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionFluentBuilder {
        crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
