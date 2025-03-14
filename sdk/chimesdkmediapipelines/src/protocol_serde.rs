// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_create_media_capture_pipeline;

pub(crate) mod shape_create_media_concatenation_pipeline;

pub(crate) mod shape_create_media_insights_pipeline;

pub(crate) mod shape_create_media_insights_pipeline_configuration;

pub(crate) mod shape_create_media_live_connector_pipeline;

pub(crate) mod shape_create_media_pipeline_kinesis_video_stream_pool;

pub(crate) mod shape_create_media_stream_pipeline;

pub(crate) mod shape_delete_media_capture_pipeline;

pub(crate) mod shape_delete_media_insights_pipeline_configuration;

pub(crate) mod shape_delete_media_pipeline;

pub(crate) mod shape_delete_media_pipeline_kinesis_video_stream_pool;

pub(crate) mod shape_get_media_capture_pipeline;

pub(crate) mod shape_get_media_insights_pipeline_configuration;

pub(crate) mod shape_get_media_pipeline;

pub(crate) mod shape_get_media_pipeline_kinesis_video_stream_pool;

pub(crate) mod shape_get_speaker_search_task;

pub(crate) mod shape_get_voice_tone_analysis_task;

pub(crate) mod shape_list_media_capture_pipelines;

pub(crate) mod shape_list_media_insights_pipeline_configurations;

pub(crate) mod shape_list_media_pipeline_kinesis_video_stream_pools;

pub(crate) mod shape_list_media_pipelines;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_start_speaker_search_task;

pub(crate) mod shape_start_voice_tone_analysis_task;

pub(crate) mod shape_stop_speaker_search_task;

pub(crate) mod shape_stop_voice_tone_analysis_task;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_media_insights_pipeline_configuration;

pub(crate) mod shape_update_media_insights_pipeline_status;

pub(crate) mod shape_update_media_pipeline_kinesis_video_stream_pool;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_media_capture_pipeline_input;

pub(crate) mod shape_create_media_concatenation_pipeline_input;

pub(crate) mod shape_create_media_insights_pipeline_configuration_input;

pub(crate) mod shape_create_media_insights_pipeline_input;

pub(crate) mod shape_create_media_live_connector_pipeline_input;

pub(crate) mod shape_create_media_pipeline_kinesis_video_stream_pool_input;

pub(crate) mod shape_create_media_stream_pipeline_input;

pub(crate) mod shape_forbidden_exception;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_resource_limit_exceeded_exception;

pub(crate) mod shape_service_failure_exception;

pub(crate) mod shape_service_unavailable_exception;

pub(crate) mod shape_start_speaker_search_task_input;

pub(crate) mod shape_start_voice_tone_analysis_task_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttled_client_exception;

pub(crate) mod shape_unauthorized_client_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_media_insights_pipeline_configuration_input;

pub(crate) mod shape_update_media_insights_pipeline_status_input;

pub(crate) mod shape_update_media_pipeline_kinesis_video_stream_pool_input;

pub(crate) mod shape_chime_sdk_meeting_configuration;

pub(crate) mod shape_concatenation_sink;

pub(crate) mod shape_concatenation_source;

pub(crate) mod shape_kinesis_video_stream_configuration;

pub(crate) mod shape_kinesis_video_stream_configuration_update;

pub(crate) mod shape_kinesis_video_stream_pool_configuration;

pub(crate) mod shape_kinesis_video_stream_pool_summary_list;

pub(crate) mod shape_kinesis_video_stream_recording_source_runtime_configuration;

pub(crate) mod shape_kinesis_video_stream_source_runtime_configuration;

pub(crate) mod shape_kinesis_video_stream_source_task_configuration;

pub(crate) mod shape_live_connector_sink_configuration;

pub(crate) mod shape_live_connector_source_configuration;

pub(crate) mod shape_media_capture_pipeline;

pub(crate) mod shape_media_capture_pipeline_summary_list;

pub(crate) mod shape_media_concatenation_pipeline;

pub(crate) mod shape_media_insights_pipeline;

pub(crate) mod shape_media_insights_pipeline_configuration;

pub(crate) mod shape_media_insights_pipeline_configuration_element;

pub(crate) mod shape_media_insights_pipeline_configuration_summary_list;

pub(crate) mod shape_media_live_connector_pipeline;

pub(crate) mod shape_media_pipeline;

pub(crate) mod shape_media_pipeline_list;

pub(crate) mod shape_media_stream_pipeline;

