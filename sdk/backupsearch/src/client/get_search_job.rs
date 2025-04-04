// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSearchJob`](crate::operation::get_search_job::builders::GetSearchJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`search_job_identifier(impl Into<String>)`](crate::operation::get_search_job::builders::GetSearchJobFluentBuilder::search_job_identifier) / [`set_search_job_identifier(Option<String>)`](crate::operation::get_search_job::builders::GetSearchJobFluentBuilder::set_search_job_identifier):<br>required: **true**<br><p>Required unique string that specifies the search job.</p><br>
    /// - On success, responds with [`GetSearchJobOutput`](crate::operation::get_search_job::GetSearchJobOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::get_search_job::GetSearchJobOutput::name): <p>Returned name of the specified search job.</p>
    ///   - [`search_scope_summary(Option<SearchScopeSummary>)`](crate::operation::get_search_job::GetSearchJobOutput::search_scope_summary): <p>Returned summary of the specified search job scope, including:</p> <ul>  <li>   <p>TotalBackupsToScanCount, the number of recovery points returned by the search.</p></li>  <li>   <p>TotalItemsToScanCount, the number of items returned by the search.</p></li> </ul>
    ///   - [`current_search_progress(Option<CurrentSearchProgress>)`](crate::operation::get_search_job::GetSearchJobOutput::current_search_progress): <p>Returns numbers representing BackupsScannedCount, ItemsScanned, and ItemsMatched.</p>
    ///   - [`status_message(Option<String>)`](crate::operation::get_search_job::GetSearchJobOutput::status_message): <p>A status message will be returned for either a earch job with a status of <code>ERRORED</code> or a status of <code>COMPLETED</code> jobs with issues.</p> <p>For example, a message may say that a search contained recovery points unable to be scanned because of a permissions issue.</p>
    ///   - [`encryption_key_arn(Option<String>)`](crate::operation::get_search_job::GetSearchJobOutput::encryption_key_arn): <p>The encryption key for the specified search job.</p> <p>Example: <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p>
    ///   - [`completion_time(Option<DateTime>)`](crate::operation::get_search_job::GetSearchJobOutput::completion_time): <p>The date and time that a search job completed, in Unix format and Coordinated Universal Time (UTC). The value of <code>CompletionTime</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    ///   - [`status(SearchJobState)`](crate::operation::get_search_job::GetSearchJobOutput::status): <p>The current status of the specified search job.</p> <p>A search job may have one of the following statuses: <code>RUNNING</code>; <code>COMPLETED</code>; <code>STOPPED</code>; <code>FAILED</code>; <code>TIMED_OUT</code>; or <code>EXPIRED</code> .</p>
    ///   - [`search_scope(Option<SearchScope>)`](crate::operation::get_search_job::GetSearchJobOutput::search_scope): <p>The search scope is all backup properties input into a search.</p>
    ///   - [`item_filters(Option<ItemFilters>)`](crate::operation::get_search_job::GetSearchJobOutput::item_filters): <p>Item Filters represent all input item properties specified when the search was created.</p>
    ///   - [`creation_time(DateTime)`](crate::operation::get_search_job::GetSearchJobOutput::creation_time): <p>The date and time that a search job was created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CompletionTime</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    ///   - [`search_job_identifier(String)`](crate::operation::get_search_job::GetSearchJobOutput::search_job_identifier): <p>The unique string that identifies the specified search job.</p>
    ///   - [`search_job_arn(String)`](crate::operation::get_search_job::GetSearchJobOutput::search_job_arn): <p>The unique string that identifies the Amazon Resource Name (ARN) of the specified search job.</p>
    /// - On failure, responds with [`SdkError<GetSearchJobError>`](crate::operation::get_search_job::GetSearchJobError)
    pub fn get_search_job(&self) -> crate::operation::get_search_job::builders::GetSearchJobFluentBuilder {
        crate::operation::get_search_job::builders::GetSearchJobFluentBuilder::new(self.handle.clone())
    }
}
