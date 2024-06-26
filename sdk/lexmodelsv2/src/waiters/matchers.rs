// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"output":{"path":"botAliasStatus","expected":"Available","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_alias_9fd6f7fd903716529(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_alias::DescribeBotAliasOutput,
        &crate::operation::describe_bot_alias::DescribeBotAliasError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_alias::DescribeBotAliasOutput,
    ) -> ::std::option::Option<&'a crate::types::BotAliasStatus> {
        let _fld_1 = _output.bot_alias_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Available";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"botAliasStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_alias_938af767887bcb339(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_alias::DescribeBotAliasOutput,
        &crate::operation::describe_bot_alias::DescribeBotAliasError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_alias::DescribeBotAliasOutput,
    ) -> ::std::option::Option<&'a crate::types::BotAliasStatus> {
        let _fld_1 = _output.bot_alias_status.as_ref()?;
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

/// Matcher union: {"output":{"path":"botAliasStatus","expected":"Deleting","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_alias_2c247089f4b76b0dc(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_alias::DescribeBotAliasOutput,
        &crate::operation::describe_bot_alias::DescribeBotAliasError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_alias::DescribeBotAliasOutput,
    ) -> ::std::option::Option<&'a crate::types::BotAliasStatus> {
        let _fld_1 = _output.bot_alias_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Deleting";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"botStatus","expected":"Available","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_b02179601eb168096(
    _result: ::std::result::Result<&crate::operation::describe_bot::DescribeBotOutput, &crate::operation::describe_bot::DescribeBotError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_bot::DescribeBotOutput) -> ::std::option::Option<&'a crate::types::BotStatus> {
        let _fld_1 = _output.bot_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Available";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"botStatus","expected":"Deleting","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_43ee2b959d4054721(
    _result: ::std::result::Result<&crate::operation::describe_bot::DescribeBotOutput, &crate::operation::describe_bot::DescribeBotError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_bot::DescribeBotOutput) -> ::std::option::Option<&'a crate::types::BotStatus> {
        let _fld_1 = _output.bot_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Deleting";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"botStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_3af10a50de73812c2(
    _result: ::std::result::Result<&crate::operation::describe_bot::DescribeBotOutput, &crate::operation::describe_bot::DescribeBotError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_bot::DescribeBotOutput) -> ::std::option::Option<&'a crate::types::BotStatus> {
        let _fld_1 = _output.bot_status.as_ref()?;
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

/// Matcher union: {"output":{"path":"botStatus","expected":"Inactive","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_0043338155d244480(
    _result: ::std::result::Result<&crate::operation::describe_bot::DescribeBotOutput, &crate::operation::describe_bot::DescribeBotError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_bot::DescribeBotOutput) -> ::std::option::Option<&'a crate::types::BotStatus> {
        let _fld_1 = _output.bot_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Inactive";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"exportStatus","expected":"Completed","comparator":"stringEquals"}}
pub(crate) fn match_describe_export_0fad93d22eb17a7c7(
    _result: ::std::result::Result<&crate::operation::describe_export::DescribeExportOutput, &crate::operation::describe_export::DescribeExportError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_export::DescribeExportOutput,
    ) -> ::std::option::Option<&'a crate::types::ExportStatus> {
        let _fld_1 = _output.export_status.as_ref()?;
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

/// Matcher union: {"output":{"path":"exportStatus","expected":"Deleting","comparator":"stringEquals"}}
pub(crate) fn match_describe_export_beaf6176a38283c5c(
    _result: ::std::result::Result<&crate::operation::describe_export::DescribeExportOutput, &crate::operation::describe_export::DescribeExportError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_export::DescribeExportOutput,
    ) -> ::std::option::Option<&'a crate::types::ExportStatus> {
        let _fld_1 = _output.export_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Deleting";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"exportStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_export_660474a45b0a2ce9d(
    _result: ::std::result::Result<&crate::operation::describe_export::DescribeExportOutput, &crate::operation::describe_export::DescribeExportError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_export::DescribeExportOutput,
    ) -> ::std::option::Option<&'a crate::types::ExportStatus> {
        let _fld_1 = _output.export_status.as_ref()?;
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

