// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The replicated Region information for a directory.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegionDescription {
    /// <p>The identifier of the directory.</p>
    pub directory_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Region. For example, <code>us-east-1</code>.</p>
    pub region_name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies whether the Region is the primary Region or an additional Region.</p>
    pub region_type: ::std::option::Option<crate::types::RegionType>,
    /// <p>The status of the replication process for the specified Region.</p>
    pub status: ::std::option::Option<crate::types::DirectoryStage>,
    /// <p>Contains VPC information for the <code>CreateDirectory</code>, <code>CreateMicrosoftAD</code>, or <code>CreateHybridAD</code> operation.</p>
    pub vpc_settings: ::std::option::Option<crate::types::DirectoryVpcSettings>,
    /// <p>The desired number of domain controllers in the specified Region for the specified directory.</p>
    pub desired_number_of_domain_controllers: ::std::option::Option<i32>,
    /// <p>Specifies when the Region replication began.</p>
    pub launch_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date and time that the Region status was last updated.</p>
    pub status_last_updated_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date and time that the Region description was last updated.</p>
    pub last_updated_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl RegionDescription {
    /// <p>The identifier of the directory.</p>
    pub fn directory_id(&self) -> ::std::option::Option<&str> {
        self.directory_id.as_deref()
    }
    /// <p>The name of the Region. For example, <code>us-east-1</code>.</p>
    pub fn region_name(&self) -> ::std::option::Option<&str> {
        self.region_name.as_deref()
    }
    /// <p>Specifies whether the Region is the primary Region or an additional Region.</p>
    pub fn region_type(&self) -> ::std::option::Option<&crate::types::RegionType> {
        self.region_type.as_ref()
    }
    /// <p>The status of the replication process for the specified Region.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::DirectoryStage> {
        self.status.as_ref()
    }
    /// <p>Contains VPC information for the <code>CreateDirectory</code>, <code>CreateMicrosoftAD</code>, or <code>CreateHybridAD</code> operation.</p>
    pub fn vpc_settings(&self) -> ::std::option::Option<&crate::types::DirectoryVpcSettings> {
        self.vpc_settings.as_ref()
    }
    /// <p>The desired number of domain controllers in the specified Region for the specified directory.</p>
    pub fn desired_number_of_domain_controllers(&self) -> ::std::option::Option<i32> {
        self.desired_number_of_domain_controllers
    }
    /// <p>Specifies when the Region replication began.</p>
    pub fn launch_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.launch_time.as_ref()
    }
    /// <p>The date and time that the Region status was last updated.</p>
    pub fn status_last_updated_date_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.status_last_updated_date_time.as_ref()
    }
    /// <p>The date and time that the Region description was last updated.</p>
    pub fn last_updated_date_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_date_time.as_ref()
    }
}
impl RegionDescription {
    /// Creates a new builder-style object to manufacture [`RegionDescription`](crate::types::RegionDescription).
    pub fn builder() -> crate::types::builders::RegionDescriptionBuilder {
        crate::types::builders::RegionDescriptionBuilder::default()
    }
}

