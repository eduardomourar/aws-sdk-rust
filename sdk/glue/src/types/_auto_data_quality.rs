// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies configuration options for automatic data quality evaluation in Glue jobs. This structure enables automated data quality checks and monitoring during ETL operations, helping to ensure data integrity and reliability without manual intervention.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AutoDataQuality {
    /// <p>Specifies whether automatic data quality evaluation is enabled. When set to <code>true</code>, data quality checks are performed automatically.</p>
    pub is_enabled: bool,
    /// <p>The evaluation context for the automatic data quality checks. This defines the scope and parameters for the data quality evaluation.</p>
    pub evaluation_context: ::std::option::Option<::std::string::String>,
}
impl AutoDataQuality {
    /// <p>Specifies whether automatic data quality evaluation is enabled. When set to <code>true</code>, data quality checks are performed automatically.</p>
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
    /// <p>The evaluation context for the automatic data quality checks. This defines the scope and parameters for the data quality evaluation.</p>
    pub fn evaluation_context(&self) -> ::std::option::Option<&str> {
        self.evaluation_context.as_deref()
    }
}
impl AutoDataQuality {
    /// Creates a new builder-style object to manufacture [`AutoDataQuality`](crate::types::AutoDataQuality).
    pub fn builder() -> crate::types::builders::AutoDataQualityBuilder {
        crate::types::builders::AutoDataQualityBuilder::default()
    }
}

/// A builder for [`AutoDataQuality`](crate::types::AutoDataQuality).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AutoDataQualityBuilder {
    pub(crate) is_enabled: ::std::option::Option<bool>,
    pub(crate) evaluation_context: ::std::option::Option<::std::string::String>,
}
impl AutoDataQualityBuilder {
    /// <p>Specifies whether automatic data quality evaluation is enabled. When set to <code>true</code>, data quality checks are performed automatically.</p>
    pub fn is_enabled(mut self, input: bool) -> Self {
        self.is_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether automatic data quality evaluation is enabled. When set to <code>true</code>, data quality checks are performed automatically.</p>
    pub fn set_is_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_enabled = input;
        self
    }
    /// <p>Specifies whether automatic data quality evaluation is enabled. When set to <code>true</code>, data quality checks are performed automatically.</p>
    pub fn get_is_enabled(&self) -> &::std::option::Option<bool> {
        &self.is_enabled
    }
    /// <p>The evaluation context for the automatic data quality checks. This defines the scope and parameters for the data quality evaluation.</p>
    pub fn evaluation_context(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.evaluation_context = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The evaluation context for the automatic data quality checks. This defines the scope and parameters for the data quality evaluation.</p>
    pub fn set_evaluation_context(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.evaluation_context = input;
        self
    }
    /// <p>The evaluation context for the automatic data quality checks. This defines the scope and parameters for the data quality evaluation.</p>
    pub fn get_evaluation_context(&self) -> &::std::option::Option<::std::string::String> {
        &self.evaluation_context
    }
    /// Consumes the builder and constructs a [`AutoDataQuality`](crate::types::AutoDataQuality).
    pub fn build(self) -> crate::types::AutoDataQuality {
        crate::types::AutoDataQuality {
            is_enabled: self.is_enabled.unwrap_or_default(),
            evaluation_context: self.evaluation_context,
        }
    }
}