/// Matcher union: {"output":{"path":"importStatus","expected":"Completed","comparator":"stringEquals"}}
pub(crate) fn match_describe_import_ef35b1ae01b6cfadb(
    _result: ::std::result::Result<&crate::operation::describe_import::DescribeImportOutput, &crate::operation::describe_import::DescribeImportError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_import::DescribeImportOutput,
    ) -> ::std::option::Option<&'a crate::types::ImportStatus> {
        let _fld_1 = _output.import_status.as_ref()?;
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

/// Matcher union: {"output":{"path":"importStatus","expected":"Deleting","comparator":"stringEquals"}}
pub(crate) fn match_describe_import_335ed71c62295e34a(
    _result: ::std::result::Result<&crate::operation::describe_import::DescribeImportOutput, &crate::operation::describe_import::DescribeImportError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_import::DescribeImportOutput,
    ) -> ::std::option::Option<&'a crate::types::ImportStatus> {
        let _fld_1 = _output.import_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Deleting";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"importStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_import_2c0087942704d9abb(
    _result: ::std::result::Result<&crate::operation::describe_import::DescribeImportOutput, &crate::operation::describe_import::DescribeImportError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_import::DescribeImportOutput,
    ) -> ::std::option::Option<&'a crate::types::ImportStatus> {
        let _fld_1 = _output.import_status.as_ref()?;
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

/// Matcher union: {"output":{"path":"botLocaleStatus","expected":"Built","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_locale_8ae807cb857c585fd(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
        &crate::operation::describe_bot_locale::DescribeBotLocaleError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
    ) -> ::std::option::Option<&'a crate::types::BotLocaleStatus> {
        let _fld_1 = _output.bot_locale_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Built";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"botLocaleStatus","expected":"Deleting","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_locale_42575f9e2ecd93849(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
        &crate::operation::describe_bot_locale::DescribeBotLocaleError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
    ) -> ::std::option::Option<&'a crate::types::BotLocaleStatus> {
        let _fld_1 = _output.bot_locale_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Deleting";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"botLocaleStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_locale_48fa8bbe7816af579(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
        &crate::operation::describe_bot_locale::DescribeBotLocaleError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
    ) -> ::std::option::Option<&'a crate::types::BotLocaleStatus> {
        let _fld_1 = _output.bot_locale_status.as_ref()?;
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

/// Matcher union: {"output":{"path":"botLocaleStatus","expected":"NotBuilt","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_locale_b8322e8fa2e87daa4(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
        &crate::operation::describe_bot_locale::DescribeBotLocaleError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
    ) -> ::std::option::Option<&'a crate::types::BotLocaleStatus> {
        let _fld_1 = _output.bot_locale_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "NotBuilt";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"botLocaleStatus","expected":"ReadyExpressTesting","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_locale_47b08019c6b65b97a(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
        &crate::operation::describe_bot_locale::DescribeBotLocaleError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_locale::DescribeBotLocaleOutput,
    ) -> ::std::option::Option<&'a crate::types::BotLocaleStatus> {
        let _fld_1 = _output.bot_locale_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "ReadyExpressTesting";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"botStatus","expected":"Available","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_version_b02179601eb168096(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_version::DescribeBotVersionOutput,
        &crate::operation::describe_bot_version::DescribeBotVersionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_version::DescribeBotVersionOutput,
    ) -> ::std::option::Option<&'a crate::types::BotStatus> {
        let _fld_1 = _output.bot_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Available";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"botStatus","expected":"Deleting","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_version_43ee2b959d4054721(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_version::DescribeBotVersionOutput,
        &crate::operation::describe_bot_version::DescribeBotVersionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_version::DescribeBotVersionOutput,
    ) -> ::std::option::Option<&'a crate::types::BotStatus> {
        let _fld_1 = _output.bot_status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "Deleting";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"botStatus","expected":"Failed","comparator":"stringEquals"}}
pub(crate) fn match_describe_bot_version_3af10a50de73812c2(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_version::DescribeBotVersionOutput,
        &crate::operation::describe_bot_version::DescribeBotVersionError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_bot_version::DescribeBotVersionOutput,
    ) -> ::std::option::Option<&'a crate::types::BotStatus> {
        let _fld_1 = _output.bot_status.as_ref()?;
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

/// Matcher union: {"errorType":"ResourceNotFoundException"}
pub(crate) fn match_describe_bot_version_1cce2c05524fb92d4(
    _result: ::std::result::Result<
        &crate::operation::describe_bot_version::DescribeBotVersionOutput,
        &crate::operation::describe_bot_version::DescribeBotVersionError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ResourceNotFoundException";
        }
    }
    false
}
