// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CopyServerlessCacheSnapshot`](crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_serverless_cache_snapshot_name(impl Into<String>)`](crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder::source_serverless_cache_snapshot_name) / [`set_source_serverless_cache_snapshot_name(Option<String>)`](crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder::set_source_serverless_cache_snapshot_name):<br>required: **true**<br><p>The identifier of the existing serverless cache’s snapshot to be copied. Available for Valkey, Redis OSS and Serverless Memcached only.</p><br>
    ///   - [`target_serverless_cache_snapshot_name(impl Into<String>)`](crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder::target_serverless_cache_snapshot_name) / [`set_target_serverless_cache_snapshot_name(Option<String>)`](crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder::set_target_serverless_cache_snapshot_name):<br>required: **true**<br><p>The identifier for the snapshot to be created. Available for Valkey, Redis OSS and Serverless Memcached only.</p><br>
    ///   - [`kms_key_id(impl Into<String>)`](crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder::set_kms_key_id):<br>required: **false**<br><p>The identifier of the KMS key used to encrypt the target snapshot. Available for Valkey, Redis OSS and Serverless Memcached only.</p><br>
    ///   - [`tags(Tag)`](crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder::set_tags):<br>required: **false**<br><p>A list of tags to be added to the target snapshot resource. A tag is a key-value pair. Available for Valkey, Redis OSS and Serverless Memcached only. Default: NULL</p><br>
    /// - On success, responds with [`CopyServerlessCacheSnapshotOutput`](crate::operation::copy_serverless_cache_snapshot::CopyServerlessCacheSnapshotOutput) with field(s):
    ///   - [`serverless_cache_snapshot(Option<ServerlessCacheSnapshot>)`](crate::operation::copy_serverless_cache_snapshot::CopyServerlessCacheSnapshotOutput::serverless_cache_snapshot): <p>The response for the attempt to copy the serverless cache snapshot. Available for Valkey, Redis OSS and Serverless Memcached only.</p>
    /// - On failure, responds with [`SdkError<CopyServerlessCacheSnapshotError>`](crate::operation::copy_serverless_cache_snapshot::CopyServerlessCacheSnapshotError)
    pub fn copy_serverless_cache_snapshot(
        &self,
    ) -> crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder {
        crate::operation::copy_serverless_cache_snapshot::builders::CopyServerlessCacheSnapshotFluentBuilder::new(self.handle.clone())
    }
}
