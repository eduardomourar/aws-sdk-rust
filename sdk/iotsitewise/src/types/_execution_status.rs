// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The status of the execution.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExecutionStatus {
    /// <p>The current state of the computation model.</p>
    pub state: crate::types::ExecutionState,
}
impl ExecutionStatus {
    /// <p>The current state of the computation model.</p>
    pub fn state(&self) -> &crate::types::ExecutionState {
        &self.state
    }
}
impl ExecutionStatus {
    /// Creates a new builder-style object to manufacture [`ExecutionStatus`](crate::types::ExecutionStatus).
    pub fn builder() -> crate::types::builders::ExecutionStatusBuilder {
        crate::types::builders::ExecutionStatusBuilder::default()
    }
}

/// A builder for [`ExecutionStatus`](crate::types::ExecutionStatus).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ExecutionStatusBuilder {
    pub(crate) state: ::std::option::Option<crate::types::ExecutionState>,
}
impl ExecutionStatusBuilder {
    /// <p>The current state of the computation model.</p>
    /// This field is required.
    pub fn state(mut self, input: crate::types::ExecutionState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current state of the computation model.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::ExecutionState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The current state of the computation model.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::ExecutionState> {
        &self.state
    }
    /// Consumes the builder and constructs a [`ExecutionStatus`](crate::types::ExecutionStatus).
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](crate::types::builders::ExecutionStatusBuilder::state)
    pub fn build(self) -> ::std::result::Result<crate::types::ExecutionStatus, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ExecutionStatus {
            state: self.state.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "state",
                    "state was not specified but it is required when building ExecutionStatus",
                )
            })?,
        })
    }
}
