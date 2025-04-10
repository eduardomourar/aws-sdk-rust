// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeInput`](crate::operation::describe_input::builders::DescribeInputFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`input_id(impl Into<String>)`](crate::operation::describe_input::builders::DescribeInputFluentBuilder::input_id) / [`set_input_id(Option<String>)`](crate::operation::describe_input::builders::DescribeInputFluentBuilder::set_input_id):<br>required: **true**<br>Unique ID of the input<br>
    /// - On success, responds with [`DescribeInputOutput`](crate::operation::describe_input::DescribeInputOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::describe_input::DescribeInputOutput::arn): The Unique ARN of the input (generated, immutable).
    ///   - [`attached_channels(Option<Vec::<String>>)`](crate::operation::describe_input::DescribeInputOutput::attached_channels): A list of channel IDs that that input is attached to (currently an input can only be attached to one channel).
    ///   - [`destinations(Option<Vec::<InputDestination>>)`](crate::operation::describe_input::DescribeInputOutput::destinations): A list of the destinations of the input (PUSH-type).
    ///   - [`id(Option<String>)`](crate::operation::describe_input::DescribeInputOutput::id): The generated ID of the input (unique for user account, immutable).
    ///   - [`input_class(Option<InputClass>)`](crate::operation::describe_input::DescribeInputOutput::input_class): STANDARD - MediaLive expects two sources to be connected to this input. If the channel is also STANDARD, both sources will be ingested. If the channel is SINGLE_PIPELINE, only the first source will be ingested; the second source will always be ignored, even if the first source fails. SINGLE_PIPELINE - You can connect only one source to this input. If the ChannelClass is also SINGLE_PIPELINE, this value is valid. If the ChannelClass is STANDARD, this value is not valid because the channel requires two sources in the input.
    ///   - [`input_devices(Option<Vec::<InputDeviceSettings>>)`](crate::operation::describe_input::DescribeInputOutput::input_devices): Settings for the input devices.
    ///   - [`input_partner_ids(Option<Vec::<String>>)`](crate::operation::describe_input::DescribeInputOutput::input_partner_ids): A list of IDs for all Inputs which are partners of this one.
    ///   - [`input_source_type(Option<InputSourceType>)`](crate::operation::describe_input::DescribeInputOutput::input_source_type): Certain pull input sources can be dynamic, meaning that they can have their URL's dynamically changes during input switch actions. Presently, this functionality only works with MP4_FILE and TS_FILE inputs.
    ///   - [`media_connect_flows(Option<Vec::<MediaConnectFlow>>)`](crate::operation::describe_input::DescribeInputOutput::media_connect_flows): A list of MediaConnect Flows for this input.
    ///   - [`name(Option<String>)`](crate::operation::describe_input::DescribeInputOutput::name): The user-assigned name (This is a mutable value).
    ///   - [`role_arn(Option<String>)`](crate::operation::describe_input::DescribeInputOutput::role_arn): The Amazon Resource Name (ARN) of the role this input assumes during and after creation.
    ///   - [`security_groups(Option<Vec::<String>>)`](crate::operation::describe_input::DescribeInputOutput::security_groups): A list of IDs for all the Input Security Groups attached to the input.
    ///   - [`sources(Option<Vec::<InputSource>>)`](crate::operation::describe_input::DescribeInputOutput::sources): A list of the sources of the input (PULL-type).
    ///   - [`state(Option<InputState>)`](crate::operation::describe_input::DescribeInputOutput::state): Placeholder documentation for InputState
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::describe_input::DescribeInputOutput::tags): A collection of key-value pairs.
    ///   - [`r#type(Option<InputType>)`](crate::operation::describe_input::DescribeInputOutput::type): The different types of inputs that AWS Elemental MediaLive supports.
    ///   - [`srt_settings(Option<SrtSettings>)`](crate::operation::describe_input::DescribeInputOutput::srt_settings): The settings associated with an SRT input.
    ///   - [`input_network_location(Option<InputNetworkLocation>)`](crate::operation::describe_input::DescribeInputOutput::input_network_location): The location of this input. AWS, for an input existing in the AWS Cloud, On-Prem for an input in a customer network.
    ///   - [`multicast_settings(Option<MulticastSettings>)`](crate::operation::describe_input::DescribeInputOutput::multicast_settings): Multicast Input settings.
    ///   - [`smpte2110_receiver_group_settings(Option<Smpte2110ReceiverGroupSettings>)`](crate::operation::describe_input::DescribeInputOutput::smpte2110_receiver_group_settings): Include this parameter if the input is a SMPTE 2110 input, to identify the stream sources for this input.
    ///   - [`sdi_sources(Option<Vec::<String>>)`](crate::operation::describe_input::DescribeInputOutput::sdi_sources): SDI Sources for this Input.
    /// - On failure, responds with [`SdkError<DescribeInputError>`](crate::operation::describe_input::DescribeInputError)
    pub fn describe_input(&self) -> crate::operation::describe_input::builders::DescribeInputFluentBuilder {
        crate::operation::describe_input::builders::DescribeInputFluentBuilder::new(self.handle.clone())
    }
}
