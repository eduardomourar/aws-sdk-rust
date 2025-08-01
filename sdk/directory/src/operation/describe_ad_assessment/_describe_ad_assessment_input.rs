// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAdAssessmentInput {
    /// <p>The identifier of the directory assessment to describe.</p>
    pub assessment_id: ::std::option::Option<::std::string::String>,
}
impl DescribeAdAssessmentInput {
    /// <p>The identifier of the directory assessment to describe.</p>
    pub fn assessment_id(&self) -> ::std::option::Option<&str> {
        self.assessment_id.as_deref()
    }
}
impl DescribeAdAssessmentInput {
    /// Creates a new builder-style object to manufacture [`DescribeAdAssessmentInput`](crate::operation::describe_ad_assessment::DescribeAdAssessmentInput).
    pub fn builder() -> crate::operation::describe_ad_assessment::builders::DescribeAdAssessmentInputBuilder {
        crate::operation::describe_ad_assessment::builders::DescribeAdAssessmentInputBuilder::default()
    }
}

/// A builder for [`DescribeAdAssessmentInput`](crate::operation::describe_ad_assessment::DescribeAdAssessmentInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeAdAssessmentInputBuilder {
    pub(crate) assessment_id: ::std::option::Option<::std::string::String>,
}
impl DescribeAdAssessmentInputBuilder {
    /// <p>The identifier of the directory assessment to describe.</p>
    /// This field is required.
    pub fn assessment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.assessment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the directory assessment to describe.</p>
    pub fn set_assessment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.assessment_id = input;
        self
    }
    /// <p>The identifier of the directory assessment to describe.</p>
    pub fn get_assessment_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.assessment_id
    }
    /// Consumes the builder and constructs a [`DescribeAdAssessmentInput`](crate::operation::describe_ad_assessment::DescribeAdAssessmentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::describe_ad_assessment::DescribeAdAssessmentInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::describe_ad_assessment::DescribeAdAssessmentInput {
            assessment_id: self.assessment_id,
        })
    }
}
