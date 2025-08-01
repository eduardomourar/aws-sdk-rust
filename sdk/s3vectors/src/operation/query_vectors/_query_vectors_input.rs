// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct QueryVectorsInput {
    /// <p>The name of the vector bucket that contains the vector index.</p>
    pub vector_bucket_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the vector index that you want to query.</p>
    pub index_name: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the vector index that you want to query.</p>
    pub index_arn: ::std::option::Option<::std::string::String>,
    /// <p>The number of results to return for each query.</p>
    pub top_k: ::std::option::Option<i32>,
    /// <p>The query vector. Ensure that the query vector has the same dimension as the dimension of the vector index that's being queried. For example, if your vector index contains vectors with 384 dimensions, your query vector must also have 384 dimensions.</p>
    pub query_vector: ::std::option::Option<crate::types::VectorData>,
    /// <p>Metadata filter to apply during the query. For more information about metadata keys, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-vectors-metadata-filtering.html">Metadata filtering</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub filter: ::std::option::Option<::aws_smithy_types::Document>,
    /// <p>Indicates whether to include metadata in the response. The default value is <code>false</code>.</p>
    pub return_metadata: ::std::option::Option<bool>,
    /// <p>Indicates whether to include the computed distance in the response. The default value is <code>false</code>.</p>
    pub return_distance: ::std::option::Option<bool>,
}
impl QueryVectorsInput {
    /// <p>The name of the vector bucket that contains the vector index.</p>
    pub fn vector_bucket_name(&self) -> ::std::option::Option<&str> {
        self.vector_bucket_name.as_deref()
    }
    /// <p>The name of the vector index that you want to query.</p>
    pub fn index_name(&self) -> ::std::option::Option<&str> {
        self.index_name.as_deref()
    }
    /// <p>The ARN of the vector index that you want to query.</p>
    pub fn index_arn(&self) -> ::std::option::Option<&str> {
        self.index_arn.as_deref()
    }
    /// <p>The number of results to return for each query.</p>
    pub fn top_k(&self) -> ::std::option::Option<i32> {
        self.top_k
    }
    /// <p>The query vector. Ensure that the query vector has the same dimension as the dimension of the vector index that's being queried. For example, if your vector index contains vectors with 384 dimensions, your query vector must also have 384 dimensions.</p>
    pub fn query_vector(&self) -> ::std::option::Option<&crate::types::VectorData> {
        self.query_vector.as_ref()
    }
    /// <p>Metadata filter to apply during the query. For more information about metadata keys, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-vectors-metadata-filtering.html">Metadata filtering</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn filter(&self) -> ::std::option::Option<&::aws_smithy_types::Document> {
        self.filter.as_ref()
    }
    /// <p>Indicates whether to include metadata in the response. The default value is <code>false</code>.</p>
    pub fn return_metadata(&self) -> ::std::option::Option<bool> {
        self.return_metadata
    }
    /// <p>Indicates whether to include the computed distance in the response. The default value is <code>false</code>.</p>
    pub fn return_distance(&self) -> ::std::option::Option<bool> {
        self.return_distance
    }
}
impl QueryVectorsInput {
    /// Creates a new builder-style object to manufacture [`QueryVectorsInput`](crate::operation::query_vectors::QueryVectorsInput).
    pub fn builder() -> crate::operation::query_vectors::builders::QueryVectorsInputBuilder {
        crate::operation::query_vectors::builders::QueryVectorsInputBuilder::default()
    }
}

