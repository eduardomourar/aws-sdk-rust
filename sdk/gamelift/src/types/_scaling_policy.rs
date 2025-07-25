// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Rule that controls how a fleet is scaled. Scaling policies are uniquely identified by the combination of name and fleet ID.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScalingPolicy {
    /// <p>A unique identifier for the fleet that is associated with this scaling policy.</p>
    pub fleet_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a Amazon GameLift Servers fleet resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:<region>
    /// ::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912
    /// </region></code>.</p>
    pub fleet_arn: ::std::option::Option<::std::string::String>,
    /// <p>A descriptive label that is associated with a fleet's scaling policy. Policy names do not need to be unique.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Current status of the scaling policy. The scaling policy can be in force only when in an <code>ACTIVE</code> status. Scaling policies can be suspended for individual fleets. If the policy is suspended for a fleet, the policy status does not change.</p>
    /// <ul>
    /// <li>
    /// <p><b>ACTIVE</b> -- The scaling policy can be used for auto-scaling a fleet.</p></li>
    /// <li>
    /// <p><b>UPDATE_REQUESTED</b> -- A request to update the scaling policy has been received.</p></li>
    /// <li>
    /// <p><b>UPDATING</b> -- A change is being made to the scaling policy.</p></li>
    /// <li>
    /// <p><b>DELETE_REQUESTED</b> -- A request to delete the scaling policy has been received.</p></li>
    /// <li>
    /// <p><b>DELETING</b> -- The scaling policy is being deleted.</p></li>
    /// <li>
    /// <p><b>DELETED</b> -- The scaling policy has been deleted.</p></li>
    /// <li>
    /// <p><b>ERROR</b> -- An error occurred in creating the policy. It should be removed and recreated.</p></li>
    /// </ul>
    pub status: ::std::option::Option<crate::types::ScalingStatusType>,
    /// <p>Amount of adjustment to make, based on the scaling adjustment type.</p>
    pub scaling_adjustment: ::std::option::Option<i32>,
    /// <p>The type of adjustment to make to a fleet's instance count.</p>
    /// <ul>
    /// <li>
    /// <p><b>ChangeInCapacity</b> -- add (or subtract) the scaling adjustment value from the current instance count. Positive values scale up while negative values scale down.</p></li>
    /// <li>
    /// <p><b>ExactCapacity</b> -- set the instance count to the scaling adjustment value.</p></li>
    /// <li>
    /// <p><b>PercentChangeInCapacity</b> -- increase or reduce the current instance count by the scaling adjustment, read as a percentage. Positive values scale up while negative values scale down.</p></li>
    /// </ul>
    pub scaling_adjustment_type: ::std::option::Option<crate::types::ScalingAdjustmentType>,
    /// <p>Comparison operator to use when measuring a metric against the threshold value.</p>
    pub comparison_operator: ::std::option::Option<crate::types::ComparisonOperatorType>,
    /// <p>Metric value used to trigger a scaling event.</p>
    pub threshold: ::std::option::Option<f64>,
    /// <p>Length of time (in minutes) the metric must be at or beyond the threshold before a scaling event is triggered.</p>
    pub evaluation_periods: ::std::option::Option<i32>,
    /// <p>Name of the Amazon GameLift Servers-defined metric that is used to trigger a scaling adjustment. For detailed descriptions of fleet metrics, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html">Monitor Amazon GameLift Servers with Amazon CloudWatch</a>.</p>
    /// <ul>
    /// <li>
    /// <p><b>ActivatingGameSessions</b> -- Game sessions in the process of being created.</p></li>
    /// <li>
    /// <p><b>ActiveGameSessions</b> -- Game sessions that are currently running.</p></li>
    /// <li>
    /// <p><b>ActiveInstances</b> -- Fleet instances that are currently running at least one game session.</p></li>
    /// <li>
    /// <p><b>AvailableGameSessions</b> -- Additional game sessions that fleet could host simultaneously, given current capacity.</p></li>
    /// <li>
    /// <p><b>AvailablePlayerSessions</b> -- Empty player slots in currently active game sessions. This includes game sessions that are not currently accepting players. Reserved player slots are not included.</p></li>
    /// <li>
    /// <p><b>CurrentPlayerSessions</b> -- Player slots in active game sessions that are being used by a player or are reserved for a player.</p></li>
    /// <li>
    /// <p><b>IdleInstances</b> -- Active instances that are currently hosting zero game sessions.</p></li>
    /// <li>
    /// <p><b>PercentAvailableGameSessions</b> -- Unused percentage of the total number of game sessions that a fleet could host simultaneously, given current capacity. Use this metric for a target-based scaling policy.</p></li>
    /// <li>
    /// <p><b>PercentIdleInstances</b> -- Percentage of the total number of active instances that are hosting zero game sessions.</p></li>
    /// <li>
    /// <p><b>QueueDepth</b> -- Pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p></li>
    /// <li>
    /// <p><b>WaitTime</b> -- Current wait time for pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p></li>
    /// </ul>
    pub metric_name: ::std::option::Option<crate::types::MetricName>,
    /// <p>The type of scaling policy to create. For a target-based policy, set the parameter <i>MetricName</i> to 'PercentAvailableGameSessions' and specify a <i>TargetConfiguration</i>. For a rule-based policy set the following parameters: <i>MetricName</i>, <i>ComparisonOperator</i>, <i>Threshold</i>, <i>EvaluationPeriods</i>, <i>ScalingAdjustmentType</i>, and <i>ScalingAdjustment</i>.</p>
    pub policy_type: ::std::option::Option<crate::types::PolicyType>,
    /// <p>An object that contains settings for a target-based scaling policy.</p>
    pub target_configuration: ::std::option::Option<crate::types::TargetConfiguration>,
    /// <p>The current status of the fleet's scaling policies in a requested fleet location. The status <code>PENDING_UPDATE</code> indicates that an update was requested for the fleet but has not yet been completed for the location.</p>
    pub update_status: ::std::option::Option<crate::types::LocationUpdateStatus>,
    /// <p>The fleet location.</p>
    pub location: ::std::option::Option<::std::string::String>,
}
impl ScalingPolicy {
    /// <p>A unique identifier for the fleet that is associated with this scaling policy.</p>
    pub fn fleet_id(&self) -> ::std::option::Option<&str> {
        self.fleet_id.as_deref()
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a Amazon GameLift Servers fleet resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:<region>
    /// ::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912
    /// </region></code>.</p>
    pub fn fleet_arn(&self) -> ::std::option::Option<&str> {
        self.fleet_arn.as_deref()
    }
    /// <p>A descriptive label that is associated with a fleet's scaling policy. Policy names do not need to be unique.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Current status of the scaling policy. The scaling policy can be in force only when in an <code>ACTIVE</code> status. Scaling policies can be suspended for individual fleets. If the policy is suspended for a fleet, the policy status does not change.</p>
    /// <ul>
    /// <li>
    /// <p><b>ACTIVE</b> -- The scaling policy can be used for auto-scaling a fleet.</p></li>
    /// <li>
    /// <p><b>UPDATE_REQUESTED</b> -- A request to update the scaling policy has been received.</p></li>
    /// <li>
    /// <p><b>UPDATING</b> -- A change is being made to the scaling policy.</p></li>
    /// <li>
    /// <p><b>DELETE_REQUESTED</b> -- A request to delete the scaling policy has been received.</p></li>
    /// <li>
    /// <p><b>DELETING</b> -- The scaling policy is being deleted.</p></li>
    /// <li>
    /// <p><b>DELETED</b> -- The scaling policy has been deleted.</p></li>
    /// <li>
    /// <p><b>ERROR</b> -- An error occurred in creating the policy. It should be removed and recreated.</p></li>
    /// </ul>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ScalingStatusType> {
        self.status.as_ref()
    }
    /// <p>Amount of adjustment to make, based on the scaling adjustment type.</p>
    pub fn scaling_adjustment(&self) -> ::std::option::Option<i32> {
        self.scaling_adjustment
    }
    /// <p>The type of adjustment to make to a fleet's instance count.</p>
    /// <ul>
    /// <li>
    /// <p><b>ChangeInCapacity</b> -- add (or subtract) the scaling adjustment value from the current instance count. Positive values scale up while negative values scale down.</p></li>
    /// <li>
    /// <p><b>ExactCapacity</b> -- set the instance count to the scaling adjustment value.</p></li>
    /// <li>
    /// <p><b>PercentChangeInCapacity</b> -- increase or reduce the current instance count by the scaling adjustment, read as a percentage. Positive values scale up while negative values scale down.</p></li>
    /// </ul>
    pub fn scaling_adjustment_type(&self) -> ::std::option::Option<&crate::types::ScalingAdjustmentType> {
        self.scaling_adjustment_type.as_ref()
    }
    /// <p>Comparison operator to use when measuring a metric against the threshold value.</p>
    pub fn comparison_operator(&self) -> ::std::option::Option<&crate::types::ComparisonOperatorType> {
        self.comparison_operator.as_ref()
    }
    /// <p>Metric value used to trigger a scaling event.</p>
    pub fn threshold(&self) -> ::std::option::Option<f64> {
        self.threshold
    }
    /// <p>Length of time (in minutes) the metric must be at or beyond the threshold before a scaling event is triggered.</p>
    pub fn evaluation_periods(&self) -> ::std::option::Option<i32> {
        self.evaluation_periods
    }
    /// <p>Name of the Amazon GameLift Servers-defined metric that is used to trigger a scaling adjustment. For detailed descriptions of fleet metrics, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html">Monitor Amazon GameLift Servers with Amazon CloudWatch</a>.</p>
    /// <ul>
    /// <li>
    /// <p><b>ActivatingGameSessions</b> -- Game sessions in the process of being created.</p></li>
    /// <li>
    /// <p><b>ActiveGameSessions</b> -- Game sessions that are currently running.</p></li>
    /// <li>
    /// <p><b>ActiveInstances</b> -- Fleet instances that are currently running at least one game session.</p></li>
    /// <li>
    /// <p><b>AvailableGameSessions</b> -- Additional game sessions that fleet could host simultaneously, given current capacity.</p></li>
    /// <li>
    /// <p><b>AvailablePlayerSessions</b> -- Empty player slots in currently active game sessions. This includes game sessions that are not currently accepting players. Reserved player slots are not included.</p></li>
    /// <li>
    /// <p><b>CurrentPlayerSessions</b> -- Player slots in active game sessions that are being used by a player or are reserved for a player.</p></li>
    /// <li>
    /// <p><b>IdleInstances</b> -- Active instances that are currently hosting zero game sessions.</p></li>
    /// <li>
    /// <p><b>PercentAvailableGameSessions</b> -- Unused percentage of the total number of game sessions that a fleet could host simultaneously, given current capacity. Use this metric for a target-based scaling policy.</p></li>
    /// <li>
    /// <p><b>PercentIdleInstances</b> -- Percentage of the total number of active instances that are hosting zero game sessions.</p></li>
    /// <li>
    /// <p><b>QueueDepth</b> -- Pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p></li>
    /// <li>
    /// <p><b>WaitTime</b> -- Current wait time for pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p></li>
    /// </ul>
    pub fn metric_name(&self) -> ::std::option::Option<&crate::types::MetricName> {
        self.metric_name.as_ref()
    }
    /// <p>The type of scaling policy to create. For a target-based policy, set the parameter <i>MetricName</i> to 'PercentAvailableGameSessions' and specify a <i>TargetConfiguration</i>. For a rule-based policy set the following parameters: <i>MetricName</i>, <i>ComparisonOperator</i>, <i>Threshold</i>, <i>EvaluationPeriods</i>, <i>ScalingAdjustmentType</i>, and <i>ScalingAdjustment</i>.</p>
    pub fn policy_type(&self) -> ::std::option::Option<&crate::types::PolicyType> {
        self.policy_type.as_ref()
    }
    /// <p>An object that contains settings for a target-based scaling policy.</p>
    pub fn target_configuration(&self) -> ::std::option::Option<&crate::types::TargetConfiguration> {
        self.target_configuration.as_ref()
    }
    /// <p>The current status of the fleet's scaling policies in a requested fleet location. The status <code>PENDING_UPDATE</code> indicates that an update was requested for the fleet but has not yet been completed for the location.</p>
    pub fn update_status(&self) -> ::std::option::Option<&crate::types::LocationUpdateStatus> {
        self.update_status.as_ref()
    }
    /// <p>The fleet location.</p>
    pub fn location(&self) -> ::std::option::Option<&str> {
        self.location.as_deref()
    }
}
impl ScalingPolicy {
    /// Creates a new builder-style object to manufacture [`ScalingPolicy`](crate::types::ScalingPolicy).
    pub fn builder() -> crate::types::builders::ScalingPolicyBuilder {
        crate::types::builders::ScalingPolicyBuilder::default()
    }
}