pub(crate) mod shape_media_stream_sink;

pub(crate) mod shape_media_stream_source;

pub(crate) mod shape_real_time_alert_configuration;

pub(crate) mod shape_s3_recording_sink_runtime_configuration;

pub(crate) mod shape_speaker_search_task;

pub(crate) mod shape_sse_aws_key_management_params;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_voice_tone_analysis_task;

pub(crate) mod shape_amazon_transcribe_call_analytics_processor_configuration;

pub(crate) mod shape_amazon_transcribe_processor_configuration;

pub(crate) mod shape_artifacts_configuration;

pub(crate) mod shape_chime_sdk_meeting_live_connector_configuration;

pub(crate) mod shape_concatenation_sink_list;

pub(crate) mod shape_concatenation_source_list;

pub(crate) mod shape_fragment_selector;

pub(crate) mod shape_kinesis_data_stream_sink_configuration;

pub(crate) mod shape_kinesis_video_stream_pool_summary;

pub(crate) mod shape_lambda_function_sink_configuration;

pub(crate) mod shape_live_connector_rtmp_configuration;

pub(crate) mod shape_live_connector_sink_list;

pub(crate) mod shape_live_connector_source_list;

pub(crate) mod shape_media_capture_pipeline_source_configuration;

pub(crate) mod shape_media_capture_pipeline_summary;

pub(crate) mod shape_media_insights_pipeline_configuration_elements;

pub(crate) mod shape_media_insights_pipeline_configuration_summary;

pub(crate) mod shape_media_insights_pipeline_element_statuses;

pub(crate) mod shape_media_insights_runtime_metadata;

pub(crate) mod shape_media_pipeline_summary;

pub(crate) mod shape_media_stream_sink_list;

pub(crate) mod shape_media_stream_source_list;

pub(crate) mod shape_real_time_alert_rule;

pub(crate) mod shape_recording_stream_configuration;

pub(crate) mod shape_s3_bucket_sink_configuration;

pub(crate) mod shape_s3_recording_sink_configuration;

pub(crate) mod shape_sns_topic_sink_configuration;

pub(crate) mod shape_source_configuration;

pub(crate) mod shape_sqs_queue_sink_configuration;

pub(crate) mod shape_stream_configuration;

pub(crate) mod shape_voice_analytics_processor_configuration;

pub(crate) mod shape_voice_enhancement_sink_configuration;

pub(crate) mod shape_audio_artifacts_configuration;

pub(crate) mod shape_chime_sdk_meeting_concatenation_configuration;

pub(crate) mod shape_composited_video_artifacts_configuration;

pub(crate) mod shape_content_artifacts_configuration;

pub(crate) mod shape_issue_detection_configuration;

pub(crate) mod shape_keyword_match_configuration;

pub(crate) mod shape_media_insights_pipeline_element_status;

pub(crate) mod shape_post_call_analytics_settings;

pub(crate) mod shape_real_time_alert_rule_list;

pub(crate) mod shape_recording_stream_list;

pub(crate) mod shape_selected_video_streams;

pub(crate) mod shape_sentiment_configuration;

pub(crate) mod shape_stream_channel_definition;

pub(crate) mod shape_streams;

pub(crate) mod shape_timestamp_range;

pub(crate) mod shape_video_artifacts_configuration;

pub(crate) mod shape_artifacts_concatenation_configuration;

pub(crate) mod shape_channel_definition;

pub(crate) mod shape_grid_view_configuration;

pub(crate) mod shape_active_speaker_only_configuration;

pub(crate) mod shape_attendee_id_list;

pub(crate) mod shape_audio_concatenation_configuration;

pub(crate) mod shape_category_name_list;

pub(crate) mod shape_composited_video_concatenation_configuration;

pub(crate) mod shape_content_concatenation_configuration;

pub(crate) mod shape_data_channel_concatenation_configuration;

pub(crate) mod shape_external_user_id_list;

pub(crate) mod shape_horizontal_layout_configuration;

pub(crate) mod shape_meeting_events_concatenation_configuration;

pub(crate) mod shape_presenter_only_configuration;

pub(crate) mod shape_transcription_messages_concatenation_configuration;

pub(crate) mod shape_vertical_layout_configuration;

pub(crate) mod shape_video_attribute;

pub(crate) mod shape_video_concatenation_configuration;

pub(crate) mod shape_channel_definitions;

pub(crate) mod shape_keyword_match_word_list;
