// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetManagedNotificationChildEvent`](crate::operation::get_managed_notification_child_event::builders::GetManagedNotificationChildEventFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::get_managed_notification_child_event::builders::GetManagedNotificationChildEventFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::get_managed_notification_child_event::builders::GetManagedNotificationChildEventFluentBuilder::set_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the <code>ManagedNotificationChildEvent</code> to return.</p><br>
    ///   - [`locale(LocaleCode)`](crate::operation::get_managed_notification_child_event::builders::GetManagedNotificationChildEventFluentBuilder::locale) / [`set_locale(Option<LocaleCode>)`](crate::operation::get_managed_notification_child_event::builders::GetManagedNotificationChildEventFluentBuilder::set_locale):<br>required: **false**<br><p>The locale code of the language used for the retrieved <code>ManagedNotificationChildEvent</code>. The default locale is English <code>en_US</code>.</p><br>
    /// - On success, responds with [`GetManagedNotificationChildEventOutput`](crate::operation::get_managed_notification_child_event::GetManagedNotificationChildEventOutput) with field(s):
    ///   - [`arn(String)`](crate::operation::get_managed_notification_child_event::GetManagedNotificationChildEventOutput::arn): <p>The ARN of the resource.</p>
    ///   - [`managed_notification_configuration_arn(String)`](crate::operation::get_managed_notification_child_event::GetManagedNotificationChildEventOutput::managed_notification_configuration_arn): <p>The Amazon Resource Name (ARN) of the <code>ManagedNotificationConfiguration</code> associated with the <code>ManagedNotificationChildEvent</code>.</p>
    ///   - [`creation_time(DateTime)`](crate::operation::get_managed_notification_child_event::GetManagedNotificationChildEventOutput::creation_time): <p>The creation time of the <code>ManagedNotificationChildEvent</code>.</p>
    ///   - [`content(Option<ManagedNotificationChildEvent>)`](crate::operation::get_managed_notification_child_event::GetManagedNotificationChildEventOutput::content): <p>The content of the <code>ManagedNotificationChildEvent</code>.</p>
    /// - On failure, responds with [`SdkError<GetManagedNotificationChildEventError>`](crate::operation::get_managed_notification_child_event::GetManagedNotificationChildEventError)
    pub fn get_managed_notification_child_event(
        &self,
    ) -> crate::operation::get_managed_notification_child_event::builders::GetManagedNotificationChildEventFluentBuilder {
        crate::operation::get_managed_notification_child_event::builders::GetManagedNotificationChildEventFluentBuilder::new(self.handle.clone())
    }
}
