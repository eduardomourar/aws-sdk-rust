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

pub(crate) mod shape_create_batch_inference_job;

pub(crate) mod shape_create_batch_segment_job;

pub(crate) mod shape_create_campaign;

pub(crate) mod shape_create_data_deletion_job;

pub(crate) mod shape_create_dataset;

pub(crate) mod shape_create_dataset_export_job;

pub(crate) mod shape_create_dataset_group;

pub(crate) mod shape_create_dataset_import_job;

pub(crate) mod shape_create_event_tracker;

pub(crate) mod shape_create_filter;

pub(crate) mod shape_create_metric_attribution;

pub(crate) mod shape_create_recommender;

pub(crate) mod shape_create_schema;

pub(crate) mod shape_create_solution;

pub(crate) mod shape_create_solution_version;

pub(crate) mod shape_delete_campaign;

pub(crate) mod shape_delete_dataset;

pub(crate) mod shape_delete_dataset_group;

pub(crate) mod shape_delete_event_tracker;

pub(crate) mod shape_delete_filter;

pub(crate) mod shape_delete_metric_attribution;

pub(crate) mod shape_delete_recommender;

pub(crate) mod shape_delete_schema;

pub(crate) mod shape_delete_solution;

pub(crate) mod shape_describe_algorithm;

pub(crate) mod shape_describe_batch_inference_job;

pub(crate) mod shape_describe_batch_segment_job;

pub(crate) mod shape_describe_campaign;

pub(crate) mod shape_describe_data_deletion_job;

pub(crate) mod shape_describe_dataset;

pub(crate) mod shape_describe_dataset_export_job;

pub(crate) mod shape_describe_dataset_group;

pub(crate) mod shape_describe_dataset_import_job;

pub(crate) mod shape_describe_event_tracker;

pub(crate) mod shape_describe_feature_transformation;

pub(crate) mod shape_describe_filter;

pub(crate) mod shape_describe_metric_attribution;

pub(crate) mod shape_describe_recipe;

pub(crate) mod shape_describe_recommender;

pub(crate) mod shape_describe_schema;

pub(crate) mod shape_describe_solution;

pub(crate) mod shape_describe_solution_version;

pub(crate) mod shape_get_solution_metrics;

pub(crate) mod shape_list_batch_inference_jobs;

pub(crate) mod shape_list_batch_segment_jobs;

pub(crate) mod shape_list_campaigns;

pub(crate) mod shape_list_data_deletion_jobs;

pub(crate) mod shape_list_dataset_export_jobs;

pub(crate) mod shape_list_dataset_groups;

pub(crate) mod shape_list_dataset_import_jobs;

pub(crate) mod shape_list_datasets;

pub(crate) mod shape_list_event_trackers;

pub(crate) mod shape_list_filters;

pub(crate) mod shape_list_metric_attribution_metrics;

pub(crate) mod shape_list_metric_attributions;

pub(crate) mod shape_list_recipes;

pub(crate) mod shape_list_recommenders;

pub(crate) mod shape_list_schemas;

pub(crate) mod shape_list_solution_versions;

pub(crate) mod shape_list_solutions;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_start_recommender;

pub(crate) mod shape_stop_recommender;

pub(crate) mod shape_stop_solution_version_creation;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_campaign;

pub(crate) mod shape_update_dataset;

pub(crate) mod shape_update_metric_attribution;

pub(crate) mod shape_update_recommender;

pub(crate) mod shape_update_solution;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_create_batch_inference_job_input;

pub(crate) mod shape_create_batch_segment_job_input;

pub(crate) mod shape_create_campaign_input;

pub(crate) mod shape_create_data_deletion_job_input;

pub(crate) mod shape_create_dataset_export_job_input;

pub(crate) mod shape_create_dataset_group_input;

pub(crate) mod shape_create_dataset_import_job_input;

pub(crate) mod shape_create_dataset_input;

pub(crate) mod shape_create_event_tracker_input;

pub(crate) mod shape_create_filter_input;

pub(crate) mod shape_create_metric_attribution_input;

pub(crate) mod shape_create_recommender_input;

pub(crate) mod shape_create_schema_input;

pub(crate) mod shape_create_solution_input;

pub(crate) mod shape_create_solution_version_input;

pub(crate) mod shape_delete_campaign_input;

