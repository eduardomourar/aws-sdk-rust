// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains detailed information about a directory assessment, including configuration parameters, status, and validation results.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Assessment {
    /// <p>The unique identifier of the directory assessment.</p>
    pub assessment_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the directory associated with this assessment.</p>
    pub directory_id: ::std::option::Option<::std::string::String>,
    /// <p>The fully qualified domain name (FQDN) of the Active Directory domain being assessed.</p>
    pub dns_name: ::std::option::Option<::std::string::String>,
    /// <p>The date and time when the assessment was initiated.</p>
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date and time when the assessment status was last updated.</p>
    pub last_update_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The current status of the assessment. Valid values include <code>SUCCESS</code>, <code>FAILED</code>, <code>PENDING</code>, and <code>IN_PROGRESS</code>.</p>
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>A detailed status code providing additional information about the assessment state.</p>
    pub status_code: ::std::option::Option<::std::string::String>,
    /// <p>A human-readable description of the current assessment status, including any error details or progress information.</p>
    pub status_reason: ::std::option::Option<::std::string::String>,
    /// <p>The IP addresses of the DNS servers or domain controllers in your self-managed AD environment.</p>
    pub customer_dns_ips: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Contains Amazon VPC information for the <code>StartADAssessment</code> operation.</p>
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>A list of subnet identifiers in the Amazon VPC in which the hybrid directory is created.</p>
    pub subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The security groups identifiers attached to the network interfaces.</p>
    pub security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The identifiers of the self-managed AD instances used to perform the assessment.</p>
    pub self_managed_instance_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The type of assessment report generated. Valid values are <code>CUSTOMER</code> and <code>SYSTEM</code>.</p>
    pub report_type: ::std::option::Option<::std::string::String>,
    /// <p>The version of the assessment framework used to evaluate your self-managed AD environment.</p>
    pub version: ::std::option::Option<::std::string::String>,
}
impl Assessment {
    /// <p>The unique identifier of the directory assessment.</p>
    pub fn assessment_id(&self) -> ::std::option::Option<&str> {
        self.assessment_id.as_deref()
    }
    /// <p>The identifier of the directory associated with this assessment.</p>
    pub fn directory_id(&self) -> ::std::option::Option<&str> {
        self.directory_id.as_deref()
    }
    /// <p>The fully qualified domain name (FQDN) of the Active Directory domain being assessed.</p>
    pub fn dns_name(&self) -> ::std::option::Option<&str> {
        self.dns_name.as_deref()
    }
    /// <p>The date and time when the assessment was initiated.</p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The date and time when the assessment status was last updated.</p>
    pub fn last_update_date_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_update_date_time.as_ref()
    }
    /// <p>The current status of the assessment. Valid values include <code>SUCCESS</code>, <code>FAILED</code>, <code>PENDING</code>, and <code>IN_PROGRESS</code>.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>A detailed status code providing additional information about the assessment state.</p>
    pub fn status_code(&self) -> ::std::option::Option<&str> {
        self.status_code.as_deref()
    }
    /// <p>A human-readable description of the current assessment status, including any error details or progress information.</p>
    pub fn status_reason(&self) -> ::std::option::Option<&str> {
        self.status_reason.as_deref()
    }
    /// <p>The IP addresses of the DNS servers or domain controllers in your self-managed AD environment.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.customer_dns_ips.is_none()`.
    pub fn customer_dns_ips(&self) -> &[::std::string::String] {
        self.customer_dns_ips.as_deref().unwrap_or_default()
    }
    /// <p>Contains Amazon VPC information for the <code>StartADAssessment</code> operation.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>A list of subnet identifiers in the Amazon VPC in which the hybrid directory is created.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.subnet_ids.is_none()`.
    pub fn subnet_ids(&self) -> &[::std::string::String] {
        self.subnet_ids.as_deref().unwrap_or_default()
    }
    /// <p>The security groups identifiers attached to the network interfaces.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.security_group_ids.is_none()`.
    pub fn security_group_ids(&self) -> &[::std::string::String] {
        self.security_group_ids.as_deref().unwrap_or_default()
    }
    /// <p>The identifiers of the self-managed AD instances used to perform the assessment.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.self_managed_instance_ids.is_none()`.
    pub fn self_managed_instance_ids(&self) -> &[::std::string::String] {
        self.self_managed_instance_ids.as_deref().unwrap_or_default()
    }
    /// <p>The type of assessment report generated. Valid values are <code>CUSTOMER</code> and <code>SYSTEM</code>.</p>
    pub fn report_type(&self) -> ::std::option::Option<&str> {
        self.report_type.as_deref()
    }
    /// <p>The version of the assessment framework used to evaluate your self-managed AD environment.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
}
impl Assessment {
    /// Creates a new builder-style object to manufacture [`Assessment`](crate::types::Assessment).
    pub fn builder() -> crate::types::builders::AssessmentBuilder {
        crate::types::builders::AssessmentBuilder::default()
    }
}

