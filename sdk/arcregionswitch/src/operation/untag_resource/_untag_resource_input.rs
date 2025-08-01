// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) for a tag you remove a resource from.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>Tag keys that you remove from a resource.</p>
    pub resource_tag_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) for a tag you remove a resource from.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>Tag keys that you remove from a resource.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.resource_tag_keys.is_none()`.
    pub fn resource_tag_keys(&self) -> &[::std::string::String] {
        self.resource_tag_keys.as_deref().unwrap_or_default()
    }
}
impl UntagResourceInput {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::operation::untag_resource::UntagResourceInput).
    pub fn builder() -> crate::operation::untag_resource::builders::UntagResourceInputBuilder {
        crate::operation::untag_resource::builders::UntagResourceInputBuilder::default()
    }
}

/// A builder for [`UntagResourceInput`](crate::operation::untag_resource::UntagResourceInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UntagResourceInputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) resource_tag_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UntagResourceInputBuilder {
    /// <p>The Amazon Resource Name (ARN) for a tag you remove a resource from.</p>
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for a tag you remove a resource from.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for a tag you remove a resource from.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// Appends an item to `resource_tag_keys`.
    ///
    /// To override the contents of this collection use [`set_resource_tag_keys`](Self::set_resource_tag_keys).
    ///
    /// <p>Tag keys that you remove from a resource.</p>
    pub fn resource_tag_keys(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.resource_tag_keys.unwrap_or_default();
        v.push(input.into());
        self.resource_tag_keys = ::std::option::Option::Some(v);
        self
    }
    /// <p>Tag keys that you remove from a resource.</p>
    pub fn set_resource_tag_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.resource_tag_keys = input;
        self
    }
    /// <p>Tag keys that you remove from a resource.</p>
    pub fn get_resource_tag_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.resource_tag_keys
    }
    /// Consumes the builder and constructs a [`UntagResourceInput`](crate::operation::untag_resource::UntagResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::untag_resource::UntagResourceInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::untag_resource::UntagResourceInput {
            arn: self.arn,
            resource_tag_keys: self.resource_tag_keys,
        })
    }
}
