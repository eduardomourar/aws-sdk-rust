// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSessionLoggerOutput {
    /// <p>The session logger details.</p>
    pub session_logger: ::std::option::Option<crate::types::SessionLogger>,
    _request_id: Option<String>,
}
impl GetSessionLoggerOutput {
    /// <p>The session logger details.</p>
    pub fn session_logger(&self) -> ::std::option::Option<&crate::types::SessionLogger> {
        self.session_logger.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for GetSessionLoggerOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetSessionLoggerOutput {
    /// Creates a new builder-style object to manufacture [`GetSessionLoggerOutput`](crate::operation::get_session_logger::GetSessionLoggerOutput).
    pub fn builder() -> crate::operation::get_session_logger::builders::GetSessionLoggerOutputBuilder {
        crate::operation::get_session_logger::builders::GetSessionLoggerOutputBuilder::default()
    }
}

/// A builder for [`GetSessionLoggerOutput`](crate::operation::get_session_logger::GetSessionLoggerOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetSessionLoggerOutputBuilder {
    pub(crate) session_logger: ::std::option::Option<crate::types::SessionLogger>,
    _request_id: Option<String>,
}
impl GetSessionLoggerOutputBuilder {
    /// <p>The session logger details.</p>
    pub fn session_logger(mut self, input: crate::types::SessionLogger) -> Self {
        self.session_logger = ::std::option::Option::Some(input);
        self
    }
    /// <p>The session logger details.</p>
    pub fn set_session_logger(mut self, input: ::std::option::Option<crate::types::SessionLogger>) -> Self {
        self.session_logger = input;
        self
    }
    /// <p>The session logger details.</p>
    pub fn get_session_logger(&self) -> &::std::option::Option<crate::types::SessionLogger> {
        &self.session_logger
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetSessionLoggerOutput`](crate::operation::get_session_logger::GetSessionLoggerOutput).
    pub fn build(self) -> crate::operation::get_session_logger::GetSessionLoggerOutput {
        crate::operation::get_session_logger::GetSessionLoggerOutput {
            session_logger: self.session_logger,
            _request_id: self._request_id,
        }
    }
}
