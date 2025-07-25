// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAccountPlanStateInput {}
impl GetAccountPlanStateInput {
    /// Creates a new builder-style object to manufacture [`GetAccountPlanStateInput`](crate::operation::get_account_plan_state::GetAccountPlanStateInput).
    pub fn builder() -> crate::operation::get_account_plan_state::builders::GetAccountPlanStateInputBuilder {
        crate::operation::get_account_plan_state::builders::GetAccountPlanStateInputBuilder::default()
    }
}

/// A builder for [`GetAccountPlanStateInput`](crate::operation::get_account_plan_state::GetAccountPlanStateInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetAccountPlanStateInputBuilder {}
impl GetAccountPlanStateInputBuilder {
    /// Consumes the builder and constructs a [`GetAccountPlanStateInput`](crate::operation::get_account_plan_state::GetAccountPlanStateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_account_plan_state::GetAccountPlanStateInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::get_account_plan_state::GetAccountPlanStateInput {})
    }
}
