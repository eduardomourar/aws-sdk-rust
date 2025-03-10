// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterMemberToAddressList`](crate::operation::register_member_to_address_list::builders::RegisterMemberToAddressListFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`address_list_id(impl Into<String>)`](crate::operation::register_member_to_address_list::builders::RegisterMemberToAddressListFluentBuilder::address_list_id) / [`set_address_list_id(Option<String>)`](crate::operation::register_member_to_address_list::builders::RegisterMemberToAddressListFluentBuilder::set_address_list_id):<br>required: **true**<br><p>The unique identifier of the address list where the address should be added.</p><br>
    ///   - [`address(impl Into<String>)`](crate::operation::register_member_to_address_list::builders::RegisterMemberToAddressListFluentBuilder::address) / [`set_address(Option<String>)`](crate::operation::register_member_to_address_list::builders::RegisterMemberToAddressListFluentBuilder::set_address):<br>required: **true**<br><p>The address to be added to the address list.</p><br>
    /// - On success, responds with [`RegisterMemberToAddressListOutput`](crate::operation::register_member_to_address_list::RegisterMemberToAddressListOutput)
    /// - On failure, responds with [`SdkError<RegisterMemberToAddressListError>`](crate::operation::register_member_to_address_list::RegisterMemberToAddressListError)
    pub fn register_member_to_address_list(
        &self,
    ) -> crate::operation::register_member_to_address_list::builders::RegisterMemberToAddressListFluentBuilder {
        crate::operation::register_member_to_address_list::builders::RegisterMemberToAddressListFluentBuilder::new(self.handle.clone())
    }
}
