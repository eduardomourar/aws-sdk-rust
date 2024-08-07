// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides details of the foundation model.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct Metadata {
    /// <p>Contains details of the foundation model usage.</p>
    pub usage: ::std::option::Option<crate::types::Usage>,
}
impl Metadata {
    /// <p>Contains details of the foundation model usage.</p>
    pub fn usage(&self) -> ::std::option::Option<&crate::types::Usage> {
        self.usage.as_ref()
    }
}
impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("Metadata");
        formatter.field("usage", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl Metadata {
    /// Creates a new builder-style object to manufacture [`Metadata`](crate::types::Metadata).
    pub fn builder() -> crate::types::builders::MetadataBuilder {
        crate::types::builders::MetadataBuilder::default()
    }
}

/// A builder for [`Metadata`](crate::types::Metadata).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct MetadataBuilder {
    pub(crate) usage: ::std::option::Option<crate::types::Usage>,
}
impl MetadataBuilder {
    /// <p>Contains details of the foundation model usage.</p>
    pub fn usage(mut self, input: crate::types::Usage) -> Self {
        self.usage = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains details of the foundation model usage.</p>
    pub fn set_usage(mut self, input: ::std::option::Option<crate::types::Usage>) -> Self {
        self.usage = input;
        self
    }
    /// <p>Contains details of the foundation model usage.</p>
    pub fn get_usage(&self) -> &::std::option::Option<crate::types::Usage> {
        &self.usage
    }
    /// Consumes the builder and constructs a [`Metadata`](crate::types::Metadata).
    pub fn build(self) -> crate::types::Metadata {
        crate::types::Metadata { usage: self.usage }
    }
}
impl ::std::fmt::Debug for MetadataBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("MetadataBuilder");
        formatter.field("usage", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
