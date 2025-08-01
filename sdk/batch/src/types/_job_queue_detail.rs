// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the details for an Batch job queue.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JobQueueDetail {
    /// <p>The job queue name.</p>
    pub job_queue_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    pub job_queue_arn: ::std::option::Option<::std::string::String>,
    /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    pub state: ::std::option::Option<crate::types::JqState>,
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub scheduling_policy_arn: ::std::option::Option<::std::string::String>,
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    pub status: ::std::option::Option<crate::types::JqStatus>,
    /// <p>A short, human-readable string to provide additional details for the current status of the job queue.</p>
    pub status_reason: ::std::option::Option<::std::string::String>,
    /// <p>The priority of the job queue. Job queue priority determines the order that job queues are evaluated when multiple queues dispatch jobs within a shared compute environment. A higher value for <code>priority</code> indicates a higher priority. Queues are evaluated in cycles, in descending order by priority. For example, a job queue with a priority value of <code>10</code> is evaluated before a queue with a priority value of <code>1</code>. All of the compute environments must be either Amazon EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). Amazon EC2 and Fargate compute environments can't be mixed.</p><note>
    /// <p>Job queue priority doesn't guarantee that a particular job executes before a job in a lower priority queue. Jobs added to higher priority queues during the queue evaluation cycle might not be evaluated until the next cycle. A job is dispatched from a queue only if resources are available when the queue is evaluated. If there are insufficient resources available at that time, the cycle proceeds to the next queue. This means that jobs added to higher priority queues might have to wait for jobs in multiple lower priority queues to complete before they are dispatched. You can use job dependencies to control the order for jobs from queues with different priorities. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_dependencies.html">Job Dependencies</a> in the <i>Batch User Guide</i>.</p>
    /// </note>
    pub priority: ::std::option::Option<i32>,
    /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    pub compute_environment_order: ::std::option::Option<::std::vec::Vec<crate::types::ComputeEnvironmentOrder>>,
    /// <p>The order of the service environment associated with the job queue. Job queues with a higher priority are evaluated first when associated with the same service environment.</p>
    pub service_environment_order: ::std::option::Option<::std::vec::Vec<crate::types::ServiceEnvironmentOrder>>,
    /// <p>The type of job queue. For service jobs that run on SageMaker Training, this value is <code>SAGEMAKER_TRAINING</code>. For regular container jobs, this value is <code>EKS</code>, <code>ECS</code>, or <code>ECS_FARGATE</code> depending on the compute environment.</p>
    pub job_queue_type: ::std::option::Option<crate::types::JobQueueType>,
    /// <p>The tags that are applied to the job queue. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>The set of actions that Batch perform on jobs that remain at the head of the job queue in the specified state longer than specified times. Batch will perform each action after <code>maxTimeSeconds</code> has passed.</p>
    pub job_state_time_limit_actions: ::std::option::Option<::std::vec::Vec<crate::types::JobStateTimeLimitAction>>,
}
impl JobQueueDetail {
    /// <p>The job queue name.</p>
    pub fn job_queue_name(&self) -> ::std::option::Option<&str> {
        self.job_queue_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    pub fn job_queue_arn(&self) -> ::std::option::Option<&str> {
        self.job_queue_arn.as_deref()
    }
    /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::JqState> {
        self.state.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub fn scheduling_policy_arn(&self) -> ::std::option::Option<&str> {
        self.scheduling_policy_arn.as_deref()
    }
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::JqStatus> {
        self.status.as_ref()
    }
    /// <p>A short, human-readable string to provide additional details for the current status of the job queue.</p>
    pub fn status_reason(&self) -> ::std::option::Option<&str> {
        self.status_reason.as_deref()
    }
    /// <p>The priority of the job queue. Job queue priority determines the order that job queues are evaluated when multiple queues dispatch jobs within a shared compute environment. A higher value for <code>priority</code> indicates a higher priority. Queues are evaluated in cycles, in descending order by priority. For example, a job queue with a priority value of <code>10</code> is evaluated before a queue with a priority value of <code>1</code>. All of the compute environments must be either Amazon EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). Amazon EC2 and Fargate compute environments can't be mixed.</p><note>
    /// <p>Job queue priority doesn't guarantee that a particular job executes before a job in a lower priority queue. Jobs added to higher priority queues during the queue evaluation cycle might not be evaluated until the next cycle. A job is dispatched from a queue only if resources are available when the queue is evaluated. If there are insufficient resources available at that time, the cycle proceeds to the next queue. This means that jobs added to higher priority queues might have to wait for jobs in multiple lower priority queues to complete before they are dispatched. You can use job dependencies to control the order for jobs from queues with different priorities. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_dependencies.html">Job Dependencies</a> in the <i>Batch User Guide</i>.</p>
    /// </note>
    pub fn priority(&self) -> ::std::option::Option<i32> {
        self.priority
    }
    /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.compute_environment_order.is_none()`.
    pub fn compute_environment_order(&self) -> &[crate::types::ComputeEnvironmentOrder] {
        self.compute_environment_order.as_deref().unwrap_or_default()
    }
    /// <p>The order of the service environment associated with the job queue. Job queues with a higher priority are evaluated first when associated with the same service environment.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.service_environment_order.is_none()`.
    pub fn service_environment_order(&self) -> &[crate::types::ServiceEnvironmentOrder] {
        self.service_environment_order.as_deref().unwrap_or_default()
    }
    /// <p>The type of job queue. For service jobs that run on SageMaker Training, this value is <code>SAGEMAKER_TRAINING</code>. For regular container jobs, this value is <code>EKS</code>, <code>ECS</code>, or <code>ECS_FARGATE</code> depending on the compute environment.</p>
    pub fn job_queue_type(&self) -> ::std::option::Option<&crate::types::JobQueueType> {
        self.job_queue_type.as_ref()
    }
    /// <p>The tags that are applied to the job queue. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
    /// <p>The set of actions that Batch perform on jobs that remain at the head of the job queue in the specified state longer than specified times. Batch will perform each action after <code>maxTimeSeconds</code> has passed.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.job_state_time_limit_actions.is_none()`.
    pub fn job_state_time_limit_actions(&self) -> &[crate::types::JobStateTimeLimitAction] {
        self.job_state_time_limit_actions.as_deref().unwrap_or_default()
    }
}
impl JobQueueDetail {
    /// Creates a new builder-style object to manufacture [`JobQueueDetail`](crate::types::JobQueueDetail).
    pub fn builder() -> crate::types::builders::JobQueueDetailBuilder {
        crate::types::builders::JobQueueDetailBuilder::default()
    }
}

/// A builder for [`JobQueueDetail`](crate::types::JobQueueDetail).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct JobQueueDetailBuilder {
    pub(crate) job_queue_name: ::std::option::Option<::std::string::String>,
    pub(crate) job_queue_arn: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::JqState>,
    pub(crate) scheduling_policy_arn: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::JqStatus>,
    pub(crate) status_reason: ::std::option::Option<::std::string::String>,
    pub(crate) priority: ::std::option::Option<i32>,
    pub(crate) compute_environment_order: ::std::option::Option<::std::vec::Vec<crate::types::ComputeEnvironmentOrder>>,
    pub(crate) service_environment_order: ::std::option::Option<::std::vec::Vec<crate::types::ServiceEnvironmentOrder>>,
    pub(crate) job_queue_type: ::std::option::Option<crate::types::JobQueueType>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) job_state_time_limit_actions: ::std::option::Option<::std::vec::Vec<crate::types::JobStateTimeLimitAction>>,
}
impl JobQueueDetailBuilder {
    /// <p>The job queue name.</p>
    /// This field is required.
    pub fn job_queue_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_queue_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The job queue name.</p>
    pub fn set_job_queue_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_queue_name = input;
        self
    }
    /// <p>The job queue name.</p>
    pub fn get_job_queue_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_queue_name
    }
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    /// This field is required.
    pub fn job_queue_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_queue_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    pub fn set_job_queue_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_queue_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    pub fn get_job_queue_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_queue_arn
    }
    /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    /// This field is required.
    pub fn state(mut self, input: crate::types::JqState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::JqState>) -> Self {
        self.state = input;
        self
    }
    /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::JqState> {
        &self.state
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub fn scheduling_policy_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.scheduling_policy_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub fn set_scheduling_policy_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.scheduling_policy_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub fn get_scheduling_policy_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.scheduling_policy_arn
    }
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    pub fn status(mut self, input: crate::types::JqStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::JqStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::JqStatus> {
        &self.status
    }
    /// <p>A short, human-readable string to provide additional details for the current status of the job queue.</p>
    pub fn status_reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A short, human-readable string to provide additional details for the current status of the job queue.</p>
    pub fn set_status_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_reason = input;
        self
    }
    /// <p>A short, human-readable string to provide additional details for the current status of the job queue.</p>
    pub fn get_status_reason(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_reason
    }
    /// <p>The priority of the job queue. Job queue priority determines the order that job queues are evaluated when multiple queues dispatch jobs within a shared compute environment. A higher value for <code>priority</code> indicates a higher priority. Queues are evaluated in cycles, in descending order by priority. For example, a job queue with a priority value of <code>10</code> is evaluated before a queue with a priority value of <code>1</code>. All of the compute environments must be either Amazon EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). Amazon EC2 and Fargate compute environments can't be mixed.</p><note>
    /// <p>Job queue priority doesn't guarantee that a particular job executes before a job in a lower priority queue. Jobs added to higher priority queues during the queue evaluation cycle might not be evaluated until the next cycle. A job is dispatched from a queue only if resources are available when the queue is evaluated. If there are insufficient resources available at that time, the cycle proceeds to the next queue. This means that jobs added to higher priority queues might have to wait for jobs in multiple lower priority queues to complete before they are dispatched. You can use job dependencies to control the order for jobs from queues with different priorities. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_dependencies.html">Job Dependencies</a> in the <i>Batch User Guide</i>.</p>
    /// </note>
    /// This field is required.
    pub fn priority(mut self, input: i32) -> Self {
        self.priority = ::std::option::Option::Some(input);
        self
    }
    /// <p>The priority of the job queue. Job queue priority determines the order that job queues are evaluated when multiple queues dispatch jobs within a shared compute environment. A higher value for <code>priority</code> indicates a higher priority. Queues are evaluated in cycles, in descending order by priority. For example, a job queue with a priority value of <code>10</code> is evaluated before a queue with a priority value of <code>1</code>. All of the compute environments must be either Amazon EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). Amazon EC2 and Fargate compute environments can't be mixed.</p><note>
    /// <p>Job queue priority doesn't guarantee that a particular job executes before a job in a lower priority queue. Jobs added to higher priority queues during the queue evaluation cycle might not be evaluated until the next cycle. A job is dispatched from a queue only if resources are available when the queue is evaluated. If there are insufficient resources available at that time, the cycle proceeds to the next queue. This means that jobs added to higher priority queues might have to wait for jobs in multiple lower priority queues to complete before they are dispatched. You can use job dependencies to control the order for jobs from queues with different priorities. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_dependencies.html">Job Dependencies</a> in the <i>Batch User Guide</i>.</p>
    /// </note>
    pub fn set_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.priority = input;
        self
    }
    /// <p>The priority of the job queue. Job queue priority determines the order that job queues are evaluated when multiple queues dispatch jobs within a shared compute environment. A higher value for <code>priority</code> indicates a higher priority. Queues are evaluated in cycles, in descending order by priority. For example, a job queue with a priority value of <code>10</code> is evaluated before a queue with a priority value of <code>1</code>. All of the compute environments must be either Amazon EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). Amazon EC2 and Fargate compute environments can't be mixed.</p><note>
    /// <p>Job queue priority doesn't guarantee that a particular job executes before a job in a lower priority queue. Jobs added to higher priority queues during the queue evaluation cycle might not be evaluated until the next cycle. A job is dispatched from a queue only if resources are available when the queue is evaluated. If there are insufficient resources available at that time, the cycle proceeds to the next queue. This means that jobs added to higher priority queues might have to wait for jobs in multiple lower priority queues to complete before they are dispatched. You can use job dependencies to control the order for jobs from queues with different priorities. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_dependencies.html">Job Dependencies</a> in the <i>Batch User Guide</i>.</p>
    /// </note>
    pub fn get_priority(&self) -> &::std::option::Option<i32> {
        &self.priority
    }
    /// Appends an item to `compute_environment_order`.
    ///
    /// To override the contents of this collection use [`set_compute_environment_order`](Self::set_compute_environment_order).
    ///
    /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    pub fn compute_environment_order(mut self, input: crate::types::ComputeEnvironmentOrder) -> Self {
        let mut v = self.compute_environment_order.unwrap_or_default();
        v.push(input);
        self.compute_environment_order = ::std::option::Option::Some(v);
        self
    }
    /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    pub fn set_compute_environment_order(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ComputeEnvironmentOrder>>) -> Self {
        self.compute_environment_order = input;
        self
    }
    /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    pub fn get_compute_environment_order(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ComputeEnvironmentOrder>> {
        &self.compute_environment_order
    }
    /// Appends an item to `service_environment_order`.
    ///
    /// To override the contents of this collection use [`set_service_environment_order`](Self::set_service_environment_order).
    ///
    /// <p>The order of the service environment associated with the job queue. Job queues with a higher priority are evaluated first when associated with the same service environment.</p>
    pub fn service_environment_order(mut self, input: crate::types::ServiceEnvironmentOrder) -> Self {
        let mut v = self.service_environment_order.unwrap_or_default();
        v.push(input);
        self.service_environment_order = ::std::option::Option::Some(v);
        self
    }
    /// <p>The order of the service environment associated with the job queue. Job queues with a higher priority are evaluated first when associated with the same service environment.</p>
    pub fn set_service_environment_order(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ServiceEnvironmentOrder>>) -> Self {
        self.service_environment_order = input;
        self
    }
    /// <p>The order of the service environment associated with the job queue. Job queues with a higher priority are evaluated first when associated with the same service environment.</p>
    pub fn get_service_environment_order(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ServiceEnvironmentOrder>> {
        &self.service_environment_order
    }
    /// <p>The type of job queue. For service jobs that run on SageMaker Training, this value is <code>SAGEMAKER_TRAINING</code>. For regular container jobs, this value is <code>EKS</code>, <code>ECS</code>, or <code>ECS_FARGATE</code> depending on the compute environment.</p>
    pub fn job_queue_type(mut self, input: crate::types::JobQueueType) -> Self {
        self.job_queue_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of job queue. For service jobs that run on SageMaker Training, this value is <code>SAGEMAKER_TRAINING</code>. For regular container jobs, this value is <code>EKS</code>, <code>ECS</code>, or <code>ECS_FARGATE</code> depending on the compute environment.</p>
    pub fn set_job_queue_type(mut self, input: ::std::option::Option<crate::types::JobQueueType>) -> Self {
        self.job_queue_type = input;
        self
    }
    /// <p>The type of job queue. For service jobs that run on SageMaker Training, this value is <code>SAGEMAKER_TRAINING</code>. For regular container jobs, this value is <code>EKS</code>, <code>ECS</code>, or <code>ECS_FARGATE</code> depending on the compute environment.</p>
    pub fn get_job_queue_type(&self) -> &::std::option::Option<crate::types::JobQueueType> {
        &self.job_queue_type
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags that are applied to the job queue. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags that are applied to the job queue. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags that are applied to the job queue. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// Appends an item to `job_state_time_limit_actions`.
    ///
    /// To override the contents of this collection use [`set_job_state_time_limit_actions`](Self::set_job_state_time_limit_actions).
    ///
    /// <p>The set of actions that Batch perform on jobs that remain at the head of the job queue in the specified state longer than specified times. Batch will perform each action after <code>maxTimeSeconds</code> has passed.</p>
    pub fn job_state_time_limit_actions(mut self, input: crate::types::JobStateTimeLimitAction) -> Self {
        let mut v = self.job_state_time_limit_actions.unwrap_or_default();
        v.push(input);
        self.job_state_time_limit_actions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The set of actions that Batch perform on jobs that remain at the head of the job queue in the specified state longer than specified times. Batch will perform each action after <code>maxTimeSeconds</code> has passed.</p>
    pub fn set_job_state_time_limit_actions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::JobStateTimeLimitAction>>) -> Self {
        self.job_state_time_limit_actions = input;
        self
    }
    /// <p>The set of actions that Batch perform on jobs that remain at the head of the job queue in the specified state longer than specified times. Batch will perform each action after <code>maxTimeSeconds</code> has passed.</p>
    pub fn get_job_state_time_limit_actions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::JobStateTimeLimitAction>> {
        &self.job_state_time_limit_actions
    }
    /// Consumes the builder and constructs a [`JobQueueDetail`](crate::types::JobQueueDetail).
    pub fn build(self) -> crate::types::JobQueueDetail {
        crate::types::JobQueueDetail {
            job_queue_name: self.job_queue_name,
            job_queue_arn: self.job_queue_arn,
            state: self.state,
            scheduling_policy_arn: self.scheduling_policy_arn,
            status: self.status,
            status_reason: self.status_reason,
            priority: self.priority,
            compute_environment_order: self.compute_environment_order,
            service_environment_order: self.service_environment_order,
            job_queue_type: self.job_queue_type,
            tags: self.tags,
            job_state_time_limit_actions: self.job_state_time_limit_actions,
        }
    }
}
