// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains a reference to a <code>Statement</code> element in a policy document that determines the result of the simulation.</p>
/// <p>This data type is used by the <code>MatchedStatements</code> member of the <code> <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_EvaluationResult.html">EvaluationResult</a> </code> type.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Statement {
    /// <p>The identifier of the policy that was provided as an input.</p>
    pub source_policy_id: ::std::option::Option<::std::string::String>,
    /// <p>The type of the policy.</p>
    pub source_policy_type: ::std::option::Option<crate::types::PolicySourceType>,
    /// <p>The row and column of the beginning of the <code>Statement</code> in an IAM policy.</p>
    pub start_position: ::std::option::Option<crate::types::Position>,
    /// <p>The row and column of the end of a <code>Statement</code> in an IAM policy.</p>
    pub end_position: ::std::option::Option<crate::types::Position>,
}
impl Statement {
    /// <p>The identifier of the policy that was provided as an input.</p>
    pub fn source_policy_id(&self) -> ::std::option::Option<&str> {
        self.source_policy_id.as_deref()
    }
    /// <p>The type of the policy.</p>
    pub fn source_policy_type(&self) -> ::std::option::Option<&crate::types::PolicySourceType> {
        self.source_policy_type.as_ref()
    }
    /// <p>The row and column of the beginning of the <code>Statement</code> in an IAM policy.</p>
    pub fn start_position(&self) -> ::std::option::Option<&crate::types::Position> {
        self.start_position.as_ref()
    }
    /// <p>The row and column of the end of a <code>Statement</code> in an IAM policy.</p>
    pub fn end_position(&self) -> ::std::option::Option<&crate::types::Position> {
        self.end_position.as_ref()
    }
}
impl Statement {
    /// Creates a new builder-style object to manufacture [`Statement`](crate::types::Statement).
    pub fn builder() -> crate::types::builders::StatementBuilder {
        crate::types::builders::StatementBuilder::default()
    }
}

/// A builder for [`Statement`](crate::types::Statement).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StatementBuilder {
    pub(crate) source_policy_id: ::std::option::Option<::std::string::String>,
    pub(crate) source_policy_type: ::std::option::Option<crate::types::PolicySourceType>,
    pub(crate) start_position: ::std::option::Option<crate::types::Position>,
    pub(crate) end_position: ::std::option::Option<crate::types::Position>,
}
impl StatementBuilder {
    /// <p>The identifier of the policy that was provided as an input.</p>
    pub fn source_policy_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_policy_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the policy that was provided as an input.</p>
    pub fn set_source_policy_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_policy_id = input;
        self
    }
    /// <p>The identifier of the policy that was provided as an input.</p>
    pub fn get_source_policy_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_policy_id
    }
    /// <p>The type of the policy.</p>
    pub fn source_policy_type(mut self, input: crate::types::PolicySourceType) -> Self {
        self.source_policy_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the policy.</p>
    pub fn set_source_policy_type(mut self, input: ::std::option::Option<crate::types::PolicySourceType>) -> Self {
        self.source_policy_type = input;
        self
    }
    /// <p>The type of the policy.</p>
    pub fn get_source_policy_type(&self) -> &::std::option::Option<crate::types::PolicySourceType> {
        &self.source_policy_type
    }
    /// <p>The row and column of the beginning of the <code>Statement</code> in an IAM policy.</p>
    pub fn start_position(mut self, input: crate::types::Position) -> Self {
        self.start_position = ::std::option::Option::Some(input);
        self
    }
    /// <p>The row and column of the beginning of the <code>Statement</code> in an IAM policy.</p>
    pub fn set_start_position(mut self, input: ::std::option::Option<crate::types::Position>) -> Self {
        self.start_position = input;
        self
    }
    /// <p>The row and column of the beginning of the <code>Statement</code> in an IAM policy.</p>
    pub fn get_start_position(&self) -> &::std::option::Option<crate::types::Position> {
        &self.start_position
    }
    /// <p>The row and column of the end of a <code>Statement</code> in an IAM policy.</p>
    pub fn end_position(mut self, input: crate::types::Position) -> Self {
        self.end_position = ::std::option::Option::Some(input);
        self
    }
    /// <p>The row and column of the end of a <code>Statement</code> in an IAM policy.</p>
    pub fn set_end_position(mut self, input: ::std::option::Option<crate::types::Position>) -> Self {
        self.end_position = input;
        self
    }
    /// <p>The row and column of the end of a <code>Statement</code> in an IAM policy.</p>
    pub fn get_end_position(&self) -> &::std::option::Option<crate::types::Position> {
        &self.end_position
    }
    /// Consumes the builder and constructs a [`Statement`](crate::types::Statement).
    pub fn build(self) -> crate::types::Statement {
        crate::types::Statement {
            source_policy_id: self.source_policy_id,
            source_policy_type: self.source_policy_type,
            start_position: self.start_position,
            end_position: self.end_position,
        }
    }
}
