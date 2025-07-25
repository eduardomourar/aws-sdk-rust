// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a condition context key. It includes the name of the key and specifies the value (or values, if the context key supports multiple values) to use in the simulation. This information is used when evaluating the <code>Condition</code> elements of the input policies.</p>
/// <p>This data type is used as an input parameter to <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_SimulateCustomPolicy.html">SimulateCustomPolicy</a> and <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_SimulatePrincipalPolicy.html">SimulatePrincipalPolicy</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ContextEntry {
    /// <p>The full name of a condition context key, including the service prefix. For example, <code>aws:SourceIp</code> or <code>s3:VersionId</code>.</p>
    pub context_key_name: ::std::option::Option<::std::string::String>,
    /// <p>The value (or values, if the condition context key supports multiple values) to provide to the simulation when the key is referenced by a <code>Condition</code> element in an input policy.</p>
    pub context_key_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The data type of the value (or values) specified in the <code>ContextKeyValues</code> parameter.</p>
    pub context_key_type: ::std::option::Option<crate::types::ContextKeyTypeEnum>,
}
impl ContextEntry {
    /// <p>The full name of a condition context key, including the service prefix. For example, <code>aws:SourceIp</code> or <code>s3:VersionId</code>.</p>
    pub fn context_key_name(&self) -> ::std::option::Option<&str> {
        self.context_key_name.as_deref()
    }
    /// <p>The value (or values, if the condition context key supports multiple values) to provide to the simulation when the key is referenced by a <code>Condition</code> element in an input policy.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.context_key_values.is_none()`.
    pub fn context_key_values(&self) -> &[::std::string::String] {
        self.context_key_values.as_deref().unwrap_or_default()
    }
    /// <p>The data type of the value (or values) specified in the <code>ContextKeyValues</code> parameter.</p>
    pub fn context_key_type(&self) -> ::std::option::Option<&crate::types::ContextKeyTypeEnum> {
        self.context_key_type.as_ref()
    }
}
impl ContextEntry {
    /// Creates a new builder-style object to manufacture [`ContextEntry`](crate::types::ContextEntry).
    pub fn builder() -> crate::types::builders::ContextEntryBuilder {
        crate::types::builders::ContextEntryBuilder::default()
    }
}

/// A builder for [`ContextEntry`](crate::types::ContextEntry).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ContextEntryBuilder {
    pub(crate) context_key_name: ::std::option::Option<::std::string::String>,
    pub(crate) context_key_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) context_key_type: ::std::option::Option<crate::types::ContextKeyTypeEnum>,
}
impl ContextEntryBuilder {
    /// <p>The full name of a condition context key, including the service prefix. For example, <code>aws:SourceIp</code> or <code>s3:VersionId</code>.</p>
    pub fn context_key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.context_key_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The full name of a condition context key, including the service prefix. For example, <code>aws:SourceIp</code> or <code>s3:VersionId</code>.</p>
    pub fn set_context_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.context_key_name = input;
        self
    }
    /// <p>The full name of a condition context key, including the service prefix. For example, <code>aws:SourceIp</code> or <code>s3:VersionId</code>.</p>
    pub fn get_context_key_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.context_key_name
    }
    /// Appends an item to `context_key_values`.
    ///
    /// To override the contents of this collection use [`set_context_key_values`](Self::set_context_key_values).
    ///
    /// <p>The value (or values, if the condition context key supports multiple values) to provide to the simulation when the key is referenced by a <code>Condition</code> element in an input policy.</p>
    pub fn context_key_values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.context_key_values.unwrap_or_default();
        v.push(input.into());
        self.context_key_values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The value (or values, if the condition context key supports multiple values) to provide to the simulation when the key is referenced by a <code>Condition</code> element in an input policy.</p>
    pub fn set_context_key_values(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.context_key_values = input;
        self
    }
    /// <p>The value (or values, if the condition context key supports multiple values) to provide to the simulation when the key is referenced by a <code>Condition</code> element in an input policy.</p>
    pub fn get_context_key_values(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.context_key_values
    }
    /// <p>The data type of the value (or values) specified in the <code>ContextKeyValues</code> parameter.</p>
    pub fn context_key_type(mut self, input: crate::types::ContextKeyTypeEnum) -> Self {
        self.context_key_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data type of the value (or values) specified in the <code>ContextKeyValues</code> parameter.</p>
    pub fn set_context_key_type(mut self, input: ::std::option::Option<crate::types::ContextKeyTypeEnum>) -> Self {
        self.context_key_type = input;
        self
    }
    /// <p>The data type of the value (or values) specified in the <code>ContextKeyValues</code> parameter.</p>
    pub fn get_context_key_type(&self) -> &::std::option::Option<crate::types::ContextKeyTypeEnum> {
        &self.context_key_type
    }
    /// Consumes the builder and constructs a [`ContextEntry`](crate::types::ContextEntry).
    pub fn build(self) -> crate::types::ContextEntry {
        crate::types::ContextEntry {
            context_key_name: self.context_key_name,
            context_key_values: self.context_key_values,
            context_key_type: self.context_key_type,
        }
    }
}
