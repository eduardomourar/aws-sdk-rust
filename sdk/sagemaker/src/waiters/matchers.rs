// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"errorType":"ValidationException"}
pub(crate) fn match_describe_endpoint_19c82d26244f04729(
    _result: ::std::result::Result<
        &crate::operation::describe_endpoint::DescribeEndpointOutput,
        &crate::operation::describe_endpoint::DescribeEndpointError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ValidationException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"EndpointStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_endpoint_1085a59fe5e4bed05(
    _result: ::std::result::Result<
        &crate::operation::describe_endpoint::DescribeEndpointOutput,
        &crate::operation::describe_endpoint::DescribeEndpointError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_endpoint::DescribeEndpointOutput,
    ) -> ::std::option::Option<&'a crate::types::EndpointStatus> {
        let _fld_1 = _output.endpoint_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
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

/// Matcher union: {"output":{"path":"EndpointStatus","expected":"InService","comparator":"stringEquals"}}
pub(crate) fn match_describe_endpoint_4d43aabb81e9d1e61(
    _result: ::std::result::Result<
        &crate::operation::describe_endpoint::DescribeEndpointOutput,
        &crate::operation::describe_endpoint::DescribeEndpointError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_endpoint::DescribeEndpointOutput,
    ) -> ::std::option::Option<&'a crate::types::EndpointStatus> {
        let _fld_1 = _output.endpoint_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "InService";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"ImageStatus","expected":"CREATED","comparator":"stringEquals"}}
pub(crate) fn match_describe_image_c762f23df3f388e1e(
    _result: ::std::result::Result<&crate::operation::describe_image::DescribeImageOutput, &crate::operation::describe_image::DescribeImageError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_image::DescribeImageOutput,
    ) -> ::std::option::Option<&'a crate::types::ImageStatus> {
        let _fld_1 = _output.image_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"ImageStatus","expected":"CREATE_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_describe_image_27890bd28399b6bae(
    _result: ::std::result::Result<&crate::operation::describe_image::DescribeImageOutput, &crate::operation::describe_image::DescribeImageError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_image::DescribeImageOutput,
    ) -> ::std::option::Option<&'a crate::types::ImageStatus> {
        let _fld_1 = _output.image_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATE_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"ValidationException"}
pub(crate) fn match_describe_image_19c82d26244f04729(
    _result: ::std::result::Result<&crate::operation::describe_image::DescribeImageOutput, &crate::operation::describe_image::DescribeImageError>,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ValidationException";
        }
    }
    false
}

/// Matcher union: {"errorType":"ResourceNotFoundException"}
pub(crate) fn match_describe_image_1cce2c05524fb92d4(
    _result: ::std::result::Result<&crate::operation::describe_image::DescribeImageOutput, &crate::operation::describe_image::DescribeImageError>,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ResourceNotFoundException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"ImageStatus","expected":"DELETE_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_describe_image_e7466e69c260a9e62(
    _result: ::std::result::Result<&crate::operation::describe_image::DescribeImageOutput, &crate::operation::describe_image::DescribeImageError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_image::DescribeImageOutput,
    ) -> ::std::option::Option<&'a crate::types::ImageStatus> {
        let _fld_1 = _output.image_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETE_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"ImageStatus","expected":"UPDATE_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_describe_image_9b4d2d3be79e89bf6(
    _result: ::std::result::Result<&crate::operation::describe_image::DescribeImageOutput, &crate::operation::describe_image::DescribeImageError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_image::DescribeImageOutput,
    ) -> ::std::option::Option<&'a crate::types::ImageStatus> {
        let _fld_1 = _output.image_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "UPDATE_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"ImageVersionStatus","expected":"CREATED","comparator":"stringEquals"}}
pub(crate) fn match_describe_image_version_3e437bf5fdf17cfc6(
    _result: ::std::result::Result<
        &crate::operation::describe_image_version::DescribeImageVersionOutput,
        &crate::operation::describe_image_version::DescribeImageVersionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_image_version::DescribeImageVersionOutput,
    ) -> ::std::option::Option<&'a crate::types::ImageVersionStatus> {
        let _fld_1 = _output.image_version_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"ImageVersionStatus","expected":"CREATE_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_describe_image_version_6f23d4602c6e621d4(
    _result: ::std::result::Result<
        &crate::operation::describe_image_version::DescribeImageVersionOutput,
        &crate::operation::describe_image_version::DescribeImageVersionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_image_version::DescribeImageVersionOutput,
    ) -> ::std::option::Option<&'a crate::types::ImageVersionStatus> {
        let _fld_1 = _output.image_version_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATE_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"ValidationException"}
