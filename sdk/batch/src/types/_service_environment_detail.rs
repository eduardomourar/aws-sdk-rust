// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Detailed information about a service environment, including its configuration, state, and capacity limits.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ServiceEnvironmentDetail {
    /// <p>The name of the service environment.</p>
    pub service_environment_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the service environment.</p>
    pub service_environment_arn: ::std::option::Option<::std::string::String>,
    /// <p>The type of service environment. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
    pub service_environment_type: ::std::option::Option<crate::types::ServiceEnvironmentType>,
    /// <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>.</p>
    pub state: ::std::option::Option<crate::types::ServiceEnvironmentState>,
    /// <p>The current status of the service environment.</p>
    pub status: ::std::option::Option<crate::types::ServiceEnvironmentStatus>,
    /// <p>The capacity limits for the service environment. This defines the maximum resources that can be used by service jobs in this environment.</p>
    pub capacity_limits: ::std::option::Option<::std::vec::Vec<crate::types::CapacityLimit>>,
    /// <p>The tags associated with the service environment. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl ServiceEnvironmentDetail {
    /// <p>The name of the service environment.</p>
    pub fn service_environment_name(&self) -> ::std::option::Option<&str> {
        self.service_environment_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the service environment.</p>
    pub fn service_environment_arn(&self) -> ::std::option::Option<&str> {
        self.service_environment_arn.as_deref()
    }
    /// <p>The type of service environment. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn service_environment_type(&self) -> ::std::option::Option<&crate::types::ServiceEnvironmentType> {
        self.service_environment_type.as_ref()
    }
    /// <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::ServiceEnvironmentState> {
        self.state.as_ref()
    }
    /// <p>The current status of the service environment.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ServiceEnvironmentStatus> {
        self.status.as_ref()
    }
    /// <p>The capacity limits for the service environment. This defines the maximum resources that can be used by service jobs in this environment.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.capacity_limits.is_none()`.
    pub fn capacity_limits(&self) -> &[crate::types::CapacityLimit] {
        self.capacity_limits.as_deref().unwrap_or_default()
    }
    /// <p>The tags associated with the service environment. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
}
impl ServiceEnvironmentDetail {
    /// Creates a new builder-style object to manufacture [`ServiceEnvironmentDetail`](crate::types::ServiceEnvironmentDetail).
    pub fn builder() -> crate::types::builders::ServiceEnvironmentDetailBuilder {
        crate::types::builders::ServiceEnvironmentDetailBuilder::default()
    }
}

/// A builder for [`ServiceEnvironmentDetail`](crate::types::ServiceEnvironmentDetail).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ServiceEnvironmentDetailBuilder {
    pub(crate) service_environment_name: ::std::option::Option<::std::string::String>,
    pub(crate) service_environment_arn: ::std::option::Option<::std::string::String>,
    pub(crate) service_environment_type: ::std::option::Option<crate::types::ServiceEnvironmentType>,
    pub(crate) state: ::std::option::Option<crate::types::ServiceEnvironmentState>,
    pub(crate) status: ::std::option::Option<crate::types::ServiceEnvironmentStatus>,
    pub(crate) capacity_limits: ::std::option::Option<::std::vec::Vec<crate::types::CapacityLimit>>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl ServiceEnvironmentDetailBuilder {
    /// <p>The name of the service environment.</p>
    /// This field is required.
    pub fn service_environment_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_environment_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service environment.</p>
    pub fn set_service_environment_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_environment_name = input;
        self
    }
    /// <p>The name of the service environment.</p>
    pub fn get_service_environment_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.service_environment_name
    }
    /// <p>The Amazon Resource Name (ARN) of the service environment.</p>
    /// This field is required.
    pub fn service_environment_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_environment_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service environment.</p>
    pub fn set_service_environment_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_environment_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service environment.</p>
    pub fn get_service_environment_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.service_environment_arn
    }
    /// <p>The type of service environment. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
    /// This field is required.
    pub fn service_environment_type(mut self, input: crate::types::ServiceEnvironmentType) -> Self {
        self.service_environment_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of service environment. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn set_service_environment_type(mut self, input: ::std::option::Option<crate::types::ServiceEnvironmentType>) -> Self {
        self.service_environment_type = input;
        self
    }
    /// <p>The type of service environment. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn get_service_environment_type(&self) -> &::std::option::Option<crate::types::ServiceEnvironmentType> {
        &self.service_environment_type
    }
    /// <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>.</p>
    pub fn state(mut self, input: crate::types::ServiceEnvironmentState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::ServiceEnvironmentState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::ServiceEnvironmentState> {
        &self.state
    }
    /// <p>The current status of the service environment.</p>
    pub fn status(mut self, input: crate::types::ServiceEnvironmentStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the service environment.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ServiceEnvironmentStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The current status of the service environment.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ServiceEnvironmentStatus> {
        &self.status
    }
    /// Appends an item to `capacity_limits`.
    ///
    /// To override the contents of this collection use [`set_capacity_limits`](Self::set_capacity_limits).
    ///
    /// <p>The capacity limits for the service environment. This defines the maximum resources that can be used by service jobs in this environment.</p>
    pub fn capacity_limits(mut self, input: crate::types::CapacityLimit) -> Self {
        let mut v = self.capacity_limits.unwrap_or_default();
        v.push(input);
        self.capacity_limits = ::std::option::Option::Some(v);
        self
    }
    /// <p>The capacity limits for the service environment. This defines the maximum resources that can be used by service jobs in this environment.</p>
    pub fn set_capacity_limits(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CapacityLimit>>) -> Self {
        self.capacity_limits = input;
        self
    }
    /// <p>The capacity limits for the service environment. This defines the maximum resources that can be used by service jobs in this environment.</p>
    pub fn get_capacity_limits(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CapacityLimit>> {
        &self.capacity_limits
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags associated with the service environment. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags associated with the service environment. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags associated with the service environment. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`ServiceEnvironmentDetail`](crate::types::ServiceEnvironmentDetail).
    pub fn build(self) -> crate::types::ServiceEnvironmentDetail {
        crate::types::ServiceEnvironmentDetail {
            service_environment_name: self.service_environment_name,
            service_environment_arn: self.service_environment_arn,
            service_environment_type: self.service_environment_type,
            state: self.state,
            status: self.status,
            capacity_limits: self.capacity_limits,
            tags: self.tags,
        }
    }
}
