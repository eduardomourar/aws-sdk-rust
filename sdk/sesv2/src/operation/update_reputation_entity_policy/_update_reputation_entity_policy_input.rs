// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to update the reputation management policy for a reputation entity.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateReputationEntityPolicyInput {
    /// <p>The type of reputation entity. Currently, only <code>RESOURCE</code> type entities are supported.</p>
    pub reputation_entity_type: ::std::option::Option<crate::types::ReputationEntityType>,
    /// <p>The unique identifier for the reputation entity. For resource-type entities, this is the Amazon Resource Name (ARN) of the resource.</p>
    pub reputation_entity_reference: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the reputation management policy to apply to this entity. This is an Amazon Web Services Amazon SES-managed policy.</p>
    pub reputation_entity_policy: ::std::option::Option<::std::string::String>,
}
impl UpdateReputationEntityPolicyInput {
    /// <p>The type of reputation entity. Currently, only <code>RESOURCE</code> type entities are supported.</p>
    pub fn reputation_entity_type(&self) -> ::std::option::Option<&crate::types::ReputationEntityType> {
        self.reputation_entity_type.as_ref()
    }
    /// <p>The unique identifier for the reputation entity. For resource-type entities, this is the Amazon Resource Name (ARN) of the resource.</p>
    pub fn reputation_entity_reference(&self) -> ::std::option::Option<&str> {
        self.reputation_entity_reference.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the reputation management policy to apply to this entity. This is an Amazon Web Services Amazon SES-managed policy.</p>
    pub fn reputation_entity_policy(&self) -> ::std::option::Option<&str> {
        self.reputation_entity_policy.as_deref()
    }
}
impl UpdateReputationEntityPolicyInput {
    /// Creates a new builder-style object to manufacture [`UpdateReputationEntityPolicyInput`](crate::operation::update_reputation_entity_policy::UpdateReputationEntityPolicyInput).
    pub fn builder() -> crate::operation::update_reputation_entity_policy::builders::UpdateReputationEntityPolicyInputBuilder {
        crate::operation::update_reputation_entity_policy::builders::UpdateReputationEntityPolicyInputBuilder::default()
    }
}

/// A builder for [`UpdateReputationEntityPolicyInput`](crate::operation::update_reputation_entity_policy::UpdateReputationEntityPolicyInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateReputationEntityPolicyInputBuilder {
    pub(crate) reputation_entity_type: ::std::option::Option<crate::types::ReputationEntityType>,
    pub(crate) reputation_entity_reference: ::std::option::Option<::std::string::String>,
    pub(crate) reputation_entity_policy: ::std::option::Option<::std::string::String>,
}
impl UpdateReputationEntityPolicyInputBuilder {
    /// <p>The type of reputation entity. Currently, only <code>RESOURCE</code> type entities are supported.</p>
    /// This field is required.
    pub fn reputation_entity_type(mut self, input: crate::types::ReputationEntityType) -> Self {
        self.reputation_entity_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of reputation entity. Currently, only <code>RESOURCE</code> type entities are supported.</p>
    pub fn set_reputation_entity_type(mut self, input: ::std::option::Option<crate::types::ReputationEntityType>) -> Self {
        self.reputation_entity_type = input;
        self
    }
    /// <p>The type of reputation entity. Currently, only <code>RESOURCE</code> type entities are supported.</p>
    pub fn get_reputation_entity_type(&self) -> &::std::option::Option<crate::types::ReputationEntityType> {
        &self.reputation_entity_type
    }
    /// <p>The unique identifier for the reputation entity. For resource-type entities, this is the Amazon Resource Name (ARN) of the resource.</p>
    /// This field is required.
    pub fn reputation_entity_reference(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reputation_entity_reference = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the reputation entity. For resource-type entities, this is the Amazon Resource Name (ARN) of the resource.</p>
    pub fn set_reputation_entity_reference(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reputation_entity_reference = input;
        self
    }
    /// <p>The unique identifier for the reputation entity. For resource-type entities, this is the Amazon Resource Name (ARN) of the resource.</p>
    pub fn get_reputation_entity_reference(&self) -> &::std::option::Option<::std::string::String> {
        &self.reputation_entity_reference
    }
    /// <p>The Amazon Resource Name (ARN) of the reputation management policy to apply to this entity. This is an Amazon Web Services Amazon SES-managed policy.</p>
    /// This field is required.
    pub fn reputation_entity_policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reputation_entity_policy = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the reputation management policy to apply to this entity. This is an Amazon Web Services Amazon SES-managed policy.</p>
    pub fn set_reputation_entity_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reputation_entity_policy = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the reputation management policy to apply to this entity. This is an Amazon Web Services Amazon SES-managed policy.</p>
    pub fn get_reputation_entity_policy(&self) -> &::std::option::Option<::std::string::String> {
        &self.reputation_entity_policy
    }
    /// Consumes the builder and constructs a [`UpdateReputationEntityPolicyInput`](crate::operation::update_reputation_entity_policy::UpdateReputationEntityPolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_reputation_entity_policy::UpdateReputationEntityPolicyInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_reputation_entity_policy::UpdateReputationEntityPolicyInput {
            reputation_entity_type: self.reputation_entity_type,
            reputation_entity_reference: self.reputation_entity_reference,
            reputation_entity_policy: self.reputation_entity_policy,
        })
    }
}