pub(crate) fn match_describe_image_version_19c82d26244f04729(
    _result: ::std::result::Result<
        &crate::operation::describe_image_version::DescribeImageVersionOutput,
        &crate::operation::describe_image_version::DescribeImageVersionError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ValidationException";
        }
    }
    false
}

/// Matcher union: {"errorType":"ResourceNotFoundException"}
pub(crate) fn match_describe_image_version_1cce2c05524fb92d4(
    _result: ::std::result::Result<
        &crate::operation::describe_image_version::DescribeImageVersionOutput,
        &crate::operation::describe_image_version::DescribeImageVersionError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ResourceNotFoundException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"ImageVersionStatus","expected":"DELETE_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_describe_image_version_909c2ed49ed2e7c9c(
    _result: ::std::result::Result<
        &crate::operation::describe_image_version::DescribeImageVersionOutput,
        &crate::operation::describe_image_version::DescribeImageVersionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_image_version::DescribeImageVersionOutput,
    ) -> ::std::option::Option<&'a crate::types::ImageVersionStatus> {
        let _fld_1 = _output.image_version_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETE_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"ValidationException"}
pub(crate) fn match_describe_notebook_instance_19c82d26244f04729(
    _result: ::std::result::Result<
        &crate::operation::describe_notebook_instance::DescribeNotebookInstanceOutput,
        &crate::operation::describe_notebook_instance::DescribeNotebookInstanceError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ValidationException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"NotebookInstanceStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_notebook_instance_7c9f1d3d76a99c6a7(
    _result: ::std::result::Result<
        &crate::operation::describe_notebook_instance::DescribeNotebookInstanceOutput,
        &crate::operation::describe_notebook_instance::DescribeNotebookInstanceError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_notebook_instance::DescribeNotebookInstanceOutput,
    ) -> ::std::option::Option<&'a crate::types::NotebookInstanceStatus> {
        let _fld_1 = _output.notebook_instance_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
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

/// Matcher union: {"output":{"path":"NotebookInstanceStatus","expected":"InService","comparator":"stringEquals"}}
pub(crate) fn match_describe_notebook_instance_ff262dc167062f094(
    _result: ::std::result::Result<
        &crate::operation::describe_notebook_instance::DescribeNotebookInstanceOutput,
        &crate::operation::describe_notebook_instance::DescribeNotebookInstanceError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_notebook_instance::DescribeNotebookInstanceOutput,
    ) -> ::std::option::Option<&'a crate::types::NotebookInstanceStatus> {
        let _fld_1 = _output.notebook_instance_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "InService";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"NotebookInstanceStatus","expected":"Stopped","comparator":"stringEquals"}}
pub(crate) fn match_describe_notebook_instance_8057f3ba4eee7dbef(
    _result: ::std::result::Result<
        &crate::operation::describe_notebook_instance::DescribeNotebookInstanceOutput,
        &crate::operation::describe_notebook_instance::DescribeNotebookInstanceError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_notebook_instance::DescribeNotebookInstanceOutput,
    ) -> ::std::option::Option<&'a crate::types::NotebookInstanceStatus> {
        let _fld_1 = _output.notebook_instance_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
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

/// Matcher union: {"output":{"path":"ProcessingJobStatus","expected":"Completed","comparator":"stringEquals"}}
pub(crate) fn match_describe_processing_job_4528a45ba6417ef81(
    _result: ::std::result::Result<
        &crate::operation::describe_processing_job::DescribeProcessingJobOutput,
        &crate::operation::describe_processing_job::DescribeProcessingJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_processing_job::DescribeProcessingJobOutput,
    ) -> ::std::option::Option<&'a crate::types::ProcessingJobStatus> {
        let _fld_1 = _output.processing_job_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Completed";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"ProcessingJobStatus","expected":"Stopped","comparator":"stringEquals"}}
pub(crate) fn match_describe_processing_job_f8e876ddd55d0c150(
    _result: ::std::result::Result<
        &crate::operation::describe_processing_job::DescribeProcessingJobOutput,
        &crate::operation::describe_processing_job::DescribeProcessingJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_processing_job::DescribeProcessingJobOutput,
    ) -> ::std::option::Option<&'a crate::types::ProcessingJobStatus> {
        let _fld_1 = _output.processing_job_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
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

