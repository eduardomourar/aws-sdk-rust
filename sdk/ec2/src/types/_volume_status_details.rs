// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a volume status.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VolumeStatusDetails {
    /// <p>The name of the volume status.</p>
    /// <ul>
    /// <li>
    /// <p><code>io-enabled</code> - Indicates the volume I/O status. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/monitoring-volume-checks.html">Amazon EBS volume status checks</a>.</p></li>
    /// <li>
    /// <p><code>io-performance</code> - Indicates the volume performance status. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/monitoring-volume-checks.html">Amazon EBS volume status checks</a>.</p></li>
    /// <li>
    /// <p><code>initialization-state</code> - Indicates the status of the volume initialization process. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/initalize-volume.html">Initialize Amazon EBS volumes</a>.</p></li>
    /// </ul>
    pub name: ::std::option::Option<crate::types::VolumeStatusName>,
    /// <p>The intended status of the volume status.</p>
    pub status: ::std::option::Option<::std::string::String>,
}
impl VolumeStatusDetails {
    /// <p>The name of the volume status.</p>
    /// <ul>
    /// <li>
    /// <p><code>io-enabled</code> - Indicates the volume I/O status. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/monitoring-volume-checks.html">Amazon EBS volume status checks</a>.</p></li>
    /// <li>
    /// <p><code>io-performance</code> - Indicates the volume performance status. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/monitoring-volume-checks.html">Amazon EBS volume status checks</a>.</p></li>
    /// <li>
    /// <p><code>initialization-state</code> - Indicates the status of the volume initialization process. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/initalize-volume.html">Initialize Amazon EBS volumes</a>.</p></li>
    /// </ul>
    pub fn name(&self) -> ::std::option::Option<&crate::types::VolumeStatusName> {
        self.name.as_ref()
    }
    /// <p>The intended status of the volume status.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
}
impl VolumeStatusDetails {
    /// Creates a new builder-style object to manufacture [`VolumeStatusDetails`](crate::types::VolumeStatusDetails).
    pub fn builder() -> crate::types::builders::VolumeStatusDetailsBuilder {
        crate::types::builders::VolumeStatusDetailsBuilder::default()
    }
}

/// A builder for [`VolumeStatusDetails`](crate::types::VolumeStatusDetails).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct VolumeStatusDetailsBuilder {
    pub(crate) name: ::std::option::Option<crate::types::VolumeStatusName>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
}
impl VolumeStatusDetailsBuilder {
    /// <p>The name of the volume status.</p>
    /// <ul>
    /// <li>
    /// <p><code>io-enabled</code> - Indicates the volume I/O status. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/monitoring-volume-checks.html">Amazon EBS volume status checks</a>.</p></li>
    /// <li>
    /// <p><code>io-performance</code> - Indicates the volume performance status. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/monitoring-volume-checks.html">Amazon EBS volume status checks</a>.</p></li>
    /// <li>
    /// <p><code>initialization-state</code> - Indicates the status of the volume initialization process. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/initalize-volume.html">Initialize Amazon EBS volumes</a>.</p></li>
    /// </ul>
    pub fn name(mut self, input: crate::types::VolumeStatusName) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the volume status.</p>
    /// <ul>
    /// <li>
    /// <p><code>io-enabled</code> - Indicates the volume I/O status. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/monitoring-volume-checks.html">Amazon EBS volume status checks</a>.</p></li>
    /// <li>
    /// <p><code>io-performance</code> - Indicates the volume performance status. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/monitoring-volume-checks.html">Amazon EBS volume status checks</a>.</p></li>
    /// <li>
    /// <p><code>initialization-state</code> - Indicates the status of the volume initialization process. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/initalize-volume.html">Initialize Amazon EBS volumes</a>.</p></li>
    /// </ul>
    pub fn set_name(mut self, input: ::std::option::Option<crate::types::VolumeStatusName>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the volume status.</p>
    /// <ul>
    /// <li>
    /// <p><code>io-enabled</code> - Indicates the volume I/O status. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/monitoring-volume-checks.html">Amazon EBS volume status checks</a>.</p></li>
    /// <li>
    /// <p><code>io-performance</code> - Indicates the volume performance status. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/monitoring-volume-checks.html">Amazon EBS volume status checks</a>.</p></li>
    /// <li>
    /// <p><code>initialization-state</code> - Indicates the status of the volume initialization process. For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/initalize-volume.html">Initialize Amazon EBS volumes</a>.</p></li>
    /// </ul>
    pub fn get_name(&self) -> &::std::option::Option<crate::types::VolumeStatusName> {
        &self.name
    }
    /// <p>The intended status of the volume status.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The intended status of the volume status.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The intended status of the volume status.</p>
    pub fn get_status(&self) -> &::std::option::Option<::std::string::String> {
        &self.status
    }
    /// Consumes the builder and constructs a [`VolumeStatusDetails`](crate::types::VolumeStatusDetails).
    pub fn build(self) -> crate::types::VolumeStatusDetails {
        crate::types::VolumeStatusDetails {
            name: self.name,
            status: self.status,
        }
    }
}
