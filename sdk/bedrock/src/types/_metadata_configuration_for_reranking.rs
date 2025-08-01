// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configuration for how metadata should be used during the reranking process in Knowledge Base vector searches. This determines which metadata fields are included or excluded when reordering search results.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MetadataConfigurationForReranking {
    /// <p>The mode for selecting which metadata fields to include in the reranking process. Valid values are ALL (use all available metadata fields) or SELECTIVE (use only specified fields).</p>
    pub selection_mode: crate::types::RerankingMetadataSelectionMode,
    /// <p>Configuration for selective mode, which allows you to explicitly include or exclude specific metadata fields during reranking. This is only used when selectionMode is set to SELECTIVE.</p>
    pub selective_mode_configuration: ::std::option::Option<crate::types::RerankingMetadataSelectiveModeConfiguration>,
}
impl MetadataConfigurationForReranking {
    /// <p>The mode for selecting which metadata fields to include in the reranking process. Valid values are ALL (use all available metadata fields) or SELECTIVE (use only specified fields).</p>
    pub fn selection_mode(&self) -> &crate::types::RerankingMetadataSelectionMode {
        &self.selection_mode
    }
    /// <p>Configuration for selective mode, which allows you to explicitly include or exclude specific metadata fields during reranking. This is only used when selectionMode is set to SELECTIVE.</p>
    pub fn selective_mode_configuration(&self) -> ::std::option::Option<&crate::types::RerankingMetadataSelectiveModeConfiguration> {
        self.selective_mode_configuration.as_ref()
    }
}
impl MetadataConfigurationForReranking {
    /// Creates a new builder-style object to manufacture [`MetadataConfigurationForReranking`](crate::types::MetadataConfigurationForReranking).
    pub fn builder() -> crate::types::builders::MetadataConfigurationForRerankingBuilder {
        crate::types::builders::MetadataConfigurationForRerankingBuilder::default()
    }
}

/// A builder for [`MetadataConfigurationForReranking`](crate::types::MetadataConfigurationForReranking).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct MetadataConfigurationForRerankingBuilder {
    pub(crate) selection_mode: ::std::option::Option<crate::types::RerankingMetadataSelectionMode>,
    pub(crate) selective_mode_configuration: ::std::option::Option<crate::types::RerankingMetadataSelectiveModeConfiguration>,
}
impl MetadataConfigurationForRerankingBuilder {
    /// <p>The mode for selecting which metadata fields to include in the reranking process. Valid values are ALL (use all available metadata fields) or SELECTIVE (use only specified fields).</p>
    /// This field is required.
    pub fn selection_mode(mut self, input: crate::types::RerankingMetadataSelectionMode) -> Self {
        self.selection_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>The mode for selecting which metadata fields to include in the reranking process. Valid values are ALL (use all available metadata fields) or SELECTIVE (use only specified fields).</p>
    pub fn set_selection_mode(mut self, input: ::std::option::Option<crate::types::RerankingMetadataSelectionMode>) -> Self {
        self.selection_mode = input;
        self
    }
    /// <p>The mode for selecting which metadata fields to include in the reranking process. Valid values are ALL (use all available metadata fields) or SELECTIVE (use only specified fields).</p>
    pub fn get_selection_mode(&self) -> &::std::option::Option<crate::types::RerankingMetadataSelectionMode> {
        &self.selection_mode
    }
    /// <p>Configuration for selective mode, which allows you to explicitly include or exclude specific metadata fields during reranking. This is only used when selectionMode is set to SELECTIVE.</p>
    pub fn selective_mode_configuration(mut self, input: crate::types::RerankingMetadataSelectiveModeConfiguration) -> Self {
        self.selective_mode_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration for selective mode, which allows you to explicitly include or exclude specific metadata fields during reranking. This is only used when selectionMode is set to SELECTIVE.</p>
    pub fn set_selective_mode_configuration(
        mut self,
        input: ::std::option::Option<crate::types::RerankingMetadataSelectiveModeConfiguration>,
    ) -> Self {
        self.selective_mode_configuration = input;
        self
    }
    /// <p>Configuration for selective mode, which allows you to explicitly include or exclude specific metadata fields during reranking. This is only used when selectionMode is set to SELECTIVE.</p>
    pub fn get_selective_mode_configuration(&self) -> &::std::option::Option<crate::types::RerankingMetadataSelectiveModeConfiguration> {
        &self.selective_mode_configuration
    }
    /// Consumes the builder and constructs a [`MetadataConfigurationForReranking`](crate::types::MetadataConfigurationForReranking).
    /// This method will fail if any of the following fields are not set:
    /// - [`selection_mode`](crate::types::builders::MetadataConfigurationForRerankingBuilder::selection_mode)
    pub fn build(self) -> ::std::result::Result<crate::types::MetadataConfigurationForReranking, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::MetadataConfigurationForReranking {
            selection_mode: self.selection_mode.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "selection_mode",
                    "selection_mode was not specified but it is required when building MetadataConfigurationForReranking",
                )
            })?,
            selective_mode_configuration: self.selective_mode_configuration,
        })
    }
}
