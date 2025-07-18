// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"output":{"path":"status","expected":"succeeded","comparator":"stringEquals"}}
pub(crate) fn match_get_transformer_job_fa4f19743ff32766a(
    _result: ::std::result::Result<
        &crate::operation::get_transformer_job::GetTransformerJobOutput,
        &crate::operation::get_transformer_job::GetTransformerJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_transformer_job::GetTransformerJobOutput,
    ) -> ::std::option::Option<&'a crate::types::TransformerJobStatus> {
        let _fld_1 = &_output.status;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "succeeded";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"status","expected":"failed","comparator":"stringEquals"}}
pub(crate) fn match_get_transformer_job_616b9614615c4607e(
    _result: ::std::result::Result<
        &crate::operation::get_transformer_job::GetTransformerJobOutput,
        &crate::operation::get_transformer_job::GetTransformerJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_transformer_job::GetTransformerJobOutput,
    ) -> ::std::option::Option<&'a crate::types::TransformerJobStatus> {
        let _fld_1 = &_output.status;
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
