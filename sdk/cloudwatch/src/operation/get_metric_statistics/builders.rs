// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_metric_statistics::_get_metric_statistics_output::GetMetricStatisticsOutputBuilder;

pub use crate::operation::get_metric_statistics::_get_metric_statistics_input::GetMetricStatisticsInputBuilder;

impl crate::operation::get_metric_statistics::builders::GetMetricStatisticsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_metric_statistics::GetMetricStatisticsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_metric_statistics::GetMetricStatisticsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_metric_statistics();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetMetricStatistics`.
///
/// <p>Gets statistics for the specified metric.</p>
/// <p>The maximum number of data points returned from a single call is 1,440. If you request more than 1,440 data points, CloudWatch returns an error. To reduce the number of data points, you can narrow the specified time range and make multiple requests across adjacent time ranges, or you can increase the specified period. Data points are not returned in chronological order.</p>
/// <p>CloudWatch aggregates data points based on the length of the period that you specify. For example, if you request statistics with a one-hour period, CloudWatch aggregates all data points with time stamps that fall within each one-hour period. Therefore, the number of values aggregated by CloudWatch is larger than the number of data points returned.</p>
/// <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you can only retrieve percentile statistics for this data if one of the following conditions is true:</p>
/// <ul>
/// <li>
/// <p>The SampleCount value of the statistic set is 1.</p></li>
/// <li>
/// <p>The Min and the Max values of the statistic set are equal.</p></li>
/// </ul>
/// <p>Percentile statistics are not available for metrics when any of the metric values are negative numbers.</p>
/// <p>Amazon CloudWatch retains metric data as follows:</p>
/// <ul>
/// <li>
/// <p>Data points with a period of less than 60 seconds are available for 3 hours. These data points are high-resolution metrics and are available only for custom metrics that have been defined with a <code>StorageResolution</code> of 1.</p></li>
/// <li>
/// <p>Data points with a period of 60 seconds (1-minute) are available for 15 days.</p></li>
/// <li>
/// <p>Data points with a period of 300 seconds (5-minute) are available for 63 days.</p></li>
/// <li>
/// <p>Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months).</p></li>
/// </ul>
/// <p>Data points that are initially published with a shorter period are aggregated together for long-term storage. For example, if you collect data using a period of 1 minute, the data remains available for 15 days with 1-minute resolution. After 15 days, this data is still available, but is aggregated and retrievable only with a resolution of 5 minutes. After 63 days, the data is further aggregated and is available with a resolution of 1 hour.</p>
/// <p>CloudWatch started retaining 5-minute and 1-hour metric data as of July 9, 2016.</p>
/// <p>For information about metrics and dimensions supported by Amazon Web Services services, see the <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CW_Support_For_AWS.html">Amazon CloudWatch Metrics and Dimensions Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetMetricStatisticsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_metric_statistics::builders::GetMetricStatisticsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_metric_statistics::GetMetricStatisticsOutput,
        crate::operation::get_metric_statistics::GetMetricStatisticsError,
    > for GetMetricStatisticsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_metric_statistics::GetMetricStatisticsOutput,
            crate::operation::get_metric_statistics::GetMetricStatisticsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetMetricStatisticsFluentBuilder {
    /// Creates a new `GetMetricStatisticsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetMetricStatistics as a reference.
    pub fn as_input(&self) -> &crate::operation::get_metric_statistics::builders::GetMetricStatisticsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_metric_statistics::GetMetricStatisticsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_metric_statistics::GetMetricStatisticsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_metric_statistics::GetMetricStatistics::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_metric_statistics::GetMetricStatistics::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_metric_statistics::GetMetricStatisticsOutput,
        crate::operation::get_metric_statistics::GetMetricStatisticsError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The namespace of the metric, with or without spaces.</p>
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace(input.into());
        self
    }
    /// <p>The namespace of the metric, with or without spaces.</p>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_namespace(input);
        self
    }
    /// <p>The namespace of the metric, with or without spaces.</p>
    pub fn get_namespace(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_namespace()
    }
    /// <p>The name of the metric, with or without spaces.</p>
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metric_name(input.into());
        self
    }
    /// <p>The name of the metric, with or without spaces.</p>
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metric_name(input);
        self
    }
    /// <p>The name of the metric, with or without spaces.</p>
    pub fn get_metric_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_metric_name()
    }
    ///
    /// Appends an item to `Dimensions`.
    ///
    /// To override the contents of this collection use [`set_dimensions`](Self::set_dimensions).
    ///
    /// <p>The dimensions. If the metric contains multiple dimensions, you must include a value for each dimension. CloudWatch treats each unique combination of dimensions as a separate metric. If a specific combination of dimensions was not published, you can't retrieve statistics for it. You must specify the same dimensions that were used when the metrics were created. For an example, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#dimension-combinations">Dimension Combinations</a> in the <i>Amazon CloudWatch User Guide</i>. For more information about specifying dimensions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    pub fn dimensions(mut self, input: crate::types::Dimension) -> Self {
        self.inner = self.inner.dimensions(input);
        self
    }
    /// <p>The dimensions. If the metric contains multiple dimensions, you must include a value for each dimension. CloudWatch treats each unique combination of dimensions as a separate metric. If a specific combination of dimensions was not published, you can't retrieve statistics for it. You must specify the same dimensions that were used when the metrics were created. For an example, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#dimension-combinations">Dimension Combinations</a> in the <i>Amazon CloudWatch User Guide</i>. For more information about specifying dimensions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    pub fn set_dimensions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Dimension>>) -> Self {
        self.inner = self.inner.set_dimensions(input);
        self
    }
    /// <p>The dimensions. If the metric contains multiple dimensions, you must include a value for each dimension. CloudWatch treats each unique combination of dimensions as a separate metric. If a specific combination of dimensions was not published, you can't retrieve statistics for it. You must specify the same dimensions that were used when the metrics were created. For an example, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#dimension-combinations">Dimension Combinations</a> in the <i>Amazon CloudWatch User Guide</i>. For more information about specifying dimensions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    pub fn get_dimensions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Dimension>> {
        self.inner.get_dimensions()
    }
    /// <p>The time stamp that determines the first data point to return. Start times are evaluated relative to the time that CloudWatch receives the request.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp. In a raw HTTP query, the time stamp must be in ISO 8601 UTC format (for example, 2016-10-03T23:00:00Z).</p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p></li>
    /// <li>
    /// <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p></li>
    /// <li>
    /// <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p></li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, 20, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, 20-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The time stamp that determines the first data point to return. Start times are evaluated relative to the time that CloudWatch receives the request.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp. In a raw HTTP query, the time stamp must be in ISO 8601 UTC format (for example, 2016-10-03T23:00:00Z).</p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p></li>
    /// <li>
    /// <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p></li>
    /// <li>
    /// <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p></li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, 20, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, 20-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The time stamp that determines the first data point to return. Start times are evaluated relative to the time that CloudWatch receives the request.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp. In a raw HTTP query, the time stamp must be in ISO 8601 UTC format (for example, 2016-10-03T23:00:00Z).</p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p></li>
    /// <li>
    /// <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p></li>
    /// <li>
    /// <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p></li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, 20, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, 20-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The time stamp that determines the last data point to return.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp. In a raw HTTP query, the time stamp must be in ISO 8601 UTC format (for example, 2016-10-10T23:00:00Z).</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The time stamp that determines the last data point to return.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp. In a raw HTTP query, the time stamp must be in ISO 8601 UTC format (for example, 2016-10-10T23:00:00Z).</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The time stamp that determines the last data point to return.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp. In a raw HTTP query, the time stamp must be in ISO 8601 UTC format (for example, 2016-10-10T23:00:00Z).</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
    /// <p>The granularity, in seconds, of the returned data points. For metrics with regular resolution, a period can be as short as one minute (60 seconds) and must be a multiple of 60. For high-resolution metrics that are collected at intervals of less than one minute, the period can be 1, 5, 10, 20, 30, 60, or any multiple of 60. High-resolution metrics are those metrics stored by a <code>PutMetricData</code> call that includes a <code>StorageResolution</code> of 1 second.</p>
    /// <p>If the <code>StartTime</code> parameter specifies a time stamp that is greater than 3 hours ago, you must specify the period as follows or no data points in that time range is returned:</p>
    /// <ul>
    /// <li>
    /// <p>Start time between 3 hours and 15 days ago - Use a multiple of 60 seconds (1 minute).</p></li>
    /// <li>
    /// <p>Start time between 15 and 63 days ago - Use a multiple of 300 seconds (5 minutes).</p></li>
    /// <li>
    /// <p>Start time greater than 63 days ago - Use a multiple of 3600 seconds (1 hour).</p></li>
    /// </ul>
    pub fn period(mut self, input: i32) -> Self {
        self.inner = self.inner.period(input);
        self
    }
    /// <p>The granularity, in seconds, of the returned data points. For metrics with regular resolution, a period can be as short as one minute (60 seconds) and must be a multiple of 60. For high-resolution metrics that are collected at intervals of less than one minute, the period can be 1, 5, 10, 20, 30, 60, or any multiple of 60. High-resolution metrics are those metrics stored by a <code>PutMetricData</code> call that includes a <code>StorageResolution</code> of 1 second.</p>
    /// <p>If the <code>StartTime</code> parameter specifies a time stamp that is greater than 3 hours ago, you must specify the period as follows or no data points in that time range is returned:</p>
    /// <ul>
    /// <li>
    /// <p>Start time between 3 hours and 15 days ago - Use a multiple of 60 seconds (1 minute).</p></li>
    /// <li>
    /// <p>Start time between 15 and 63 days ago - Use a multiple of 300 seconds (5 minutes).</p></li>
    /// <li>
    /// <p>Start time greater than 63 days ago - Use a multiple of 3600 seconds (1 hour).</p></li>
    /// </ul>
    pub fn set_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_period(input);
        self
    }
    /// <p>The granularity, in seconds, of the returned data points. For metrics with regular resolution, a period can be as short as one minute (60 seconds) and must be a multiple of 60. For high-resolution metrics that are collected at intervals of less than one minute, the period can be 1, 5, 10, 20, 30, 60, or any multiple of 60. High-resolution metrics are those metrics stored by a <code>PutMetricData</code> call that includes a <code>StorageResolution</code> of 1 second.</p>
    /// <p>If the <code>StartTime</code> parameter specifies a time stamp that is greater than 3 hours ago, you must specify the period as follows or no data points in that time range is returned:</p>
    /// <ul>
    /// <li>
    /// <p>Start time between 3 hours and 15 days ago - Use a multiple of 60 seconds (1 minute).</p></li>
    /// <li>
    /// <p>Start time between 15 and 63 days ago - Use a multiple of 300 seconds (5 minutes).</p></li>
    /// <li>
    /// <p>Start time greater than 63 days ago - Use a multiple of 3600 seconds (1 hour).</p></li>
    /// </ul>
    pub fn get_period(&self) -> &::std::option::Option<i32> {
        self.inner.get_period()
    }
    ///
    /// Appends an item to `Statistics`.
    ///
    /// To override the contents of this collection use [`set_statistics`](Self::set_statistics).
    ///
    /// <p>The metric statistics, other than percentile. For percentile statistics, use <code>ExtendedStatistics</code>. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both.</p>
    pub fn statistics(mut self, input: crate::types::Statistic) -> Self {
        self.inner = self.inner.statistics(input);
        self
    }
    /// <p>The metric statistics, other than percentile. For percentile statistics, use <code>ExtendedStatistics</code>. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both.</p>
    pub fn set_statistics(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Statistic>>) -> Self {
        self.inner = self.inner.set_statistics(input);
        self
    }
    /// <p>The metric statistics, other than percentile. For percentile statistics, use <code>ExtendedStatistics</code>. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both.</p>
    pub fn get_statistics(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Statistic>> {
        self.inner.get_statistics()
    }
    ///
    /// Appends an item to `ExtendedStatistics`.
    ///
    /// To override the contents of this collection use [`set_extended_statistics`](Self::set_extended_statistics).
    ///
    /// <p>The percentile statistics. Specify values between p0.0 and p100. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both. Percentile statistics are not available for metrics when any of the metric values are negative numbers.</p>
    pub fn extended_statistics(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.extended_statistics(input.into());
        self
    }
    /// <p>The percentile statistics. Specify values between p0.0 and p100. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both. Percentile statistics are not available for metrics when any of the metric values are negative numbers.</p>
    pub fn set_extended_statistics(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_extended_statistics(input);
        self
    }
    /// <p>The percentile statistics. Specify values between p0.0 and p100. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both. Percentile statistics are not available for metrics when any of the metric values are negative numbers.</p>
    pub fn get_extended_statistics(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_extended_statistics()
    }
    /// <p>The unit for a given metric. If you omit <code>Unit</code>, all data that was collected with any unit is returned, along with the corresponding units that were specified when the data was reported to CloudWatch. If you specify a unit, the operation returns only data that was collected with that unit specified. If you specify a unit that does not match the data collected, the results of the operation are null. CloudWatch does not perform unit conversions.</p>
    pub fn unit(mut self, input: crate::types::StandardUnit) -> Self {
        self.inner = self.inner.unit(input);
        self
    }
    /// <p>The unit for a given metric. If you omit <code>Unit</code>, all data that was collected with any unit is returned, along with the corresponding units that were specified when the data was reported to CloudWatch. If you specify a unit, the operation returns only data that was collected with that unit specified. If you specify a unit that does not match the data collected, the results of the operation are null. CloudWatch does not perform unit conversions.</p>
    pub fn set_unit(mut self, input: ::std::option::Option<crate::types::StandardUnit>) -> Self {
        self.inner = self.inner.set_unit(input);
        self
    }
    /// <p>The unit for a given metric. If you omit <code>Unit</code>, all data that was collected with any unit is returned, along with the corresponding units that were specified when the data was reported to CloudWatch. If you specify a unit, the operation returns only data that was collected with that unit specified. If you specify a unit that does not match the data collected, the results of the operation are null. CloudWatch does not perform unit conversions.</p>
    pub fn get_unit(&self) -> &::std::option::Option<crate::types::StandardUnit> {
        self.inner.get_unit()
    }
}