/// A builder for [`ScalingPolicy`](crate::types::ScalingPolicy).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ScalingPolicyBuilder {
    pub(crate) fleet_id: ::std::option::Option<::std::string::String>,
    pub(crate) fleet_arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ScalingStatusType>,
    pub(crate) scaling_adjustment: ::std::option::Option<i32>,
    pub(crate) scaling_adjustment_type: ::std::option::Option<crate::types::ScalingAdjustmentType>,
    pub(crate) comparison_operator: ::std::option::Option<crate::types::ComparisonOperatorType>,
    pub(crate) threshold: ::std::option::Option<f64>,
    pub(crate) evaluation_periods: ::std::option::Option<i32>,
    pub(crate) metric_name: ::std::option::Option<crate::types::MetricName>,
    pub(crate) policy_type: ::std::option::Option<crate::types::PolicyType>,
    pub(crate) target_configuration: ::std::option::Option<crate::types::TargetConfiguration>,
    pub(crate) update_status: ::std::option::Option<crate::types::LocationUpdateStatus>,
    pub(crate) location: ::std::option::Option<::std::string::String>,
}
impl ScalingPolicyBuilder {
    /// <p>A unique identifier for the fleet that is associated with this scaling policy.</p>
    pub fn fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fleet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the fleet that is associated with this scaling policy.</p>
    pub fn set_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fleet_id = input;
        self
    }
    /// <p>A unique identifier for the fleet that is associated with this scaling policy.</p>
    pub fn get_fleet_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.fleet_id
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a Amazon GameLift Servers fleet resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:<region>
    /// ::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912
    /// </region></code>.</p>
    pub fn fleet_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fleet_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a Amazon GameLift Servers fleet resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:<region>
    /// ::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912
    /// </region></code>.</p>
    pub fn set_fleet_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fleet_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a Amazon GameLift Servers fleet resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:<region>
    /// ::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912
    /// </region></code>.</p>
    pub fn get_fleet_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.fleet_arn
    }
    /// <p>A descriptive label that is associated with a fleet's scaling policy. Policy names do not need to be unique.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A descriptive label that is associated with a fleet's scaling policy. Policy names do not need to be unique.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A descriptive label that is associated with a fleet's scaling policy. Policy names do not need to be unique.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>Current status of the scaling policy. The scaling policy can be in force only when in an <code>ACTIVE</code> status. Scaling policies can be suspended for individual fleets. If the policy is suspended for a fleet, the policy status does not change.</p>
    /// <ul>
    /// <li>
    /// <p><b>ACTIVE</b> -- The scaling policy can be used for auto-scaling a fleet.</p></li>
    /// <li>
    /// <p><b>UPDATE_REQUESTED</b> -- A request to update the scaling policy has been received.</p></li>
    /// <li>
    /// <p><b>UPDATING</b> -- A change is being made to the scaling policy.</p></li>
    /// <li>
    /// <p><b>DELETE_REQUESTED</b> -- A request to delete the scaling policy has been received.</p></li>
    /// <li>
    /// <p><b>DELETING</b> -- The scaling policy is being deleted.</p></li>
    /// <li>
    /// <p><b>DELETED</b> -- The scaling policy has been deleted.</p></li>
    /// <li>
    /// <p><b>ERROR</b> -- An error occurred in creating the policy. It should be removed and recreated.</p></li>
    /// </ul>
    pub fn status(mut self, input: crate::types::ScalingStatusType) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Current status of the scaling policy. The scaling policy can be in force only when in an <code>ACTIVE</code> status. Scaling policies can be suspended for individual fleets. If the policy is suspended for a fleet, the policy status does not change.</p>
    /// <ul>
    /// <li>
    /// <p><b>ACTIVE</b> -- The scaling policy can be used for auto-scaling a fleet.</p></li>
    /// <li>
    /// <p><b>UPDATE_REQUESTED</b> -- A request to update the scaling policy has been received.</p></li>
    /// <li>
    /// <p><b>UPDATING</b> -- A change is being made to the scaling policy.</p></li>
    /// <li>
    /// <p><b>DELETE_REQUESTED</b> -- A request to delete the scaling policy has been received.</p></li>
    /// <li>
    /// <p><b>DELETING</b> -- The scaling policy is being deleted.</p></li>
    /// <li>
    /// <p><b>DELETED</b> -- The scaling policy has been deleted.</p></li>
    /// <li>
    /// <p><b>ERROR</b> -- An error occurred in creating the policy. It should be removed and recreated.</p></li>
    /// </ul>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ScalingStatusType>) -> Self {
        self.status = input;
        self
    }
    /// <p>Current status of the scaling policy. The scaling policy can be in force only when in an <code>ACTIVE</code> status. Scaling policies can be suspended for individual fleets. If the policy is suspended for a fleet, the policy status does not change.</p>
    /// <ul>
    /// <li>
    /// <p><b>ACTIVE</b> -- The scaling policy can be used for auto-scaling a fleet.</p></li>
    /// <li>
    /// <p><b>UPDATE_REQUESTED</b> -- A request to update the scaling policy has been received.</p></li>
    /// <li>
    /// <p><b>UPDATING</b> -- A change is being made to the scaling policy.</p></li>
    /// <li>
    /// <p><b>DELETE_REQUESTED</b> -- A request to delete the scaling policy has been received.</p></li>
    /// <li>
    /// <p><b>DELETING</b> -- The scaling policy is being deleted.</p></li>
    /// <li>
    /// <p><b>DELETED</b> -- The scaling policy has been deleted.</p></li>
    /// <li>
    /// <p><b>ERROR</b> -- An error occurred in creating the policy. It should be removed and recreated.</p></li>
    /// </ul>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ScalingStatusType> {
        &self.status
    }
    /// <p>Amount of adjustment to make, based on the scaling adjustment type.</p>
    pub fn scaling_adjustment(mut self, input: i32) -> Self {
        self.scaling_adjustment = ::std::option::Option::Some(input);
        self
    }
    /// <p>Amount of adjustment to make, based on the scaling adjustment type.</p>
    pub fn set_scaling_adjustment(mut self, input: ::std::option::Option<i32>) -> Self {
        self.scaling_adjustment = input;
        self
    }
    /// <p>Amount of adjustment to make, based on the scaling adjustment type.</p>
    pub fn get_scaling_adjustment(&self) -> &::std::option::Option<i32> {
        &self.scaling_adjustment
    }
    /// <p>The type of adjustment to make to a fleet's instance count.</p>
    /// <ul>
    /// <li>
    /// <p><b>ChangeInCapacity</b> -- add (or subtract) the scaling adjustment value from the current instance count. Positive values scale up while negative values scale down.</p></li>
    /// <li>
    /// <p><b>ExactCapacity</b> -- set the instance count to the scaling adjustment value.</p></li>
    /// <li>
    /// <p><b>PercentChangeInCapacity</b> -- increase or reduce the current instance count by the scaling adjustment, read as a percentage. Positive values scale up while negative values scale down.</p></li>
    /// </ul>
    pub fn scaling_adjustment_type(mut self, input: crate::types::ScalingAdjustmentType) -> Self {
        self.scaling_adjustment_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of adjustment to make to a fleet's instance count.</p>
    /// <ul>
    /// <li>
    /// <p><b>ChangeInCapacity</b> -- add (or subtract) the scaling adjustment value from the current instance count. Positive values scale up while negative values scale down.</p></li>
    /// <li>
    /// <p><b>ExactCapacity</b> -- set the instance count to the scaling adjustment value.</p></li>
    /// <li>
    /// <p><b>PercentChangeInCapacity</b> -- increase or reduce the current instance count by the scaling adjustment, read as a percentage. Positive values scale up while negative values scale down.</p></li>
    /// </ul>
    pub fn set_scaling_adjustment_type(mut self, input: ::std::option::Option<crate::types::ScalingAdjustmentType>) -> Self {
        self.scaling_adjustment_type = input;
        self
    }
    /// <p>The type of adjustment to make to a fleet's instance count.</p>
    /// <ul>
    /// <li>
    /// <p><b>ChangeInCapacity</b> -- add (or subtract) the scaling adjustment value from the current instance count. Positive values scale up while negative values scale down.</p></li>
    /// <li>
    /// <p><b>ExactCapacity</b> -- set the instance count to the scaling adjustment value.</p></li>
    /// <li>
    /// <p><b>PercentChangeInCapacity</b> -- increase or reduce the current instance count by the scaling adjustment, read as a percentage. Positive values scale up while negative values scale down.</p></li>
    /// </ul>
    pub fn get_scaling_adjustment_type(&self) -> &::std::option::Option<crate::types::ScalingAdjustmentType> {
        &self.scaling_adjustment_type
    }
    /// <p>Comparison operator to use when measuring a metric against the threshold value.</p>
    pub fn comparison_operator(mut self, input: crate::types::ComparisonOperatorType) -> Self {
        self.comparison_operator = ::std::option::Option::Some(input);
        self
    }
    /// <p>Comparison operator to use when measuring a metric against the threshold value.</p>
    pub fn set_comparison_operator(mut self, input: ::std::option::Option<crate::types::ComparisonOperatorType>) -> Self {
        self.comparison_operator = input;
        self
    }
    /// <p>Comparison operator to use when measuring a metric against the threshold value.</p>
    pub fn get_comparison_operator(&self) -> &::std::option::Option<crate::types::ComparisonOperatorType> {
        &self.comparison_operator
    }
    /// <p>Metric value used to trigger a scaling event.</p>
    pub fn threshold(mut self, input: f64) -> Self {
        self.threshold = ::std::option::Option::Some(input);
        self
    }
    /// <p>Metric value used to trigger a scaling event.</p>
    pub fn set_threshold(mut self, input: ::std::option::Option<f64>) -> Self {
        self.threshold = input;
        self
    }
    /// <p>Metric value used to trigger a scaling event.</p>
    pub fn get_threshold(&self) -> &::std::option::Option<f64> {
        &self.threshold
    }
    /// <p>Length of time (in minutes) the metric must be at or beyond the threshold before a scaling event is triggered.</p>
    pub fn evaluation_periods(mut self, input: i32) -> Self {
        self.evaluation_periods = ::std::option::Option::Some(input);
        self
    }
    /// <p>Length of time (in minutes) the metric must be at or beyond the threshold before a scaling event is triggered.</p>
    pub fn set_evaluation_periods(mut self, input: ::std::option::Option<i32>) -> Self {
        self.evaluation_periods = input;
        self
    }
    /// <p>Length of time (in minutes) the metric must be at or beyond the threshold before a scaling event is triggered.</p>
    pub fn get_evaluation_periods(&self) -> &::std::option::Option<i32> {
        &self.evaluation_periods
    }
    /// <p>Name of the Amazon GameLift Servers-defined metric that is used to trigger a scaling adjustment. For detailed descriptions of fleet metrics, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html">Monitor Amazon GameLift Servers with Amazon CloudWatch</a>.</p>
    /// <ul>
    /// <li>
    /// <p><b>ActivatingGameSessions</b> -- Game sessions in the process of being created.</p></li>
    /// <li>
    /// <p><b>ActiveGameSessions</b> -- Game sessions that are currently running.</p></li>
    /// <li>
    /// <p><b>ActiveInstances</b> -- Fleet instances that are currently running at least one game session.</p></li>
    /// <li>
    /// <p><b>AvailableGameSessions</b> -- Additional game sessions that fleet could host simultaneously, given current capacity.</p></li>
    /// <li>
    /// <p><b>AvailablePlayerSessions</b> -- Empty player slots in currently active game sessions. This includes game sessions that are not currently accepting players. Reserved player slots are not included.</p></li>
    /// <li>
    /// <p><b>CurrentPlayerSessions</b> -- Player slots in active game sessions that are being used by a player or are reserved for a player.</p></li>
    /// <li>
    /// <p><b>IdleInstances</b> -- Active instances that are currently hosting zero game sessions.</p></li>
    /// <li>
    /// <p><b>PercentAvailableGameSessions</b> -- Unused percentage of the total number of game sessions that a fleet could host simultaneously, given current capacity. Use this metric for a target-based scaling policy.</p></li>
    /// <li>
    /// <p><b>PercentIdleInstances</b> -- Percentage of the total number of active instances that are hosting zero game sessions.</p></li>
    /// <li>
    /// <p><b>QueueDepth</b> -- Pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p></li>
    /// <li>
    /// <p><b>WaitTime</b> -- Current wait time for pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p></li>
    /// </ul>
    pub fn metric_name(mut self, input: crate::types::MetricName) -> Self {
        self.metric_name = ::std::option::Option::Some(input);
        self
    }
    /// <p>Name of the Amazon GameLift Servers-defined metric that is used to trigger a scaling adjustment. For detailed descriptions of fleet metrics, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html">Monitor Amazon GameLift Servers with Amazon CloudWatch</a>.</p>
    /// <ul>
    /// <li>
    /// <p><b>ActivatingGameSessions</b> -- Game sessions in the process of being created.</p></li>
    /// <li>
    /// <p><b>ActiveGameSessions</b> -- Game sessions that are currently running.</p></li>
    /// <li>
    /// <p><b>ActiveInstances</b> -- Fleet instances that are currently running at least one game session.</p></li>
    /// <li>
    /// <p><b>AvailableGameSessions</b> -- Additional game sessions that fleet could host simultaneously, given current capacity.</p></li>
    /// <li>
    /// <p><b>AvailablePlayerSessions</b> -- Empty player slots in currently active game sessions. This includes game sessions that are not currently accepting players. Reserved player slots are not included.</p></li>
    /// <li>
    /// <p><b>CurrentPlayerSessions</b> -- Player slots in active game sessions that are being used by a player or are reserved for a player.</p></li>
    /// <li>
    /// <p><b>IdleInstances</b> -- Active instances that are currently hosting zero game sessions.</p></li>
    /// <li>
    /// <p><b>PercentAvailableGameSessions</b> -- Unused percentage of the total number of game sessions that a fleet could host simultaneously, given current capacity. Use this metric for a target-based scaling policy.</p></li>
    /// <li>
    /// <p><b>PercentIdleInstances</b> -- Percentage of the total number of active instances that are hosting zero game sessions.</p></li>
    /// <li>
    /// <p><b>QueueDepth</b> -- Pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p></li>
    /// <li>
    /// <p><b>WaitTime</b> -- Current wait time for pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p></li>
    /// </ul>
    pub fn set_metric_name(mut self, input: ::std::option::Option<crate::types::MetricName>) -> Self {
        self.metric_name = input;
        self
    }
    /// <p>Name of the Amazon GameLift Servers-defined metric that is used to trigger a scaling adjustment. For detailed descriptions of fleet metrics, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html">Monitor Amazon GameLift Servers with Amazon CloudWatch</a>.</p>
    /// <ul>
    /// <li>
    /// <p><b>ActivatingGameSessions</b> -- Game sessions in the process of being created.</p></li>
    /// <li>
    /// <p><b>ActiveGameSessions</b> -- Game sessions that are currently running.</p></li>
    /// <li>
    /// <p><b>ActiveInstances</b> -- Fleet instances that are currently running at least one game session.</p></li>
    /// <li>
    /// <p><b>AvailableGameSessions</b> -- Additional game sessions that fleet could host simultaneously, given current capacity.</p></li>
    /// <li>
    /// <p><b>AvailablePlayerSessions</b> -- Empty player slots in currently active game sessions. This includes game sessions that are not currently accepting players. Reserved player slots are not included.</p></li>
    /// <li>
    /// <p><b>CurrentPlayerSessions</b> -- Player slots in active game sessions that are being used by a player or are reserved for a player.</p></li>
    /// <li>
    /// <p><b>IdleInstances</b> -- Active instances that are currently hosting zero game sessions.</p></li>
    /// <li>
    /// <p><b>PercentAvailableGameSessions</b> -- Unused percentage of the total number of game sessions that a fleet could host simultaneously, given current capacity. Use this metric for a target-based scaling policy.</p></li>
    /// <li>
    /// <p><b>PercentIdleInstances</b> -- Percentage of the total number of active instances that are hosting zero game sessions.</p></li>
    /// <li>
    /// <p><b>QueueDepth</b> -- Pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p></li>
    /// <li>
    /// <p><b>WaitTime</b> -- Current wait time for pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p></li>
    /// </ul>
    pub fn get_metric_name(&self) -> &::std::option::Option<crate::types::MetricName> {
        &self.metric_name
    }
    /// <p>The type of scaling policy to create. For a target-based policy, set the parameter <i>MetricName</i> to 'PercentAvailableGameSessions' and specify a <i>TargetConfiguration</i>. For a rule-based policy set the following parameters: <i>MetricName</i>, <i>ComparisonOperator</i>, <i>Threshold</i>, <i>EvaluationPeriods</i>, <i>ScalingAdjustmentType</i>, and <i>ScalingAdjustment</i>.</p>
    pub fn policy_type(mut self, input: crate::types::PolicyType) -> Self {
        self.policy_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of scaling policy to create. For a target-based policy, set the parameter <i>MetricName</i> to 'PercentAvailableGameSessions' and specify a <i>TargetConfiguration</i>. For a rule-based policy set the following parameters: <i>MetricName</i>, <i>ComparisonOperator</i>, <i>Threshold</i>, <i>EvaluationPeriods</i>, <i>ScalingAdjustmentType</i>, and <i>ScalingAdjustment</i>.</p>
    pub fn set_policy_type(mut self, input: ::std::option::Option<crate::types::PolicyType>) -> Self {
        self.policy_type = input;
        self
    }
    /// <p>The type of scaling policy to create. For a target-based policy, set the parameter <i>MetricName</i> to 'PercentAvailableGameSessions' and specify a <i>TargetConfiguration</i>. For a rule-based policy set the following parameters: <i>MetricName</i>, <i>ComparisonOperator</i>, <i>Threshold</i>, <i>EvaluationPeriods</i>, <i>ScalingAdjustmentType</i>, and <i>ScalingAdjustment</i>.</p>
    pub fn get_policy_type(&self) -> &::std::option::Option<crate::types::PolicyType> {
        &self.policy_type
    }
    /// <p>An object that contains settings for a target-based scaling policy.</p>
    pub fn target_configuration(mut self, input: crate::types::TargetConfiguration) -> Self {
        self.target_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>An object that contains settings for a target-based scaling policy.</p>
    pub fn set_target_configuration(mut self, input: ::std::option::Option<crate::types::TargetConfiguration>) -> Self {
        self.target_configuration = input;
        self
    }
    /// <p>An object that contains settings for a target-based scaling policy.</p>
    pub fn get_target_configuration(&self) -> &::std::option::Option<crate::types::TargetConfiguration> {
        &self.target_configuration
    }
    /// <p>The current status of the fleet's scaling policies in a requested fleet location. The status <code>PENDING_UPDATE</code> indicates that an update was requested for the fleet but has not yet been completed for the location.</p>
    pub fn update_status(mut self, input: crate::types::LocationUpdateStatus) -> Self {
        self.update_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the fleet's scaling policies in a requested fleet location. The status <code>PENDING_UPDATE</code> indicates that an update was requested for the fleet but has not yet been completed for the location.</p>
    pub fn set_update_status(mut self, input: ::std::option::Option<crate::types::LocationUpdateStatus>) -> Self {
        self.update_status = input;
        self
    }
    /// <p>The current status of the fleet's scaling policies in a requested fleet location. The status <code>PENDING_UPDATE</code> indicates that an update was requested for the fleet but has not yet been completed for the location.</p>
    pub fn get_update_status(&self) -> &::std::option::Option<crate::types::LocationUpdateStatus> {
        &self.update_status
    }
    /// <p>The fleet location.</p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.location = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The fleet location.</p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.location = input;
        self
    }
    /// <p>The fleet location.</p>
    pub fn get_location(&self) -> &::std::option::Option<::std::string::String> {
        &self.location
    }
    /// Consumes the builder and constructs a [`ScalingPolicy`](crate::types::ScalingPolicy).
    pub fn build(self) -> crate::types::ScalingPolicy {
        crate::types::ScalingPolicy {
            fleet_id: self.fleet_id,
            fleet_arn: self.fleet_arn,
            name: self.name,
            status: self.status,
            scaling_adjustment: self.scaling_adjustment,
            scaling_adjustment_type: self.scaling_adjustment_type,
            comparison_operator: self.comparison_operator,
            threshold: self.threshold,
            evaluation_periods: self.evaluation_periods,
            metric_name: self.metric_name,
            policy_type: self.policy_type,
            target_configuration: self.target_configuration,
            update_status: self.update_status,
            location: self.location,
        }
    }
}