/// A builder for [`QueryVectorsInput`](crate::operation::query_vectors::QueryVectorsInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct QueryVectorsInputBuilder {
    pub(crate) vector_bucket_name: ::std::option::Option<::std::string::String>,
    pub(crate) index_name: ::std::option::Option<::std::string::String>,
    pub(crate) index_arn: ::std::option::Option<::std::string::String>,
    pub(crate) top_k: ::std::option::Option<i32>,
    pub(crate) query_vector: ::std::option::Option<crate::types::VectorData>,
    pub(crate) filter: ::std::option::Option<::aws_smithy_types::Document>,
    pub(crate) return_metadata: ::std::option::Option<bool>,
    pub(crate) return_distance: ::std::option::Option<bool>,
}
impl QueryVectorsInputBuilder {
    /// <p>The name of the vector bucket that contains the vector index.</p>
    pub fn vector_bucket_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vector_bucket_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the vector bucket that contains the vector index.</p>
    pub fn set_vector_bucket_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vector_bucket_name = input;
        self
    }
    /// <p>The name of the vector bucket that contains the vector index.</p>
    pub fn get_vector_bucket_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.vector_bucket_name
    }
    /// <p>The name of the vector index that you want to query.</p>
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the vector index that you want to query.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// <p>The name of the vector index that you want to query.</p>
    pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_name
    }
    /// <p>The ARN of the vector index that you want to query.</p>
    pub fn index_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the vector index that you want to query.</p>
    pub fn set_index_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_arn = input;
        self
    }
    /// <p>The ARN of the vector index that you want to query.</p>
    pub fn get_index_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_arn
    }
    /// <p>The number of results to return for each query.</p>
    /// This field is required.
    pub fn top_k(mut self, input: i32) -> Self {
        self.top_k = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of results to return for each query.</p>
    pub fn set_top_k(mut self, input: ::std::option::Option<i32>) -> Self {
        self.top_k = input;
        self
    }
    /// <p>The number of results to return for each query.</p>
    pub fn get_top_k(&self) -> &::std::option::Option<i32> {
        &self.top_k
    }
    /// <p>The query vector. Ensure that the query vector has the same dimension as the dimension of the vector index that's being queried. For example, if your vector index contains vectors with 384 dimensions, your query vector must also have 384 dimensions.</p>
    /// This field is required.
    pub fn query_vector(mut self, input: crate::types::VectorData) -> Self {
        self.query_vector = ::std::option::Option::Some(input);
        self
    }
    /// <p>The query vector. Ensure that the query vector has the same dimension as the dimension of the vector index that's being queried. For example, if your vector index contains vectors with 384 dimensions, your query vector must also have 384 dimensions.</p>
    pub fn set_query_vector(mut self, input: ::std::option::Option<crate::types::VectorData>) -> Self {
        self.query_vector = input;
        self
    }
    /// <p>The query vector. Ensure that the query vector has the same dimension as the dimension of the vector index that's being queried. For example, if your vector index contains vectors with 384 dimensions, your query vector must also have 384 dimensions.</p>
    pub fn get_query_vector(&self) -> &::std::option::Option<crate::types::VectorData> {
        &self.query_vector
    }
    /// <p>Metadata filter to apply during the query. For more information about metadata keys, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-vectors-metadata-filtering.html">Metadata filtering</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn filter(mut self, input: ::aws_smithy_types::Document) -> Self {
        self.filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>Metadata filter to apply during the query. For more information about metadata keys, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-vectors-metadata-filtering.html">Metadata filtering</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_filter(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
        self.filter = input;
        self
    }
    /// <p>Metadata filter to apply during the query. For more information about metadata keys, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-vectors-metadata-filtering.html">Metadata filtering</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_filter(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
        &self.filter
    }
    /// <p>Indicates whether to include metadata in the response. The default value is <code>false</code>.</p>
    pub fn return_metadata(mut self, input: bool) -> Self {
        self.return_metadata = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to include metadata in the response. The default value is <code>false</code>.</p>
    pub fn set_return_metadata(mut self, input: ::std::option::Option<bool>) -> Self {
        self.return_metadata = input;
        self
    }
    /// <p>Indicates whether to include metadata in the response. The default value is <code>false</code>.</p>
    pub fn get_return_metadata(&self) -> &::std::option::Option<bool> {
        &self.return_metadata
    }
    /// <p>Indicates whether to include the computed distance in the response. The default value is <code>false</code>.</p>
    pub fn return_distance(mut self, input: bool) -> Self {
        self.return_distance = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to include the computed distance in the response. The default value is <code>false</code>.</p>
    pub fn set_return_distance(mut self, input: ::std::option::Option<bool>) -> Self {
        self.return_distance = input;
        self
    }
    /// <p>Indicates whether to include the computed distance in the response. The default value is <code>false</code>.</p>
    pub fn get_return_distance(&self) -> &::std::option::Option<bool> {
        &self.return_distance
    }
    /// Consumes the builder and constructs a [`QueryVectorsInput`](crate::operation::query_vectors::QueryVectorsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::query_vectors::QueryVectorsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::query_vectors::QueryVectorsInput {
            vector_bucket_name: self.vector_bucket_name,
            index_name: self.index_name,
            index_arn: self.index_arn,
            top_k: self.top_k,
            query_vector: self.query_vector,
            filter: self.filter,
            return_metadata: self.return_metadata,
            return_distance: self.return_distance,
        })
    }
}
