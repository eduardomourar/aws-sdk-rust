// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeProjectOutput {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub project_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the project.</p>
    pub project_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the project.</p>
    pub project_id: ::std::option::Option<::std::string::String>,
    /// <p>The description of the project.</p>
    pub project_description: ::std::option::Option<::std::string::String>,
    /// <p>Information used to provision a service catalog product. For information, see <a href="https://docs.aws.amazon.com/servicecatalog/latest/adminguide/introduction.html">What is Amazon Web Services Service Catalog</a>.</p>
    pub service_catalog_provisioning_details: ::std::option::Option<crate::types::ServiceCatalogProvisioningDetails>,
    /// <p>Information about a provisioned service catalog product.</p>
    pub service_catalog_provisioned_product_details: ::std::option::Option<crate::types::ServiceCatalogProvisionedProductDetails>,
    /// <p>The status of the project.</p>
    pub project_status: ::std::option::Option<crate::types::ProjectStatus>,
    /// <p>An array of template providers associated with the project.</p>
    pub template_provider_details: ::std::option::Option<::std::vec::Vec<crate::types::TemplateProviderDetail>>,
    /// <p>Information about the user who created or modified a SageMaker resource.</p>
    pub created_by: ::std::option::Option<crate::types::UserContext>,
    /// <p>The time when the project was created.</p>
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The timestamp when project was last modified.</p>
    pub last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Information about the user who created or modified a SageMaker resource.</p>
    pub last_modified_by: ::std::option::Option<crate::types::UserContext>,
    _request_id: Option<String>,
}
impl DescribeProjectOutput {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn project_arn(&self) -> ::std::option::Option<&str> {
        self.project_arn.as_deref()
    }
    /// <p>The name of the project.</p>
    pub fn project_name(&self) -> ::std::option::Option<&str> {
        self.project_name.as_deref()
    }
    /// <p>The ID of the project.</p>
    pub fn project_id(&self) -> ::std::option::Option<&str> {
        self.project_id.as_deref()
    }
    /// <p>The description of the project.</p>
    pub fn project_description(&self) -> ::std::option::Option<&str> {
        self.project_description.as_deref()
    }
    /// <p>Information used to provision a service catalog product. For information, see <a href="https://docs.aws.amazon.com/servicecatalog/latest/adminguide/introduction.html">What is Amazon Web Services Service Catalog</a>.</p>
    pub fn service_catalog_provisioning_details(&self) -> ::std::option::Option<&crate::types::ServiceCatalogProvisioningDetails> {
        self.service_catalog_provisioning_details.as_ref()
    }
    /// <p>Information about a provisioned service catalog product.</p>
    pub fn service_catalog_provisioned_product_details(&self) -> ::std::option::Option<&crate::types::ServiceCatalogProvisionedProductDetails> {
        self.service_catalog_provisioned_product_details.as_ref()
    }
    /// <p>The status of the project.</p>
    pub fn project_status(&self) -> ::std::option::Option<&crate::types::ProjectStatus> {
        self.project_status.as_ref()
    }
    /// <p>An array of template providers associated with the project.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.template_provider_details.is_none()`.
    pub fn template_provider_details(&self) -> &[crate::types::TemplateProviderDetail] {
        self.template_provider_details.as_deref().unwrap_or_default()
    }
    /// <p>Information about the user who created or modified a SageMaker resource.</p>
    pub fn created_by(&self) -> ::std::option::Option<&crate::types::UserContext> {
        self.created_by.as_ref()
    }
    /// <p>The time when the project was created.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The timestamp when project was last modified.</p>
    pub fn last_modified_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified_time.as_ref()
    }
    /// <p>Information about the user who created or modified a SageMaker resource.</p>
    pub fn last_modified_by(&self) -> ::std::option::Option<&crate::types::UserContext> {
        self.last_modified_by.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeProjectOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeProjectOutput {
    /// Creates a new builder-style object to manufacture [`DescribeProjectOutput`](crate::operation::describe_project::DescribeProjectOutput).
    pub fn builder() -> crate::operation::describe_project::builders::DescribeProjectOutputBuilder {
        crate::operation::describe_project::builders::DescribeProjectOutputBuilder::default()
    }
}

/// A builder for [`DescribeProjectOutput`](crate::operation::describe_project::DescribeProjectOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeProjectOutputBuilder {
    pub(crate) project_arn: ::std::option::Option<::std::string::String>,
    pub(crate) project_name: ::std::option::Option<::std::string::String>,
    pub(crate) project_id: ::std::option::Option<::std::string::String>,
    pub(crate) project_description: ::std::option::Option<::std::string::String>,
    pub(crate) service_catalog_provisioning_details: ::std::option::Option<crate::types::ServiceCatalogProvisioningDetails>,
    pub(crate) service_catalog_provisioned_product_details: ::std::option::Option<crate::types::ServiceCatalogProvisionedProductDetails>,
    pub(crate) project_status: ::std::option::Option<crate::types::ProjectStatus>,
    pub(crate) template_provider_details: ::std::option::Option<::std::vec::Vec<crate::types::TemplateProviderDetail>>,
    pub(crate) created_by: ::std::option::Option<crate::types::UserContext>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modified_by: ::std::option::Option<crate::types::UserContext>,
    _request_id: Option<String>,
}
impl DescribeProjectOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    /// This field is required.
    pub fn project_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn set_project_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn get_project_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.project_arn
    }
    /// <p>The name of the project.</p>
    /// This field is required.
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the project.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_name = input;
        self
    }
    /// <p>The name of the project.</p>
    pub fn get_project_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.project_name
    }
    /// <p>The ID of the project.</p>
    /// This field is required.
    pub fn project_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the project.</p>
    pub fn set_project_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_id = input;
        self
    }
    /// <p>The ID of the project.</p>
    pub fn get_project_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.project_id
    }
    /// <p>The description of the project.</p>
    pub fn project_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the project.</p>
    pub fn set_project_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_description = input;
        self
    }
    /// <p>The description of the project.</p>
    pub fn get_project_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.project_description
    }
    /// <p>Information used to provision a service catalog product. For information, see <a href="https://docs.aws.amazon.com/servicecatalog/latest/adminguide/introduction.html">What is Amazon Web Services Service Catalog</a>.</p>
    pub fn service_catalog_provisioning_details(mut self, input: crate::types::ServiceCatalogProvisioningDetails) -> Self {
        self.service_catalog_provisioning_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information used to provision a service catalog product. For information, see <a href="https://docs.aws.amazon.com/servicecatalog/latest/adminguide/introduction.html">What is Amazon Web Services Service Catalog</a>.</p>
    pub fn set_service_catalog_provisioning_details(mut self, input: ::std::option::Option<crate::types::ServiceCatalogProvisioningDetails>) -> Self {
        self.service_catalog_provisioning_details = input;
        self
    }
    /// <p>Information used to provision a service catalog product. For information, see <a href="https://docs.aws.amazon.com/servicecatalog/latest/adminguide/introduction.html">What is Amazon Web Services Service Catalog</a>.</p>
    pub fn get_service_catalog_provisioning_details(&self) -> &::std::option::Option<crate::types::ServiceCatalogProvisioningDetails> {
        &self.service_catalog_provisioning_details
    }
    /// <p>Information about a provisioned service catalog product.</p>
    pub fn service_catalog_provisioned_product_details(mut self, input: crate::types::ServiceCatalogProvisionedProductDetails) -> Self {
        self.service_catalog_provisioned_product_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about a provisioned service catalog product.</p>
    pub fn set_service_catalog_provisioned_product_details(
        mut self,
        input: ::std::option::Option<crate::types::ServiceCatalogProvisionedProductDetails>,
    ) -> Self {
        self.service_catalog_provisioned_product_details = input;
        self
    }
    /// <p>Information about a provisioned service catalog product.</p>
    pub fn get_service_catalog_provisioned_product_details(&self) -> &::std::option::Option<crate::types::ServiceCatalogProvisionedProductDetails> {
        &self.service_catalog_provisioned_product_details
    }
    /// <p>The status of the project.</p>
    /// This field is required.
    pub fn project_status(mut self, input: crate::types::ProjectStatus) -> Self {
        self.project_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the project.</p>
    pub fn set_project_status(mut self, input: ::std::option::Option<crate::types::ProjectStatus>) -> Self {
        self.project_status = input;
        self
    }
    /// <p>The status of the project.</p>
    pub fn get_project_status(&self) -> &::std::option::Option<crate::types::ProjectStatus> {
        &self.project_status
    }
    /// Appends an item to `template_provider_details`.
    ///
    /// To override the contents of this collection use [`set_template_provider_details`](Self::set_template_provider_details).
    ///
    /// <p>An array of template providers associated with the project.</p>
    pub fn template_provider_details(mut self, input: crate::types::TemplateProviderDetail) -> Self {
        let mut v = self.template_provider_details.unwrap_or_default();
        v.push(input);
        self.template_provider_details = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of template providers associated with the project.</p>
    pub fn set_template_provider_details(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TemplateProviderDetail>>) -> Self {
        self.template_provider_details = input;
        self
    }
    /// <p>An array of template providers associated with the project.</p>
    pub fn get_template_provider_details(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TemplateProviderDetail>> {
        &self.template_provider_details
    }
    /// <p>Information about the user who created or modified a SageMaker resource.</p>
    pub fn created_by(mut self, input: crate::types::UserContext) -> Self {
        self.created_by = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the user who created or modified a SageMaker resource.</p>
    pub fn set_created_by(mut self, input: ::std::option::Option<crate::types::UserContext>) -> Self {
        self.created_by = input;
        self
    }
    /// <p>Information about the user who created or modified a SageMaker resource.</p>
    pub fn get_created_by(&self) -> &::std::option::Option<crate::types::UserContext> {
        &self.created_by
    }
    /// <p>The time when the project was created.</p>
    /// This field is required.
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when the project was created.</p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The time when the project was created.</p>
    pub fn get_creation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_time
    }
    /// <p>The timestamp when project was last modified.</p>
    pub fn last_modified_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when project was last modified.</p>
    pub fn set_last_modified_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_modified_time = input;
        self
    }
    /// <p>The timestamp when project was last modified.</p>
    pub fn get_last_modified_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_modified_time
    }
    /// <p>Information about the user who created or modified a SageMaker resource.</p>
    pub fn last_modified_by(mut self, input: crate::types::UserContext) -> Self {
        self.last_modified_by = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the user who created or modified a SageMaker resource.</p>
    pub fn set_last_modified_by(mut self, input: ::std::option::Option<crate::types::UserContext>) -> Self {
        self.last_modified_by = input;
        self
    }
    /// <p>Information about the user who created or modified a SageMaker resource.</p>
    pub fn get_last_modified_by(&self) -> &::std::option::Option<crate::types::UserContext> {
        &self.last_modified_by
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeProjectOutput`](crate::operation::describe_project::DescribeProjectOutput).
    pub fn build(self) -> crate::operation::describe_project::DescribeProjectOutput {
        crate::operation::describe_project::DescribeProjectOutput {
            project_arn: self.project_arn,
            project_name: self.project_name,
            project_id: self.project_id,
            project_description: self.project_description,
            service_catalog_provisioning_details: self.service_catalog_provisioning_details,
            service_catalog_provisioned_product_details: self.service_catalog_provisioned_product_details,
            project_status: self.project_status,
            template_provider_details: self.template_provider_details,
            created_by: self.created_by,
            creation_time: self.creation_time,
            last_modified_time: self.last_modified_time,
            last_modified_by: self.last_modified_by,
            _request_id: self._request_id,
        }
    }
}