/// A builder for [`RegionDescription`](crate::types::RegionDescription).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RegionDescriptionBuilder {
    pub(crate) directory_id: ::std::option::Option<::std::string::String>,
    pub(crate) region_name: ::std::option::Option<::std::string::String>,
    pub(crate) region_type: ::std::option::Option<crate::types::RegionType>,
    pub(crate) status: ::std::option::Option<crate::types::DirectoryStage>,
    pub(crate) vpc_settings: ::std::option::Option<crate::types::DirectoryVpcSettings>,
    pub(crate) desired_number_of_domain_controllers: ::std::option::Option<i32>,
    pub(crate) launch_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) status_last_updated_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl RegionDescriptionBuilder {
    /// <p>The identifier of the directory.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.directory_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the directory.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.directory_id = input;
        self
    }
    /// <p>The identifier of the directory.</p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.directory_id
    }
    /// <p>The name of the Region. For example, <code>us-east-1</code>.</p>
    pub fn region_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Region. For example, <code>us-east-1</code>.</p>
    pub fn set_region_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region_name = input;
        self
    }
    /// <p>The name of the Region. For example, <code>us-east-1</code>.</p>
    pub fn get_region_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.region_name
    }
    /// <p>Specifies whether the Region is the primary Region or an additional Region.</p>
    pub fn region_type(mut self, input: crate::types::RegionType) -> Self {
        self.region_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the Region is the primary Region or an additional Region.</p>
    pub fn set_region_type(mut self, input: ::std::option::Option<crate::types::RegionType>) -> Self {
        self.region_type = input;
        self
    }
    /// <p>Specifies whether the Region is the primary Region or an additional Region.</p>
    pub fn get_region_type(&self) -> &::std::option::Option<crate::types::RegionType> {
        &self.region_type
    }
    /// <p>The status of the replication process for the specified Region.</p>
    pub fn status(mut self, input: crate::types::DirectoryStage) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the replication process for the specified Region.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::DirectoryStage>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status of the replication process for the specified Region.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::DirectoryStage> {
        &self.status
    }
    /// <p>Contains VPC information for the <code>CreateDirectory</code>, <code>CreateMicrosoftAD</code>, or <code>CreateHybridAD</code> operation.</p>
    pub fn vpc_settings(mut self, input: crate::types::DirectoryVpcSettings) -> Self {
        self.vpc_settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains VPC information for the <code>CreateDirectory</code>, <code>CreateMicrosoftAD</code>, or <code>CreateHybridAD</code> operation.</p>
    pub fn set_vpc_settings(mut self, input: ::std::option::Option<crate::types::DirectoryVpcSettings>) -> Self {
        self.vpc_settings = input;
        self
    }
    /// <p>Contains VPC information for the <code>CreateDirectory</code>, <code>CreateMicrosoftAD</code>, or <code>CreateHybridAD</code> operation.</p>
    pub fn get_vpc_settings(&self) -> &::std::option::Option<crate::types::DirectoryVpcSettings> {
        &self.vpc_settings
    }
    /// <p>The desired number of domain controllers in the specified Region for the specified directory.</p>
    pub fn desired_number_of_domain_controllers(mut self, input: i32) -> Self {
        self.desired_number_of_domain_controllers = ::std::option::Option::Some(input);
        self
    }
    /// <p>The desired number of domain controllers in the specified Region for the specified directory.</p>
    pub fn set_desired_number_of_domain_controllers(mut self, input: ::std::option::Option<i32>) -> Self {
        self.desired_number_of_domain_controllers = input;
        self
    }
    /// <p>The desired number of domain controllers in the specified Region for the specified directory.</p>
    pub fn get_desired_number_of_domain_controllers(&self) -> &::std::option::Option<i32> {
        &self.desired_number_of_domain_controllers
    }
    /// <p>Specifies when the Region replication began.</p>
    pub fn launch_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.launch_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies when the Region replication began.</p>
    pub fn set_launch_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.launch_time = input;
        self
    }
    /// <p>Specifies when the Region replication began.</p>
    pub fn get_launch_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.launch_time
    }
    /// <p>The date and time that the Region status was last updated.</p>
    pub fn status_last_updated_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.status_last_updated_date_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time that the Region status was last updated.</p>
    pub fn set_status_last_updated_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.status_last_updated_date_time = input;
        self
    }
    /// <p>The date and time that the Region status was last updated.</p>
    pub fn get_status_last_updated_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.status_last_updated_date_time
    }
    /// <p>The date and time that the Region description was last updated.</p>
    pub fn last_updated_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_date_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time that the Region description was last updated.</p>
    pub fn set_last_updated_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_updated_date_time = input;
        self
    }
    /// <p>The date and time that the Region description was last updated.</p>
    pub fn get_last_updated_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_updated_date_time
    }
    /// Consumes the builder and constructs a [`RegionDescription`](crate::types::RegionDescription).
    pub fn build(self) -> crate::types::RegionDescription {
        crate::types::RegionDescription {
            directory_id: self.directory_id,
            region_name: self.region_name,
            region_type: self.region_type,
            status: self.status,
            vpc_settings: self.vpc_settings,
            desired_number_of_domain_controllers: self.desired_number_of_domain_controllers,
            launch_time: self.launch_time,
            status_last_updated_date_time: self.status_last_updated_date_time,
            last_updated_date_time: self.last_updated_date_time,
        }
    }
}
