// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddRegion`](crate::operation::add_region::builders::AddRegionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::operation::add_region::builders::AddRegionFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::add_region::builders::AddRegionFluentBuilder::set_directory_id):<br>required: **true**<br><p>The identifier of the directory to which you want to add Region replication.</p><br>
    ///   - [`region_name(impl Into<String>)`](crate::operation::add_region::builders::AddRegionFluentBuilder::region_name) / [`set_region_name(Option<String>)`](crate::operation::add_region::builders::AddRegionFluentBuilder::set_region_name):<br>required: **true**<br><p>The name of the Region where you want to add domain controllers for replication. For example, <code>us-east-1</code>.</p><br>
    ///   - [`vpc_settings(DirectoryVpcSettings)`](crate::operation::add_region::builders::AddRegionFluentBuilder::vpc_settings) / [`set_vpc_settings(Option<DirectoryVpcSettings>)`](crate::operation::add_region::builders::AddRegionFluentBuilder::set_vpc_settings):<br>required: **true**<br><p>Contains VPC information for the <code>CreateDirectory</code>, <code>CreateMicrosoftAD</code>, or <code>CreateHybridAD</code> operation.</p><br>
    /// - On success, responds with [`AddRegionOutput`](crate::operation::add_region::AddRegionOutput)
    /// - On failure, responds with [`SdkError<AddRegionError>`](crate::operation::add_region::AddRegionError)
    pub fn add_region(&self) -> crate::operation::add_region::builders::AddRegionFluentBuilder {
        crate::operation::add_region::builders::AddRegionFluentBuilder::new(self.handle.clone())
    }
}