/// A builder for [`Assessment`](crate::types::Assessment).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AssessmentBuilder {
    pub(crate) assessment_id: ::std::option::Option<::std::string::String>,
    pub(crate) directory_id: ::std::option::Option<::std::string::String>,
    pub(crate) dns_name: ::std::option::Option<::std::string::String>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_update_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) status_code: ::std::option::Option<::std::string::String>,
    pub(crate) status_reason: ::std::option::Option<::std::string::String>,
    pub(crate) customer_dns_ips: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) self_managed_instance_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) report_type: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
}
impl AssessmentBuilder {
    /// <p>The unique identifier of the directory assessment.</p>
    pub fn assessment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.assessment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the directory assessment.</p>
    pub fn set_assessment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.assessment_id = input;
        self
    }
    /// <p>The unique identifier of the directory assessment.</p>
    pub fn get_assessment_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.assessment_id
    }
    /// <p>The identifier of the directory associated with this assessment.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.directory_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the directory associated with this assessment.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.directory_id = input;
        self
    }
    /// <p>The identifier of the directory associated with this assessment.</p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.directory_id
    }
    /// <p>The fully qualified domain name (FQDN) of the Active Directory domain being assessed.</p>
    pub fn dns_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dns_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The fully qualified domain name (FQDN) of the Active Directory domain being assessed.</p>
    pub fn set_dns_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dns_name = input;
        self
    }
    /// <p>The fully qualified domain name (FQDN) of the Active Directory domain being assessed.</p>
    pub fn get_dns_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.dns_name
    }
    /// <p>The date and time when the assessment was initiated.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the assessment was initiated.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The date and time when the assessment was initiated.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.start_time
    }
    /// <p>The date and time when the assessment status was last updated.</p>
    pub fn last_update_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_update_date_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the assessment status was last updated.</p>
    pub fn set_last_update_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_update_date_time = input;
        self
    }
    /// <p>The date and time when the assessment status was last updated.</p>
    pub fn get_last_update_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_update_date_time
    }
    /// <p>The current status of the assessment. Valid values include <code>SUCCESS</code>, <code>FAILED</code>, <code>PENDING</code>, and <code>IN_PROGRESS</code>.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The current status of the assessment. Valid values include <code>SUCCESS</code>, <code>FAILED</code>, <code>PENDING</code>, and <code>IN_PROGRESS</code>.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The current status of the assessment. Valid values include <code>SUCCESS</code>, <code>FAILED</code>, <code>PENDING</code>, and <code>IN_PROGRESS</code>.</p>
    pub fn get_status(&self) -> &::std::option::Option<::std::string::String> {
        &self.status
    }
    /// <p>A detailed status code providing additional information about the assessment state.</p>
    pub fn status_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A detailed status code providing additional information about the assessment state.</p>
    pub fn set_status_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_code = input;
        self
    }
    /// <p>A detailed status code providing additional information about the assessment state.</p>
    pub fn get_status_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_code
    }
    /// <p>A human-readable description of the current assessment status, including any error details or progress information.</p>
    pub fn status_reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A human-readable description of the current assessment status, including any error details or progress information.</p>
    pub fn set_status_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_reason = input;
        self
    }
    /// <p>A human-readable description of the current assessment status, including any error details or progress information.</p>
    pub fn get_status_reason(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_reason
    }
    /// Appends an item to `customer_dns_ips`.
    ///
    /// To override the contents of this collection use [`set_customer_dns_ips`](Self::set_customer_dns_ips).
    ///
    /// <p>The IP addresses of the DNS servers or domain controllers in your self-managed AD environment.</p>
    pub fn customer_dns_ips(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.customer_dns_ips.unwrap_or_default();
        v.push(input.into());
        self.customer_dns_ips = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IP addresses of the DNS servers or domain controllers in your self-managed AD environment.</p>
    pub fn set_customer_dns_ips(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.customer_dns_ips = input;
        self
    }
    /// <p>The IP addresses of the DNS servers or domain controllers in your self-managed AD environment.</p>
    pub fn get_customer_dns_ips(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.customer_dns_ips
    }
    /// <p>Contains Amazon VPC information for the <code>StartADAssessment</code> operation.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Contains Amazon VPC information for the <code>StartADAssessment</code> operation.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>Contains Amazon VPC information for the <code>StartADAssessment</code> operation.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_id
    }
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>A list of subnet identifiers in the Amazon VPC in which the hybrid directory is created.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of subnet identifiers in the Amazon VPC in which the hybrid directory is created.</p>
    pub fn set_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.subnet_ids = input;
        self
    }
    /// <p>A list of subnet identifiers in the Amazon VPC in which the hybrid directory is created.</p>
    pub fn get_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.subnet_ids
    }
    /// Appends an item to `security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>The security groups identifiers attached to the network interfaces.</p>
    pub fn security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.security_group_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The security groups identifiers attached to the network interfaces.</p>
    pub fn set_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.security_group_ids = input;
        self
    }
    /// <p>The security groups identifiers attached to the network interfaces.</p>
    pub fn get_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.security_group_ids
    }
    /// Appends an item to `self_managed_instance_ids`.
    ///
    /// To override the contents of this collection use [`set_self_managed_instance_ids`](Self::set_self_managed_instance_ids).
    ///
    /// <p>The identifiers of the self-managed AD instances used to perform the assessment.</p>
    pub fn self_managed_instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.self_managed_instance_ids.unwrap_or_default();
        v.push(input.into());
        self.self_managed_instance_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The identifiers of the self-managed AD instances used to perform the assessment.</p>
    pub fn set_self_managed_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.self_managed_instance_ids = input;
        self
    }
    /// <p>The identifiers of the self-managed AD instances used to perform the assessment.</p>
    pub fn get_self_managed_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.self_managed_instance_ids
    }
    /// <p>The type of assessment report generated. Valid values are <code>CUSTOMER</code> and <code>SYSTEM</code>.</p>
    pub fn report_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.report_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of assessment report generated. Valid values are <code>CUSTOMER</code> and <code>SYSTEM</code>.</p>
    pub fn set_report_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.report_type = input;
        self
    }
    /// <p>The type of assessment report generated. Valid values are <code>CUSTOMER</code> and <code>SYSTEM</code>.</p>
    pub fn get_report_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.report_type
    }
    /// <p>The version of the assessment framework used to evaluate your self-managed AD environment.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the assessment framework used to evaluate your self-managed AD environment.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// <p>The version of the assessment framework used to evaluate your self-managed AD environment.</p>
    pub fn get_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.version
    }
    /// Consumes the builder and constructs a [`Assessment`](crate::types::Assessment).
    pub fn build(self) -> crate::types::Assessment {
        crate::types::Assessment {
            assessment_id: self.assessment_id,
            directory_id: self.directory_id,
            dns_name: self.dns_name,
            start_time: self.start_time,
            last_update_date_time: self.last_update_date_time,
            status: self.status,
            status_code: self.status_code,
            status_reason: self.status_reason,
            customer_dns_ips: self.customer_dns_ips,
            vpc_id: self.vpc_id,
            subnet_ids: self.subnet_ids,
            security_group_ids: self.security_group_ids,
            self_managed_instance_ids: self.self_managed_instance_ids,
            report_type: self.report_type,
            version: self.version,
        }
    }
}