/// Matcher union: {"output":{"path":"ProcessingJobStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_processing_job_5533ce4b8ba52bbbf(
    _result: ::std::result::Result<
        &crate::operation::describe_processing_job::DescribeProcessingJobOutput,
        &crate::operation::describe_processing_job::DescribeProcessingJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_processing_job::DescribeProcessingJobOutput,
    ) -> ::std::option::Option<&'a crate::types::ProcessingJobStatus> {
        let _fld_1 = _output.processing_job_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
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

/// Matcher union: {"errorType":"ValidationException"}
pub(crate) fn match_describe_processing_job_19c82d26244f04729(
    _result: ::std::result::Result<
        &crate::operation::describe_processing_job::DescribeProcessingJobOutput,
        &crate::operation::describe_processing_job::DescribeProcessingJobError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ValidationException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"TrainingJobStatus","expected":"Completed","comparator":"stringEquals"}}
pub(crate) fn match_describe_training_job_95b4bc45043b9d4dc(
    _result: ::std::result::Result<
        &crate::operation::describe_training_job::DescribeTrainingJobOutput,
        &crate::operation::describe_training_job::DescribeTrainingJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_training_job::DescribeTrainingJobOutput,
    ) -> ::std::option::Option<&'a crate::types::TrainingJobStatus> {
        let _fld_1 = _output.training_job_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Completed";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"TrainingJobStatus","expected":"Stopped","comparator":"stringEquals"}}
pub(crate) fn match_describe_training_job_4d7ab72a9eaf2c016(
    _result: ::std::result::Result<
        &crate::operation::describe_training_job::DescribeTrainingJobOutput,
        &crate::operation::describe_training_job::DescribeTrainingJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_training_job::DescribeTrainingJobOutput,
    ) -> ::std::option::Option<&'a crate::types::TrainingJobStatus> {
        let _fld_1 = _output.training_job_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
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

/// Matcher union: {"output":{"path":"TrainingJobStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_training_job_8e51e307a7c13c9a7(
    _result: ::std::result::Result<
        &crate::operation::describe_training_job::DescribeTrainingJobOutput,
        &crate::operation::describe_training_job::DescribeTrainingJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_training_job::DescribeTrainingJobOutput,
    ) -> ::std::option::Option<&'a crate::types::TrainingJobStatus> {
        let _fld_1 = _output.training_job_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
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

/// Matcher union: {"errorType":"ValidationException"}
pub(crate) fn match_describe_training_job_19c82d26244f04729(
    _result: ::std::result::Result<
        &crate::operation::describe_training_job::DescribeTrainingJobOutput,
        &crate::operation::describe_training_job::DescribeTrainingJobError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ValidationException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"TransformJobStatus","expected":"Completed","comparator":"stringEquals"}}
pub(crate) fn match_describe_transform_job_d9577c7471dd42b7a(
    _result: ::std::result::Result<
        &crate::operation::describe_transform_job::DescribeTransformJobOutput,
        &crate::operation::describe_transform_job::DescribeTransformJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_transform_job::DescribeTransformJobOutput,
    ) -> ::std::option::Option<&'a crate::types::TransformJobStatus> {
        let _fld_1 = _output.transform_job_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Completed";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"TransformJobStatus","expected":"Stopped","comparator":"stringEquals"}}
pub(crate) fn match_describe_transform_job_b42a5a6f67af61abe(
    _result: ::std::result::Result<
        &crate::operation::describe_transform_job::DescribeTransformJobOutput,
        &crate::operation::describe_transform_job::DescribeTransformJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_transform_job::DescribeTransformJobOutput,
    ) -> ::std::option::Option<&'a crate::types::TransformJobStatus> {
        let _fld_1 = _output.transform_job_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
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

/// Matcher union: {"output":{"path":"TransformJobStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_transform_job_a567a8b226279fe48(
    _result: ::std::result::Result<
        &crate::operation::describe_transform_job::DescribeTransformJobOutput,
        &crate::operation::describe_transform_job::DescribeTransformJobError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_transform_job::DescribeTransformJobOutput,
    ) -> ::std::option::Option<&'a crate::types::TransformJobStatus> {
        let _fld_1 = _output.transform_job_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
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

/// Matcher union: {"errorType":"ValidationException"}
pub(crate) fn match_describe_transform_job_19c82d26244f04729(
    _result: ::std::result::Result<
        &crate::operation::describe_transform_job::DescribeTransformJobOutput,
        &crate::operation::describe_transform_job::DescribeTransformJobError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ValidationException";
        }
    }
    false
}
