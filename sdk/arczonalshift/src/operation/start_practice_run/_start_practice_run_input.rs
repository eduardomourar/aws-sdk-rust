// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartPracticeRunInput {
    /// <p>The identifier for the resource that you want to start a practice run zonal shift for. The identifier is the Amazon Resource Name (ARN) for the resource.</p>
    pub resource_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The Availability Zone (for example, <code>use1-az1</code>) that traffic is shifted away from for the resource that you specify for the practice run.</p>
    pub away_from: ::std::option::Option<::std::string::String>,
    /// <p>The initial comment that you enter about the practice run. Be aware that this comment can be overwritten by Amazon Web Services if the automatic check for balanced capacity fails. For more information, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.how-it-works.capacity-check.html"> Capacity checks for practice runs</a> in the Amazon Application Recovery Controller Developer Guide.</p>
    pub comment: ::std::option::Option<::std::string::String>,
}
impl StartPracticeRunInput {
    /// <p>The identifier for the resource that you want to start a practice run zonal shift for. The identifier is the Amazon Resource Name (ARN) for the resource.</p>
    pub fn resource_identifier(&self) -> ::std::option::Option<&str> {
        self.resource_identifier.as_deref()
    }
    /// <p>The Availability Zone (for example, <code>use1-az1</code>) that traffic is shifted away from for the resource that you specify for the practice run.</p>
    pub fn away_from(&self) -> ::std::option::Option<&str> {
        self.away_from.as_deref()
    }
    /// <p>The initial comment that you enter about the practice run. Be aware that this comment can be overwritten by Amazon Web Services if the automatic check for balanced capacity fails. For more information, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.how-it-works.capacity-check.html"> Capacity checks for practice runs</a> in the Amazon Application Recovery Controller Developer Guide.</p>
    pub fn comment(&self) -> ::std::option::Option<&str> {
        self.comment.as_deref()
    }
}
impl StartPracticeRunInput {
    /// Creates a new builder-style object to manufacture [`StartPracticeRunInput`](crate::operation::start_practice_run::StartPracticeRunInput).
    pub fn builder() -> crate::operation::start_practice_run::builders::StartPracticeRunInputBuilder {
        crate::operation::start_practice_run::builders::StartPracticeRunInputBuilder::default()
    }
}

/// A builder for [`StartPracticeRunInput`](crate::operation::start_practice_run::StartPracticeRunInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartPracticeRunInputBuilder {
    pub(crate) resource_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) away_from: ::std::option::Option<::std::string::String>,
    pub(crate) comment: ::std::option::Option<::std::string::String>,
}
impl StartPracticeRunInputBuilder {
    /// <p>The identifier for the resource that you want to start a practice run zonal shift for. The identifier is the Amazon Resource Name (ARN) for the resource.</p>
    /// This field is required.
    pub fn resource_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the resource that you want to start a practice run zonal shift for. The identifier is the Amazon Resource Name (ARN) for the resource.</p>
    pub fn set_resource_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_identifier = input;
        self
    }
    /// <p>The identifier for the resource that you want to start a practice run zonal shift for. The identifier is the Amazon Resource Name (ARN) for the resource.</p>
    pub fn get_resource_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_identifier
    }
    /// <p>The Availability Zone (for example, <code>use1-az1</code>) that traffic is shifted away from for the resource that you specify for the practice run.</p>
    /// This field is required.
    pub fn away_from(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.away_from = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Availability Zone (for example, <code>use1-az1</code>) that traffic is shifted away from for the resource that you specify for the practice run.</p>
    pub fn set_away_from(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.away_from = input;
        self
    }
    /// <p>The Availability Zone (for example, <code>use1-az1</code>) that traffic is shifted away from for the resource that you specify for the practice run.</p>
    pub fn get_away_from(&self) -> &::std::option::Option<::std::string::String> {
        &self.away_from
    }
    /// <p>The initial comment that you enter about the practice run. Be aware that this comment can be overwritten by Amazon Web Services if the automatic check for balanced capacity fails. For more information, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.how-it-works.capacity-check.html"> Capacity checks for practice runs</a> in the Amazon Application Recovery Controller Developer Guide.</p>
    /// This field is required.
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.comment = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The initial comment that you enter about the practice run. Be aware that this comment can be overwritten by Amazon Web Services if the automatic check for balanced capacity fails. For more information, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.how-it-works.capacity-check.html"> Capacity checks for practice runs</a> in the Amazon Application Recovery Controller Developer Guide.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.comment = input;
        self
    }
    /// <p>The initial comment that you enter about the practice run. Be aware that this comment can be overwritten by Amazon Web Services if the automatic check for balanced capacity fails. For more information, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.how-it-works.capacity-check.html"> Capacity checks for practice runs</a> in the Amazon Application Recovery Controller Developer Guide.</p>
    pub fn get_comment(&self) -> &::std::option::Option<::std::string::String> {
        &self.comment
    }
    /// Consumes the builder and constructs a [`StartPracticeRunInput`](crate::operation::start_practice_run::StartPracticeRunInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::start_practice_run::StartPracticeRunInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::start_practice_run::StartPracticeRunInput {
            resource_identifier: self.resource_identifier,
            away_from: self.away_from,
            comment: self.comment,
        })
    }
}
