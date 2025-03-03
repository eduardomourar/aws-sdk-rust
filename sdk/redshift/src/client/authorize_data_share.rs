// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AuthorizeDataShare`](crate::operation::authorize_data_share::builders::AuthorizeDataShareFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`data_share_arn(impl Into<String>)`](crate::operation::authorize_data_share::builders::AuthorizeDataShareFluentBuilder::data_share_arn) / [`set_data_share_arn(Option<String>)`](crate::operation::authorize_data_share::builders::AuthorizeDataShareFluentBuilder::set_data_share_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the datashare namespace that producers are to authorize sharing for.</p><br>
    ///   - [`consumer_identifier(impl Into<String>)`](crate::operation::authorize_data_share::builders::AuthorizeDataShareFluentBuilder::consumer_identifier) / [`set_consumer_identifier(Option<String>)`](crate::operation::authorize_data_share::builders::AuthorizeDataShareFluentBuilder::set_consumer_identifier):<br>required: **true**<br><p>The identifier of the data consumer that is authorized to access the datashare. This identifier is an Amazon Web Services account ID or a keyword, such as ADX.</p><br>
    ///   - [`allow_writes(bool)`](crate::operation::authorize_data_share::builders::AuthorizeDataShareFluentBuilder::allow_writes) / [`set_allow_writes(Option<bool>)`](crate::operation::authorize_data_share::builders::AuthorizeDataShareFluentBuilder::set_allow_writes):<br>required: **false**<br><p>If set to true, allows write operations for a datashare.</p><br>
    /// - On success, responds with [`AuthorizeDataShareOutput`](crate::operation::authorize_data_share::AuthorizeDataShareOutput) with field(s):
    ///   - [`data_share_arn(Option<String>)`](crate::operation::authorize_data_share::AuthorizeDataShareOutput::data_share_arn): <p>The Amazon Resource Name (ARN) of the datashare that the consumer is to use.</p>
    ///   - [`producer_arn(Option<String>)`](crate::operation::authorize_data_share::AuthorizeDataShareOutput::producer_arn): <p>The Amazon Resource Name (ARN) of the producer namespace.</p>
    ///   - [`allow_publicly_accessible_consumers(Option<bool>)`](crate::operation::authorize_data_share::AuthorizeDataShareOutput::allow_publicly_accessible_consumers): <p>A value that specifies whether the datashare can be shared to a publicly accessible cluster.</p>
    ///   - [`data_share_associations(Option<Vec::<DataShareAssociation>>)`](crate::operation::authorize_data_share::AuthorizeDataShareOutput::data_share_associations): <p>A value that specifies when the datashare has an association between producer and data consumers.</p>
    ///   - [`managed_by(Option<String>)`](crate::operation::authorize_data_share::AuthorizeDataShareOutput::managed_by): <p>The identifier of a datashare to show its managing entity.</p>
    ///   - [`data_share_type(Option<DataShareType>)`](crate::operation::authorize_data_share::AuthorizeDataShareOutput::data_share_type): <p>The type of the datashare created by RegisterNamespace.</p>
    /// - On failure, responds with [`SdkError<AuthorizeDataShareError>`](crate::operation::authorize_data_share::AuthorizeDataShareError)
    pub fn authorize_data_share(&self) -> crate::operation::authorize_data_share::builders::AuthorizeDataShareFluentBuilder {
        crate::operation::authorize_data_share::builders::AuthorizeDataShareFluentBuilder::new(self.handle.clone())
    }
}
