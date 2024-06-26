// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`InvokeDeviceMethod`](crate::operation::invoke_device_method::builders::InvokeDeviceMethodFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`device_id(impl Into<String>)`](crate::operation::invoke_device_method::builders::InvokeDeviceMethodFluentBuilder::device_id) / [`set_device_id(Option<String>)`](crate::operation::invoke_device_method::builders::InvokeDeviceMethodFluentBuilder::set_device_id):<br>required: **true**<br><p>The unique identifier of the device.</p><br>
    ///   - [`device_method(DeviceMethod)`](crate::operation::invoke_device_method::builders::InvokeDeviceMethodFluentBuilder::device_method) / [`set_device_method(Option<DeviceMethod>)`](crate::operation::invoke_device_method::builders::InvokeDeviceMethodFluentBuilder::set_device_method):<br>required: **false**<br><p>The device method to invoke.</p><br>
    ///   - [`device_method_parameters(impl Into<String>)`](crate::operation::invoke_device_method::builders::InvokeDeviceMethodFluentBuilder::device_method_parameters) / [`set_device_method_parameters(Option<String>)`](crate::operation::invoke_device_method::builders::InvokeDeviceMethodFluentBuilder::set_device_method_parameters):<br>required: **false**<br><p>A JSON encoded string containing the device method request parameters.</p><br>
    /// - On success, responds with [`InvokeDeviceMethodOutput`](crate::operation::invoke_device_method::InvokeDeviceMethodOutput) with field(s):
    ///   - [`device_method_response(Option<String>)`](crate::operation::invoke_device_method::InvokeDeviceMethodOutput::device_method_response): <p>A JSON encoded string containing the device method response.</p>
    /// - On failure, responds with [`SdkError<InvokeDeviceMethodError>`](crate::operation::invoke_device_method::InvokeDeviceMethodError)
    pub fn invoke_device_method(&self) -> crate::operation::invoke_device_method::builders::InvokeDeviceMethodFluentBuilder {
        crate::operation::invoke_device_method::builders::InvokeDeviceMethodFluentBuilder::new(self.handle.clone())
    }
}
