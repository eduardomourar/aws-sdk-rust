// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure that represents user-provided metadata that can be associated with an IAM resource. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Tag {
    /// <p>The key name that can be used to look up or retrieve the associated value. For example, <code>Department</code> or <code>Cost Center</code> are common choices.</p>
    pub key: ::std::string::String,
    /// <p>The value associated with this tag. For example, tags with a key name of <code>Department</code> could have values such as <code>Human Resources</code>, <code>Accounting</code>, and <code>Support</code>. Tags with a key name of <code>Cost Center</code> might have values that consist of the number associated with the different cost centers in your company. Typically, many resources have tags with the same key name but with different values.</p>
    pub value: ::std::string::String,
}
impl Tag {
    /// <p>The key name that can be used to look up or retrieve the associated value. For example, <code>Department</code> or <code>Cost Center</code> are common choices.</p>
    pub fn key(&self) -> &str {
        use std::ops::Deref;
        self.key.deref()
    }
    /// <p>The value associated with this tag. For example, tags with a key name of <code>Department</code> could have values such as <code>Human Resources</code>, <code>Accounting</code>, and <code>Support</code>. Tags with a key name of <code>Cost Center</code> might have values that consist of the number associated with the different cost centers in your company. Typically, many resources have tags with the same key name but with different values.</p>
    pub fn value(&self) -> &str {
        use std::ops::Deref;
        self.value.deref()
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::types::Tag).
    pub fn builder() -> crate::types::builders::TagBuilder {
        crate::types::builders::TagBuilder::default()
    }
}

/// A builder for [`Tag`](crate::types::Tag).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TagBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl TagBuilder {
    /// <p>The key name that can be used to look up or retrieve the associated value. For example, <code>Department</code> or <code>Cost Center</code> are common choices.</p>
    /// This field is required.
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key name that can be used to look up or retrieve the associated value. For example, <code>Department</code> or <code>Cost Center</code> are common choices.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>The key name that can be used to look up or retrieve the associated value. For example, <code>Department</code> or <code>Cost Center</code> are common choices.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    /// <p>The value associated with this tag. For example, tags with a key name of <code>Department</code> could have values such as <code>Human Resources</code>, <code>Accounting</code>, and <code>Support</code>. Tags with a key name of <code>Cost Center</code> might have values that consist of the number associated with the different cost centers in your company. Typically, many resources have tags with the same key name but with different values.</p>
    /// This field is required.
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value associated with this tag. For example, tags with a key name of <code>Department</code> could have values such as <code>Human Resources</code>, <code>Accounting</code>, and <code>Support</code>. Tags with a key name of <code>Cost Center</code> might have values that consist of the number associated with the different cost centers in your company. Typically, many resources have tags with the same key name but with different values.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>The value associated with this tag. For example, tags with a key name of <code>Department</code> could have values such as <code>Human Resources</code>, <code>Accounting</code>, and <code>Support</code>. Tags with a key name of <code>Cost Center</code> might have values that consist of the number associated with the different cost centers in your company. Typically, many resources have tags with the same key name but with different values.</p>
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.value
    }
    /// Consumes the builder and constructs a [`Tag`](crate::types::Tag).
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](crate::types::builders::TagBuilder::key)
    /// - [`value`](crate::types::builders::TagBuilder::value)
    pub fn build(self) -> ::std::result::Result<crate::types::Tag, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Tag {
            key: self.key.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field("key", "key was not specified but it is required when building Tag")
            })?,
            value: self.value.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "value",
                    "value was not specified but it is required when building Tag",
                )
            })?,
        })
    }
}
