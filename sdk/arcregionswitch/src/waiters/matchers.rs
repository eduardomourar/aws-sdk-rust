// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"output":{"path":"evaluationState","expected":"passed","comparator":"stringEquals"}}
pub(crate) fn match_get_plan_evaluation_status_e561bcbfa6a90a8a3(
    _result: ::std::result::Result<
        &crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
        &crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
    ) -> ::std::option::Option<&'a crate::types::EvaluationStatus> {
        let _fld_1 = _output.evaluation_state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "passed";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"evaluationState","expected":"actionRequired","comparator":"stringEquals"}}
pub(crate) fn match_get_plan_evaluation_status_484dee2c76832f011(
    _result: ::std::result::Result<
        &crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
        &crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
    ) -> ::std::option::Option<&'a crate::types::EvaluationStatus> {
        let _fld_1 = _output.evaluation_state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "actionRequired";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"evaluationState","expected":"pendingEvaluation","comparator":"stringEquals"}}
pub(crate) fn match_get_plan_evaluation_status_23bfb24027fe40125(
    _result: ::std::result::Result<
        &crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
        &crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
    ) -> ::std::option::Option<&'a crate::types::EvaluationStatus> {
        let _fld_1 = _output.evaluation_state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "pendingEvaluation";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"executionState","expected":"completed","comparator":"stringEquals"}}
pub(crate) fn match_get_plan_execution_d6d50c338ff5a44a9(
    _result: ::std::result::Result<
        &crate::operation::get_plan_execution::GetPlanExecutionOutput,
        &crate::operation::get_plan_execution::GetPlanExecutionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_plan_execution::GetPlanExecutionOutput,
    ) -> ::std::option::Option<&'a crate::types::ExecutionState> {
        let _fld_1 = &_output.execution_state;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "completed";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"executionState","expected":"completedWithExceptions","comparator":"stringEquals"}}
pub(crate) fn match_get_plan_execution_73c5112ac2d8d28a1(
    _result: ::std::result::Result<
        &crate::operation::get_plan_execution::GetPlanExecutionOutput,
        &crate::operation::get_plan_execution::GetPlanExecutionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_plan_execution::GetPlanExecutionOutput,
    ) -> ::std::option::Option<&'a crate::types::ExecutionState> {
        let _fld_1 = &_output.execution_state;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "completedWithExceptions";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"executionState","expected":"failed","comparator":"stringEquals"}}
pub(crate) fn match_get_plan_execution_3a1384a8d3c648582(
    _result: ::std::result::Result<
        &crate::operation::get_plan_execution::GetPlanExecutionOutput,
        &crate::operation::get_plan_execution::GetPlanExecutionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_plan_execution::GetPlanExecutionOutput,
    ) -> ::std::option::Option<&'a crate::types::ExecutionState> {
        let _fld_1 = &_output.execution_state;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "failed";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"executionState","expected":"canceled","comparator":"stringEquals"}}
pub(crate) fn match_get_plan_execution_a509573ecc4408e2d(
    _result: ::std::result::Result<
        &crate::operation::get_plan_execution::GetPlanExecutionOutput,
        &crate::operation::get_plan_execution::GetPlanExecutionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_plan_execution::GetPlanExecutionOutput,
    ) -> ::std::option::Option<&'a crate::types::ExecutionState> {
        let _fld_1 = &_output.execution_state;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "canceled";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"executionState","expected":"planExecutionTimedOut","comparator":"stringEquals"}}
pub(crate) fn match_get_plan_execution_d2e91ed24bf36ed01(
    _result: ::std::result::Result<
        &crate::operation::get_plan_execution::GetPlanExecutionOutput,
        &crate::operation::get_plan_execution::GetPlanExecutionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_plan_execution::GetPlanExecutionOutput,
    ) -> ::std::option::Option<&'a crate::types::ExecutionState> {
        let _fld_1 = &_output.execution_state;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "planExecutionTimedOut";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}
