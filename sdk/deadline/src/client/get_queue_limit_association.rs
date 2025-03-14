// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetQueueLimitAssociation`](crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`farm_id(impl Into<String>)`](crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationFluentBuilder::farm_id) / [`set_farm_id(Option<String>)`](crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationFluentBuilder::set_farm_id):<br>required: **true**<br><p>The unique identifier of the farm that contains the associated queue and limit.</p><br>
    ///   - [`queue_id(impl Into<String>)`](crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationFluentBuilder::queue_id) / [`set_queue_id(Option<String>)`](crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationFluentBuilder::set_queue_id):<br>required: **true**<br><p>The unique identifier of the queue associated with the limit.</p><br>
    ///   - [`limit_id(impl Into<String>)`](crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationFluentBuilder::limit_id) / [`set_limit_id(Option<String>)`](crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationFluentBuilder::set_limit_id):<br>required: **true**<br><p>The unique identifier of the limit associated with the queue.</p><br>
    /// - On success, responds with [`GetQueueLimitAssociationOutput`](crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput) with field(s):
    ///   - [`created_at(DateTime)`](crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput::created_at): <p>The Unix timestamp of the date and time that the association was created.</p>
    ///   - [`created_by(String)`](crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput::created_by): <p>The user identifier of the person that created the association.</p>
    ///   - [`updated_at(Option<DateTime>)`](crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput::updated_at): <p>The Unix timestamp of the date and time that the association was last updated.</p>
    ///   - [`updated_by(Option<String>)`](crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput::updated_by): <p>The user identifier of the person that last updated the association.</p>
    ///   - [`queue_id(String)`](crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput::queue_id): <p>The unique identifier of the queue associated with the limit.</p>
    ///   - [`limit_id(String)`](crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput::limit_id): <p>The unique identifier of the limit associated with the queue.</p>
    ///   - [`status(QueueLimitAssociationStatus)`](crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput::status): <p>The current status of the limit.</p>
    /// - On failure, responds with [`SdkError<GetQueueLimitAssociationError>`](crate::operation::get_queue_limit_association::GetQueueLimitAssociationError)
    pub fn get_queue_limit_association(&self) -> crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationFluentBuilder {
        crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationFluentBuilder::new(self.handle.clone())
    }
}
