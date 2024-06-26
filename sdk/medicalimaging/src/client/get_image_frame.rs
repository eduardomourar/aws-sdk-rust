// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetImageFrame`](crate::operation::get_image_frame::builders::GetImageFrameFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`datastore_id(impl Into<String>)`](crate::operation::get_image_frame::builders::GetImageFrameFluentBuilder::datastore_id) / [`set_datastore_id(Option<String>)`](crate::operation::get_image_frame::builders::GetImageFrameFluentBuilder::set_datastore_id):<br>required: **true**<br><p>The data store identifier.</p><br>
    ///   - [`image_set_id(impl Into<String>)`](crate::operation::get_image_frame::builders::GetImageFrameFluentBuilder::image_set_id) / [`set_image_set_id(Option<String>)`](crate::operation::get_image_frame::builders::GetImageFrameFluentBuilder::set_image_set_id):<br>required: **true**<br><p>The image set identifier.</p><br>
    ///   - [`image_frame_information(ImageFrameInformation)`](crate::operation::get_image_frame::builders::GetImageFrameFluentBuilder::image_frame_information) / [`set_image_frame_information(Option<ImageFrameInformation>)`](crate::operation::get_image_frame::builders::GetImageFrameFluentBuilder::set_image_frame_information):<br>required: **true**<br><p>Information about the image frame (pixel data) identifier.</p><br>
    /// - On success, responds with [`GetImageFrameOutput`](crate::operation::get_image_frame::GetImageFrameOutput) with field(s):
    ///   - [`image_frame_blob(ByteStream)`](crate::operation::get_image_frame::GetImageFrameOutput::image_frame_blob): <p>The blob containing the aggregated image frame information.</p>
    ///   - [`content_type(Option<String>)`](crate::operation::get_image_frame::GetImageFrameOutput::content_type): <p>The format in which the image frame information is returned to the customer. Default is <code>application/octet-stream</code>.</p>
    /// - On failure, responds with [`SdkError<GetImageFrameError>`](crate::operation::get_image_frame::GetImageFrameError)
    pub fn get_image_frame(&self) -> crate::operation::get_image_frame::builders::GetImageFrameFluentBuilder {
        crate::operation::get_image_frame::builders::GetImageFrameFluentBuilder::new(self.handle.clone())
    }
}
