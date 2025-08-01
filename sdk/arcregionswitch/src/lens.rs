// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_get_plan_evaluation_status_output_output_next_token(
    input: &crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_get_plan_execution_output_output_next_token(
    input: &crate::operation::get_plan_execution::GetPlanExecutionOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_plan_execution_events_output_output_next_token(
    input: &crate::operation::list_plan_execution_events::ListPlanExecutionEventsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_plan_executions_output_output_next_token(
    input: &crate::operation::list_plan_executions::ListPlanExecutionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_plans_output_output_next_token(
    input: &crate::operation::list_plans::ListPlansOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_plans_in_region_output_output_next_token(
    input: &crate::operation::list_plans_in_region::ListPlansInRegionOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_route53_health_checks_output_output_next_token(
    input: &crate::operation::list_route53_health_checks::ListRoute53HealthChecksOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_plan_evaluation_status_output_output_warnings(
    input: crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ResourceWarning>> {
    let input = input.warnings?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_plan_execution_output_output_step_states(
    input: crate::operation::get_plan_execution::GetPlanExecutionOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::StepState>> {
    let input = input.step_states?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_plan_execution_events_output_output_items(
    input: crate::operation::list_plan_execution_events::ListPlanExecutionEventsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ExecutionEvent>> {
    let input = input.items?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_plan_executions_output_output_items(
    input: crate::operation::list_plan_executions::ListPlanExecutionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AbbreviatedExecution>> {
    let input = input.items?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_plans_output_output_plans(
    input: crate::operation::list_plans::ListPlansOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AbbreviatedPlan>> {
    let input = input.plans?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_plans_in_region_output_output_plans(
    input: crate::operation::list_plans_in_region::ListPlansInRegionOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AbbreviatedPlan>> {
    let input = input.plans?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_route53_health_checks_output_output_health_checks(
    input: crate::operation::list_route53_health_checks::ListRoute53HealthChecksOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Route53HealthCheck>> {
    let input = input.health_checks?;
    ::std::option::Option::Some(input)
}
