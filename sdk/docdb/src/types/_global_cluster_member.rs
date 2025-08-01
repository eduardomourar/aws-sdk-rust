// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A data structure with information about any primary and secondary clusters associated with an Amazon DocumentDB global clusters.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GlobalClusterMember {
    /// <p>The Amazon Resource Name (ARN) for each Amazon DocumentDB cluster.</p>
    pub db_cluster_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) for each read-only secondary cluster associated with the Amazon DocumentDB global cluster.</p>
    pub readers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specifies whether the Amazon DocumentDB cluster is the primary cluster (that is, has read-write capability) for the Amazon DocumentDB global cluster with which it is associated.</p>
    pub is_writer: ::std::option::Option<bool>,
}
impl GlobalClusterMember {
    /// <p>The Amazon Resource Name (ARN) for each Amazon DocumentDB cluster.</p>
    pub fn db_cluster_arn(&self) -> ::std::option::Option<&str> {
        self.db_cluster_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) for each read-only secondary cluster associated with the Amazon DocumentDB global cluster.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.readers.is_none()`.
    pub fn readers(&self) -> &[::std::string::String] {
        self.readers.as_deref().unwrap_or_default()
    }
    /// <p>Specifies whether the Amazon DocumentDB cluster is the primary cluster (that is, has read-write capability) for the Amazon DocumentDB global cluster with which it is associated.</p>
    pub fn is_writer(&self) -> ::std::option::Option<bool> {
        self.is_writer
    }
}
impl GlobalClusterMember {
    /// Creates a new builder-style object to manufacture [`GlobalClusterMember`](crate::types::GlobalClusterMember).
    pub fn builder() -> crate::types::builders::GlobalClusterMemberBuilder {
        crate::types::builders::GlobalClusterMemberBuilder::default()
    }
}

/// A builder for [`GlobalClusterMember`](crate::types::GlobalClusterMember).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GlobalClusterMemberBuilder {
    pub(crate) db_cluster_arn: ::std::option::Option<::std::string::String>,
    pub(crate) readers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) is_writer: ::std::option::Option<bool>,
}
impl GlobalClusterMemberBuilder {
    /// <p>The Amazon Resource Name (ARN) for each Amazon DocumentDB cluster.</p>
    pub fn db_cluster_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.db_cluster_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for each Amazon DocumentDB cluster.</p>
    pub fn set_db_cluster_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.db_cluster_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for each Amazon DocumentDB cluster.</p>
    pub fn get_db_cluster_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.db_cluster_arn
    }
    /// Appends an item to `readers`.
    ///
    /// To override the contents of this collection use [`set_readers`](Self::set_readers).
    ///
    /// <p>The Amazon Resource Name (ARN) for each read-only secondary cluster associated with the Amazon DocumentDB global cluster.</p>
    pub fn readers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.readers.unwrap_or_default();
        v.push(input.into());
        self.readers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon Resource Name (ARN) for each read-only secondary cluster associated with the Amazon DocumentDB global cluster.</p>
    pub fn set_readers(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.readers = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for each read-only secondary cluster associated with the Amazon DocumentDB global cluster.</p>
    pub fn get_readers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.readers
    }
    /// <p>Specifies whether the Amazon DocumentDB cluster is the primary cluster (that is, has read-write capability) for the Amazon DocumentDB global cluster with which it is associated.</p>
    pub fn is_writer(mut self, input: bool) -> Self {
        self.is_writer = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the Amazon DocumentDB cluster is the primary cluster (that is, has read-write capability) for the Amazon DocumentDB global cluster with which it is associated.</p>
    pub fn set_is_writer(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_writer = input;
        self
    }
    /// <p>Specifies whether the Amazon DocumentDB cluster is the primary cluster (that is, has read-write capability) for the Amazon DocumentDB global cluster with which it is associated.</p>
    pub fn get_is_writer(&self) -> &::std::option::Option<bool> {
        &self.is_writer
    }
    /// Consumes the builder and constructs a [`GlobalClusterMember`](crate::types::GlobalClusterMember).
    pub fn build(self) -> crate::types::GlobalClusterMember {
        crate::types::GlobalClusterMember {
            db_cluster_arn: self.db_cluster_arn,
            readers: self.readers,
            is_writer: self.is_writer,
        }
    }
}