pub(crate) mod shape_delete_dataset_group_input;

pub(crate) mod shape_delete_dataset_input;

pub(crate) mod shape_delete_event_tracker_input;

pub(crate) mod shape_delete_filter_input;

pub(crate) mod shape_delete_metric_attribution_input;

pub(crate) mod shape_delete_recommender_input;

pub(crate) mod shape_delete_schema_input;

pub(crate) mod shape_delete_solution_input;

pub(crate) mod shape_describe_algorithm_input;

pub(crate) mod shape_describe_batch_inference_job_input;

pub(crate) mod shape_describe_batch_segment_job_input;

pub(crate) mod shape_describe_campaign_input;

pub(crate) mod shape_describe_data_deletion_job_input;

pub(crate) mod shape_describe_dataset_export_job_input;

pub(crate) mod shape_describe_dataset_group_input;

pub(crate) mod shape_describe_dataset_import_job_input;

pub(crate) mod shape_describe_dataset_input;

pub(crate) mod shape_describe_event_tracker_input;

pub(crate) mod shape_describe_feature_transformation_input;

pub(crate) mod shape_describe_filter_input;

pub(crate) mod shape_describe_metric_attribution_input;

pub(crate) mod shape_describe_recipe_input;

pub(crate) mod shape_describe_recommender_input;

pub(crate) mod shape_describe_schema_input;

pub(crate) mod shape_describe_solution_input;

pub(crate) mod shape_describe_solution_version_input;

pub(crate) mod shape_get_solution_metrics_input;

pub(crate) mod shape_invalid_input_exception;

pub(crate) mod shape_invalid_next_token_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_list_batch_inference_jobs_input;

pub(crate) mod shape_list_batch_segment_jobs_input;

pub(crate) mod shape_list_campaigns_input;

pub(crate) mod shape_list_data_deletion_jobs_input;

pub(crate) mod shape_list_dataset_export_jobs_input;

pub(crate) mod shape_list_dataset_groups_input;

pub(crate) mod shape_list_dataset_import_jobs_input;

pub(crate) mod shape_list_datasets_input;

pub(crate) mod shape_list_event_trackers_input;

pub(crate) mod shape_list_filters_input;

pub(crate) mod shape_list_metric_attribution_metrics_input;

pub(crate) mod shape_list_metric_attributions_input;

pub(crate) mod shape_list_recipes_input;

pub(crate) mod shape_list_recommenders_input;

pub(crate) mod shape_list_schemas_input;

pub(crate) mod shape_list_solution_versions_input;

pub(crate) mod shape_list_solutions_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_resource_already_exists_exception;

pub(crate) mod shape_resource_in_use_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_start_recommender_input;

pub(crate) mod shape_stop_recommender_input;

pub(crate) mod shape_stop_solution_version_creation_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_too_many_tag_keys_exception;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_campaign_input;

pub(crate) mod shape_update_dataset_input;

pub(crate) mod shape_update_metric_attribution_input;

pub(crate) mod shape_update_recommender_input;

pub(crate) mod shape_update_solution_input;

pub(crate) mod shape_algorithm;

pub(crate) mod shape_batch_inference_job;

pub(crate) mod shape_batch_inference_job_config;

pub(crate) mod shape_batch_inference_job_input;

pub(crate) mod shape_batch_inference_job_output;

pub(crate) mod shape_batch_inference_jobs;

pub(crate) mod shape_batch_segment_job;

pub(crate) mod shape_batch_segment_job_input;

pub(crate) mod shape_batch_segment_job_output;

pub(crate) mod shape_batch_segment_jobs;

pub(crate) mod shape_campaign;

pub(crate) mod shape_campaign_config;

pub(crate) mod shape_campaigns;

pub(crate) mod shape_data_deletion_job;

pub(crate) mod shape_data_deletion_jobs;

pub(crate) mod shape_data_source;

pub(crate) mod shape_dataset;

pub(crate) mod shape_dataset_export_job;

pub(crate) mod shape_dataset_export_job_output;

pub(crate) mod shape_dataset_export_jobs;

pub(crate) mod shape_dataset_group;

pub(crate) mod shape_dataset_groups;

pub(crate) mod shape_dataset_import_job;

pub(crate) mod shape_dataset_import_jobs;

pub(crate) mod shape_dataset_schema;

