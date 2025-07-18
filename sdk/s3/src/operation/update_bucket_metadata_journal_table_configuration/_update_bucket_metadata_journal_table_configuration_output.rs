// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateBucketMetadataJournalTableConfigurationOutput {
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl crate::s3_request_id::RequestIdExt for UpdateBucketMetadataJournalTableConfigurationOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for UpdateBucketMetadataJournalTableConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateBucketMetadataJournalTableConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`UpdateBucketMetadataJournalTableConfigurationOutput`](crate::operation::update_bucket_metadata_journal_table_configuration::UpdateBucketMetadataJournalTableConfigurationOutput).
    pub fn builder(
    ) -> crate::operation::update_bucket_metadata_journal_table_configuration::builders::UpdateBucketMetadataJournalTableConfigurationOutputBuilder
    {
        crate::operation::update_bucket_metadata_journal_table_configuration::builders::UpdateBucketMetadataJournalTableConfigurationOutputBuilder::default()
    }
}

/// A builder for [`UpdateBucketMetadataJournalTableConfigurationOutput`](crate::operation::update_bucket_metadata_journal_table_configuration::UpdateBucketMetadataJournalTableConfigurationOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateBucketMetadataJournalTableConfigurationOutputBuilder {
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl UpdateBucketMetadataJournalTableConfigurationOutputBuilder {
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(&mut self, extended_request_id: Option<String>) -> &mut Self {
        self._extended_request_id = extended_request_id;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateBucketMetadataJournalTableConfigurationOutput`](crate::operation::update_bucket_metadata_journal_table_configuration::UpdateBucketMetadataJournalTableConfigurationOutput).
    pub fn build(self) -> crate::operation::update_bucket_metadata_journal_table_configuration::UpdateBucketMetadataJournalTableConfigurationOutput {
        crate::operation::update_bucket_metadata_journal_table_configuration::UpdateBucketMetadataJournalTableConfigurationOutput {
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
