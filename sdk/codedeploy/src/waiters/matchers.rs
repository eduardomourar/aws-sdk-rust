// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"output":{"path":"deploymentInfo.status","expected":"Succeeded","comparator":"stringEquals"}}
pub(crate) fn match_get_deployment_71cb2e8fd4234325e(
    _result: ::std::result::Result<&crate::operation::get_deployment::GetDeploymentOutput, &crate::operation::get_deployment::GetDeploymentError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_deployment::GetDeploymentOutput,
    ) -> ::std::option::Option<&'a crate::types::DeploymentStatus> {
        let _fld_1 = _output.deployment_info.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Succeeded";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"deploymentInfo.status","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_get_deployment_817cfc4588b7c2c91(
    _result: ::std::result::Result<&crate::operation::get_deployment::GetDeploymentOutput, &crate::operation::get_deployment::GetDeploymentError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_deployment::GetDeploymentOutput,
    ) -> ::std::option::Option<&'a crate::types::DeploymentStatus> {
        let _fld_1 = _output.deployment_info.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Failed";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"deploymentInfo.status","expected":"Stopped","comparator":"stringEquals"}}
pub(crate) fn match_get_deployment_3fade3e995b1dd6db(
    _result: ::std::result::Result<&crate::operation::get_deployment::GetDeploymentOutput, &crate::operation::get_deployment::GetDeploymentError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_deployment::GetDeploymentOutput,
    ) -> ::std::option::Option<&'a crate::types::DeploymentStatus> {
        let _fld_1 = _output.deployment_info.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Stopped";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}
