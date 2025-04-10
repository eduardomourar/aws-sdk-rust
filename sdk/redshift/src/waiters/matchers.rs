// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"output":{"path":"Clusters[].ClusterStatus","expected":"available","comparator":"allStringEquals"}}
pub(crate) fn match_describe_clusters_d94f699084f934f23(
    _result: ::std::result::Result<
        &crate::operation::describe_clusters::DescribeClustersOutput,
        &crate::operation::describe_clusters::DescribeClustersError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_clusters::DescribeClustersOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.clusters.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Cluster) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.cluster_status.as_ref();
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
                    let right = "available";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Clusters[].ClusterStatus","expected":"deleting","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_clusters_36fb459f8f7fb96c3(
    _result: ::std::result::Result<
        &crate::operation::describe_clusters::DescribeClustersOutput,
        &crate::operation::describe_clusters::DescribeClustersError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_clusters::DescribeClustersOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.clusters.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Cluster) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.cluster_status.as_ref();
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
                let right = "deleting";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"ClusterNotFound"}
pub(crate) fn match_describe_clusters_f8bb8a172c7bfc68f(
    _result: ::std::result::Result<
        &crate::operation::describe_clusters::DescribeClustersOutput,
        &crate::operation::describe_clusters::DescribeClustersError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ClusterNotFound";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"Clusters[].ClusterStatus","expected":"creating","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_clusters_578c8eab71e8a5d97(
    _result: ::std::result::Result<
        &crate::operation::describe_clusters::DescribeClustersOutput,
        &crate::operation::describe_clusters::DescribeClustersError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_clusters::DescribeClustersOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.clusters.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Cluster) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.cluster_status.as_ref();
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
                let right = "creating";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Clusters[].ClusterStatus","expected":"modifying","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_clusters_fdd41c9d20f5639c9(
    _result: ::std::result::Result<
        &crate::operation::describe_clusters::DescribeClustersOutput,
        &crate::operation::describe_clusters::DescribeClustersError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_clusters::DescribeClustersOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.clusters.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Cluster) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.cluster_status.as_ref();
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
                let right = "modifying";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Clusters[].RestoreStatus.Status","expected":"completed","comparator":"allStringEquals"}}
pub(crate) fn match_describe_clusters_99b2faa1a0233f771(
    _result: ::std::result::Result<
        &crate::operation::describe_clusters::DescribeClustersOutput,
        &crate::operation::describe_clusters::DescribeClustersError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_clusters::DescribeClustersOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.clusters.as_ref()?;
        let _prj_4 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Cluster) -> ::std::option::Option<&::std::string::String> {
                    let _fld_2 = _v.restore_status.as_ref();
                    let _fld_3 = _fld_2.and_then(|v| v.status.as_ref());
                    _fld_3
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_4)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "completed";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Snapshots[].Status","expected":"available","comparator":"allStringEquals"}}
pub(crate) fn match_describe_cluster_snapshots_9940112fa5e89d7d9(
    _result: ::std::result::Result<
        &crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsOutput,
        &crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.snapshots.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Snapshot) -> ::std::option::Option<&::std::string::String> {
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
                    let right = "available";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Snapshots[].Status","expected":"failed","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_cluster_snapshots_c752b4060229720c6(
    _result: ::std::result::Result<
        &crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsOutput,
        &crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.snapshots.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Snapshot) -> ::std::option::Option<&::std::string::String> {
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

/// Matcher union: {"output":{"path":"Snapshots[].Status","expected":"deleted","comparator":"anyStringEquals"}}
pub(crate) fn match_describe_cluster_snapshots_305bbaff8086662e2(
    _result: ::std::result::Result<
        &crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsOutput,
        &crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a ::std::string::String>> {
        let _fld_1 = _output.snapshots.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::Snapshot) -> ::std::option::Option<&::std::string::String> {
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
                let right = "deleted";
                let _cmp_1 = _tmp_2 == right;
                _cmp_1
            })
        })
        .unwrap_or_default()
}