pub(crate) mod shape_datasets;

pub(crate) mod shape_event_tracker;

pub(crate) mod shape_event_trackers;

pub(crate) mod shape_feature_transformation;

pub(crate) mod shape_filter;

pub(crate) mod shape_filters;

pub(crate) mod shape_metric_attribute;

pub(crate) mod shape_metric_attributes;

pub(crate) mod shape_metric_attribution;

pub(crate) mod shape_metric_attribution_output;

pub(crate) mod shape_metric_attributions;

pub(crate) mod shape_metrics;

pub(crate) mod shape_recipe;

pub(crate) mod shape_recipes;

pub(crate) mod shape_recommender;

pub(crate) mod shape_recommender_config;

pub(crate) mod shape_recommenders;

pub(crate) mod shape_schemas;

pub(crate) mod shape_solution;

pub(crate) mod shape_solution_config;

pub(crate) mod shape_solution_update_config;

pub(crate) mod shape_solution_version;

pub(crate) mod shape_solution_versions;

pub(crate) mod shape_solutions;

pub(crate) mod shape_tag;

pub(crate) mod shape_tags;

pub(crate) mod shape_theme_generation_config;

pub(crate) mod shape_algorithm_image;

pub(crate) mod shape_auto_ml_config;

pub(crate) mod shape_auto_ml_result;

pub(crate) mod shape_auto_training_config;

pub(crate) mod shape_batch_inference_job_summary;

pub(crate) mod shape_batch_segment_job_summary;

pub(crate) mod shape_campaign_summary;

pub(crate) mod shape_campaign_update_summary;

pub(crate) mod shape_data_deletion_job_summary;

pub(crate) mod shape_dataset_export_job_summary;

pub(crate) mod shape_dataset_group_summary;

pub(crate) mod shape_dataset_import_job_summary;

pub(crate) mod shape_dataset_schema_summary;

pub(crate) mod shape_dataset_summary;

pub(crate) mod shape_dataset_update_summary;

pub(crate) mod shape_default_hyper_parameter_ranges;

pub(crate) mod shape_event_tracker_summary;

pub(crate) mod shape_events_config;

pub(crate) mod shape_featurization_parameters;

pub(crate) mod shape_fields_for_theme_generation;

pub(crate) mod shape_filter_summary;

pub(crate) mod shape_hpo_config;

pub(crate) mod shape_hyper_parameters;

pub(crate) mod shape_metric_attribution_summary;

pub(crate) mod shape_optimization_objective;

pub(crate) mod shape_recipe_summary;

pub(crate) mod shape_recommender_summary;

pub(crate) mod shape_recommender_update_summary;

pub(crate) mod shape_resource_config;

pub(crate) mod shape_s3_data_config;

pub(crate) mod shape_solution_summary;

pub(crate) mod shape_solution_update_summary;

pub(crate) mod shape_solution_version_summary;

pub(crate) mod shape_training_data_config;

pub(crate) mod shape_tuned_hpo_params;

pub(crate) mod shape_default_categorical_hyper_parameter_ranges;

pub(crate) mod shape_default_continuous_hyper_parameter_ranges;

pub(crate) mod shape_default_integer_hyper_parameter_ranges;

pub(crate) mod shape_event_parameters;

pub(crate) mod shape_feature_transformation_parameters;

pub(crate) mod shape_hpo_objective;

pub(crate) mod shape_hpo_resource_config;

pub(crate) mod shape_hyper_parameter_ranges;

pub(crate) mod shape_arn_list;

pub(crate) mod shape_categorical_hyper_parameter_range;

pub(crate) mod shape_continuous_hyper_parameter_range;

pub(crate) mod shape_default_categorical_hyper_parameter_range;

pub(crate) mod shape_default_continuous_hyper_parameter_range;

pub(crate) mod shape_default_integer_hyper_parameter_range;

pub(crate) mod shape_event_parameters_list;

pub(crate) mod shape_excluded_dataset_columns;

pub(crate) mod shape_integer_hyper_parameter_range;

pub(crate) mod shape_categorical_hyper_parameter_ranges;

pub(crate) mod shape_categorical_values;

pub(crate) mod shape_column_names_list;

pub(crate) mod shape_continuous_hyper_parameter_ranges;

pub(crate) mod shape_integer_hyper_parameter_ranges;
