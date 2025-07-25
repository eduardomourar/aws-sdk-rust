// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary of the image set metadata.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImageSetsMetadataSummary {
    /// <p>The image set identifier.</p>
    pub image_set_id: ::std::string::String,
    /// <p>The image set version.</p>
    pub version: ::std::option::Option<i32>,
    /// <p>The time an image set is created. Sample creation date is provided in <code>1985-04-12T23:20:50.52Z</code> format.</p>
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time an image set was last updated.</p>
    pub updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The DICOM tags associated with the image set.</p>
    pub dicom_tags: ::std::option::Option<crate::types::DicomTags>,
    /// <p>The flag to determine whether the image set is primary or not.</p>
    pub is_primary: ::std::option::Option<bool>,
}
impl ImageSetsMetadataSummary {
    /// <p>The image set identifier.</p>
    pub fn image_set_id(&self) -> &str {
        use std::ops::Deref;
        self.image_set_id.deref()
    }
    /// <p>The image set version.</p>
    pub fn version(&self) -> ::std::option::Option<i32> {
        self.version
    }
    /// <p>The time an image set is created. Sample creation date is provided in <code>1985-04-12T23:20:50.52Z</code> format.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The time an image set was last updated.</p>
    pub fn updated_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.updated_at.as_ref()
    }
    /// <p>The DICOM tags associated with the image set.</p>
    pub fn dicom_tags(&self) -> ::std::option::Option<&crate::types::DicomTags> {
        self.dicom_tags.as_ref()
    }
    /// <p>The flag to determine whether the image set is primary or not.</p>
    pub fn is_primary(&self) -> ::std::option::Option<bool> {
        self.is_primary
    }
}
impl ImageSetsMetadataSummary {
    /// Creates a new builder-style object to manufacture [`ImageSetsMetadataSummary`](crate::types::ImageSetsMetadataSummary).
    pub fn builder() -> crate::types::builders::ImageSetsMetadataSummaryBuilder {
        crate::types::builders::ImageSetsMetadataSummaryBuilder::default()
    }
}

/// A builder for [`ImageSetsMetadataSummary`](crate::types::ImageSetsMetadataSummary).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ImageSetsMetadataSummaryBuilder {
    pub(crate) image_set_id: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<i32>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) dicom_tags: ::std::option::Option<crate::types::DicomTags>,
    pub(crate) is_primary: ::std::option::Option<bool>,
}
impl ImageSetsMetadataSummaryBuilder {
    /// <p>The image set identifier.</p>
    /// This field is required.
    pub fn image_set_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_set_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The image set identifier.</p>
    pub fn set_image_set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_set_id = input;
        self
    }
    /// <p>The image set identifier.</p>
    pub fn get_image_set_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.image_set_id
    }
    /// <p>The image set version.</p>
    pub fn version(mut self, input: i32) -> Self {
        self.version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The image set version.</p>
    pub fn set_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.version = input;
        self
    }
    /// <p>The image set version.</p>
    pub fn get_version(&self) -> &::std::option::Option<i32> {
        &self.version
    }
    /// <p>The time an image set is created. Sample creation date is provided in <code>1985-04-12T23:20:50.52Z</code> format.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time an image set is created. Sample creation date is provided in <code>1985-04-12T23:20:50.52Z</code> format.</p>
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The time an image set is created. Sample creation date is provided in <code>1985-04-12T23:20:50.52Z</code> format.</p>
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// <p>The time an image set was last updated.</p>
    pub fn updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time an image set was last updated.</p>
    pub fn set_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_at = input;
        self
    }
    /// <p>The time an image set was last updated.</p>
    pub fn get_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_at
    }
    /// <p>The DICOM tags associated with the image set.</p>
    pub fn dicom_tags(mut self, input: crate::types::DicomTags) -> Self {
        self.dicom_tags = ::std::option::Option::Some(input);
        self
    }
    /// <p>The DICOM tags associated with the image set.</p>
    pub fn set_dicom_tags(mut self, input: ::std::option::Option<crate::types::DicomTags>) -> Self {
        self.dicom_tags = input;
        self
    }
    /// <p>The DICOM tags associated with the image set.</p>
    pub fn get_dicom_tags(&self) -> &::std::option::Option<crate::types::DicomTags> {
        &self.dicom_tags
    }
    /// <p>The flag to determine whether the image set is primary or not.</p>
    pub fn is_primary(mut self, input: bool) -> Self {
        self.is_primary = ::std::option::Option::Some(input);
        self
    }
    /// <p>The flag to determine whether the image set is primary or not.</p>
    pub fn set_is_primary(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_primary = input;
        self
    }
    /// <p>The flag to determine whether the image set is primary or not.</p>
    pub fn get_is_primary(&self) -> &::std::option::Option<bool> {
        &self.is_primary
    }
    /// Consumes the builder and constructs a [`ImageSetsMetadataSummary`](crate::types::ImageSetsMetadataSummary).
    /// This method will fail if any of the following fields are not set:
    /// - [`image_set_id`](crate::types::builders::ImageSetsMetadataSummaryBuilder::image_set_id)
    pub fn build(self) -> ::std::result::Result<crate::types::ImageSetsMetadataSummary, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ImageSetsMetadataSummary {
            image_set_id: self.image_set_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "image_set_id",
                    "image_set_id was not specified but it is required when building ImageSetsMetadataSummary",
                )
            })?,
            version: self.version,
            created_at: self.created_at,
            updated_at: self.updated_at,
            dicom_tags: self.dicom_tags,
            is_primary: self.is_primary,
        })
    }
}
