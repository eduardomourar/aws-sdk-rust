// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"success":true}
pub(crate) fn match_describe_apps_c955e57777ec0d736(
    _result: ::std::result::Result<&crate::operation::describe_apps::DescribeAppsOutput, &crate::operation::describe_apps::DescribeAppsError>,
) -> bool {
    _result.is_ok()
}

/// Matcher union: {"success":false}
pub(crate) fn match_describe_apps_06e5f7e2d702e0110(
    _result: ::std::result::Result<&crate::operation::describe_apps::DescribeAppsOutput, &crate::operation::describe_apps::DescribeAppsError>,
) -> bool {
    _result.is_err()
}

/// Matcher union: {"output":{"path":"Deployments[].Status","expected":"successful","comparator":"allStringEquals"}}
pub(crate) fn match_describe_deployments_e773d13ad26dfe4cc(
    _result: ::std::result::Result<
        &crate::operation::describe_deployments::DescribeDeploymentsOutput,
        &crate::operation::describe_deployments::DescribeDeploymentsError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_deployments::DescribeDeploymentsOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.deployments.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Deployment) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "successful";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Deployments[].Status","expected":"failed","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_deployments_dc40b1ad835f576c0(
    _result: ::std::result::Result<
        &crate::operation::describe_deployments::DescribeDeploymentsOutput,
        &crate::operation::describe_deployments::DescribeDeploymentsError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_deployments::DescribeDeploymentsOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.deployments.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Deployment) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "failed";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"online","comparator":"allStringEquals"}}
pub(crate) fn match_describe_instances_879e5860f33174a71(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "online";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"setup_failed","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_ae8d9fe9dcb532dd4(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "setup_failed";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"shutting_down","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_cb65e3427d01d7667(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "shutting_down";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"start_failed","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_c942b3f1e73dfae7f(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "start_failed";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"stopped","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_dad92608d71b243c3(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "stopped";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"stopping","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_6eb5e99b738450c30(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "stopping";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"terminating","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_f086d9d7330075a64(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "terminating";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"terminated","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_e88d0bb4db07224fa(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "terminated";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"stop_failed","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_8d7f49fcbbbfabdf5(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "stop_failed";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"registered","comparator":"allStringEquals"}}
pub(crate) fn match_describe_instances_20d7399f283b555ce(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "registered";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"stopped","comparator":"allStringEquals"}}
pub(crate) fn match_describe_instances_e418e6a07b1ce7336(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "stopped";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"booting","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_3d4fffbbb77860e29(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "booting";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"pending","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_a139a4c958fa25611(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "pending";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"rebooting","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_5e5e2df8010ad623d(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "rebooting";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"requested","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_23c8afba15d15ee2a(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "requested";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"running_setup","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_0f39a1b9274d8ccda(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "running_setup";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"terminated","comparator":"allStringEquals"}}
pub(crate) fn match_describe_instances_7510952b0735ba013(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "terminated";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"ResourceNotFoundException"}
pub(crate) fn match_describe_instances_1cce2c05524fb92d4(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ResourceNotFoundException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"Instances[].Status","expected":"online","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_instances_0ec6c686a5d9310f4(
    _result: ::std::result::Result<
        &crate::operation::describe_instances::DescribeInstancesOutput,
        &crate::operation::describe_instances::DescribeInstancesError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_instances::DescribeInstancesOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.instances.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Instance) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            value.iter().any(|value| {
                let _tmp_2 = value.as_str();
                let right = "online";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}
