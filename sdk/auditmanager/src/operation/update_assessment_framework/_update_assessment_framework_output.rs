// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAssessmentFrameworkOutput {
    /// <p>The framework object.</p>
    pub framework: ::std::option::Option<crate::types::Framework>,
    _request_id: Option<String>,
}
impl UpdateAssessmentFrameworkOutput {
    /// <p>The framework object.</p>
    pub fn framework(&self) -> ::std::option::Option<&crate::types::Framework> {
        self.framework.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for UpdateAssessmentFrameworkOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateAssessmentFrameworkOutput {
    /// Creates a new builder-style object to manufacture [`UpdateAssessmentFrameworkOutput`](crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput).
    pub fn builder() -> crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkOutputBuilder {
        crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkOutputBuilder::default()
    }
}

/// A builder for [`UpdateAssessmentFrameworkOutput`](crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateAssessmentFrameworkOutputBuilder {
    pub(crate) framework: ::std::option::Option<crate::types::Framework>,
    _request_id: Option<String>,
}
impl UpdateAssessmentFrameworkOutputBuilder {
    /// <p>The framework object.</p>
    pub fn framework(mut self, input: crate::types::Framework) -> Self {
        self.framework = ::std::option::Option::Some(input);
        self
    }
    /// <p>The framework object.</p>
    pub fn set_framework(mut self, input: ::std::option::Option<crate::types::Framework>) -> Self {
        self.framework = input;
        self
    }
    /// <p>The framework object.</p>
    pub fn get_framework(&self) -> &::std::option::Option<crate::types::Framework> {
        &self.framework
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateAssessmentFrameworkOutput`](crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput).
    pub fn build(self) -> crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput {
        crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput {
            framework: self.framework,
            _request_id: self._request_id,
        }
    }
}
