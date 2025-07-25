// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The protocol configuration for an agent runtime. This structure defines how the agent runtime communicates with clients.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProtocolConfiguration {
    /// <p>The server protocol for the agent runtime. This field specifies which protocol the agent runtime uses to communicate with clients.</p>
    pub server_protocol: crate::types::ServerProtocol,
}
impl ProtocolConfiguration {
    /// <p>The server protocol for the agent runtime. This field specifies which protocol the agent runtime uses to communicate with clients.</p>
    pub fn server_protocol(&self) -> &crate::types::ServerProtocol {
        &self.server_protocol
    }
}
impl ProtocolConfiguration {
    /// Creates a new builder-style object to manufacture [`ProtocolConfiguration`](crate::types::ProtocolConfiguration).
    pub fn builder() -> crate::types::builders::ProtocolConfigurationBuilder {
        crate::types::builders::ProtocolConfigurationBuilder::default()
    }
}

/// A builder for [`ProtocolConfiguration`](crate::types::ProtocolConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ProtocolConfigurationBuilder {
    pub(crate) server_protocol: ::std::option::Option<crate::types::ServerProtocol>,
}
impl ProtocolConfigurationBuilder {
    /// <p>The server protocol for the agent runtime. This field specifies which protocol the agent runtime uses to communicate with clients.</p>
    /// This field is required.
    pub fn server_protocol(mut self, input: crate::types::ServerProtocol) -> Self {
        self.server_protocol = ::std::option::Option::Some(input);
        self
    }
    /// <p>The server protocol for the agent runtime. This field specifies which protocol the agent runtime uses to communicate with clients.</p>
    pub fn set_server_protocol(mut self, input: ::std::option::Option<crate::types::ServerProtocol>) -> Self {
        self.server_protocol = input;
        self
    }
    /// <p>The server protocol for the agent runtime. This field specifies which protocol the agent runtime uses to communicate with clients.</p>
    pub fn get_server_protocol(&self) -> &::std::option::Option<crate::types::ServerProtocol> {
        &self.server_protocol
    }
    /// Consumes the builder and constructs a [`ProtocolConfiguration`](crate::types::ProtocolConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`server_protocol`](crate::types::builders::ProtocolConfigurationBuilder::server_protocol)
    pub fn build(self) -> ::std::result::Result<crate::types::ProtocolConfiguration, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ProtocolConfiguration {
            server_protocol: self.server_protocol.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "server_protocol",
                    "server_protocol was not specified but it is required when building ProtocolConfiguration",
                )
            })?,
        })
    }
}
