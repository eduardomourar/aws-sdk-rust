// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_knowledge_base::_create_knowledge_base_output::CreateKnowledgeBaseOutputBuilder;

pub use crate::operation::create_knowledge_base::_create_knowledge_base_input::CreateKnowledgeBaseInputBuilder;

impl crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_knowledge_base::CreateKnowledgeBaseOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_knowledge_base::CreateKnowledgeBaseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_knowledge_base();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateKnowledgeBase`.
///
/// <p>Creates a knowledge base. A knowledge base contains your data sources so that Large Language Models (LLMs) can use your data. To create a knowledge base, you must first set up your data sources and configure a supported vector store. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/knowlege-base-prereq.html">Set up a knowledge base</a>.</p><note>
/// <p>If you prefer to let Amazon Bedrock create and manage a vector store for you in Amazon OpenSearch Service, use the console. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/knowledge-base-create">Create a knowledge base</a>.</p>
/// </note>
/// <ul>
/// <li>
/// <p>Provide the <code>name</code> and an optional <code>description</code>.</p></li>
/// <li>
/// <p>Provide the Amazon Resource Name (ARN) with permissions to create a knowledge base in the <code>roleArn</code> field.</p></li>
/// <li>
/// <p>Provide the embedding model to use in the <code>embeddingModelArn</code> field in the <code>knowledgeBaseConfiguration</code> object.</p></li>
/// <li>
/// <p>Provide the configuration for your vector store in the <code>storageConfiguration</code> object.</p>
/// <ul>
/// <li>
/// <p>For an Amazon OpenSearch Service database, use the <code>opensearchServerlessConfiguration</code> object. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/knowledge-base-setup-oss.html">Create a vector store in Amazon OpenSearch Service</a>.</p></li>
/// <li>
/// <p>For an Amazon Aurora database, use the <code>RdsConfiguration</code> object. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/knowledge-base-setup-rds.html">Create a vector store in Amazon Aurora</a>.</p></li>
/// <li>
/// <p>For a Pinecone database, use the <code>pineconeConfiguration</code> object. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/knowledge-base-setup-pinecone.html">Create a vector store in Pinecone</a>.</p></li>
/// <li>
/// <p>For a Redis Enterprise Cloud database, use the <code>redisEnterpriseCloudConfiguration</code> object. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/knowledge-base-setup-redis.html">Create a vector store in Redis Enterprise Cloud</a>.</p></li>
/// </ul></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateKnowledgeBaseFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_knowledge_base::CreateKnowledgeBaseOutput,
        crate::operation::create_knowledge_base::CreateKnowledgeBaseError,
    > for CreateKnowledgeBaseFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_knowledge_base::CreateKnowledgeBaseOutput,
            crate::operation::create_knowledge_base::CreateKnowledgeBaseError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateKnowledgeBaseFluentBuilder {
    /// Creates a new `CreateKnowledgeBaseFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateKnowledgeBase as a reference.
    pub fn as_input(&self) -> &crate::operation::create_knowledge_base::builders::CreateKnowledgeBaseInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_knowledge_base::CreateKnowledgeBaseOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_knowledge_base::CreateKnowledgeBaseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_knowledge_base::CreateKnowledgeBase::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_knowledge_base::CreateKnowledgeBase::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_knowledge_base::CreateKnowledgeBaseOutput,
        crate::operation::create_knowledge_base::CreateKnowledgeBaseError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>A unique, case-sensitive identifier to ensure that the API request completes no more than one time. If this token matches a previous request, Amazon Bedrock ignores the request, but does not return an error. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier to ensure that the API request completes no more than one time. If this token matches a previous request, Amazon Bedrock ignores the request, but does not return an error. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier to ensure that the API request completes no more than one time. If this token matches a previous request, Amazon Bedrock ignores the request, but does not return an error. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>A name for the knowledge base.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A name for the knowledge base.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A name for the knowledge base.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A description of the knowledge base.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the knowledge base.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the knowledge base.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role with permissions to invoke API operations on the knowledge base.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role with permissions to invoke API operations on the knowledge base.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role with permissions to invoke API operations on the knowledge base.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// <p>Contains details about the embeddings model used for the knowledge base.</p>
    pub fn knowledge_base_configuration(mut self, input: crate::types::KnowledgeBaseConfiguration) -> Self {
        self.inner = self.inner.knowledge_base_configuration(input);
        self
    }
    /// <p>Contains details about the embeddings model used for the knowledge base.</p>
    pub fn set_knowledge_base_configuration(mut self, input: ::std::option::Option<crate::types::KnowledgeBaseConfiguration>) -> Self {
        self.inner = self.inner.set_knowledge_base_configuration(input);
        self
    }
    /// <p>Contains details about the embeddings model used for the knowledge base.</p>
    pub fn get_knowledge_base_configuration(&self) -> &::std::option::Option<crate::types::KnowledgeBaseConfiguration> {
        self.inner.get_knowledge_base_configuration()
    }
    /// <p>Contains details about the configuration of the vector database used for the knowledge base.</p>
    pub fn storage_configuration(mut self, input: crate::types::StorageConfiguration) -> Self {
        self.inner = self.inner.storage_configuration(input);
        self
    }
    /// <p>Contains details about the configuration of the vector database used for the knowledge base.</p>
    pub fn set_storage_configuration(mut self, input: ::std::option::Option<crate::types::StorageConfiguration>) -> Self {
        self.inner = self.inner.set_storage_configuration(input);
        self
    }
    /// <p>Contains details about the configuration of the vector database used for the knowledge base.</p>
    pub fn get_storage_configuration(&self) -> &::std::option::Option<crate::types::StorageConfiguration> {
        self.inner.get_storage_configuration()
    }
    ///
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Specify the key-value pairs for the tags that you want to attach to your knowledge base in this object.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Specify the key-value pairs for the tags that you want to attach to your knowledge base in this object.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Specify the key-value pairs for the tags that you want to attach to your knowledge base in this object.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
