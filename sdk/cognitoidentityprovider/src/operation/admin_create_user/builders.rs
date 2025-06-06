// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::admin_create_user::_admin_create_user_output::AdminCreateUserOutputBuilder;

pub use crate::operation::admin_create_user::_admin_create_user_input::AdminCreateUserInputBuilder;

impl crate::operation::admin_create_user::builders::AdminCreateUserInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::admin_create_user::AdminCreateUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::admin_create_user::AdminCreateUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.admin_create_user();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AdminCreateUser`.
///
/// <p>Creates a new user in the specified user pool.</p>
/// <p>If <code>MessageAction</code> isn't set, the default is to send a welcome message via email or phone (SMS).</p>
/// <p>This message is based on a template that you configured in your call to create or update a user pool. This template includes your custom sign-up instructions and placeholders for user name and temporary password.</p>
/// <p>Alternatively, you can call <code>AdminCreateUser</code> with <code>SUPPRESS</code> for the <code>MessageAction</code> parameter, and Amazon Cognito won't send any email.</p>
/// <p>In either case, if the user has a password, they will be in the <code>FORCE_CHANGE_PASSWORD</code> state until they sign in and set their password. Your invitation message template must have the <code>{####}</code> password placeholder if your users have passwords. If your template doesn't have this placeholder, Amazon Cognito doesn't deliver the invitation message. In this case, you must update your message template and resend the password with a new <code>AdminCreateUser</code> request with a <code>MessageAction</code> value of <code>RESEND</code>.</p><note>
/// <p>This action might generate an SMS text message. Starting June 1, 2021, US telecom carriers require you to register an origination phone number before you can send SMS messages to US phone numbers. If you use SMS text messages in Amazon Cognito, you must register a phone number with <a href="https://console.aws.amazon.com/pinpoint/home/">Amazon Pinpoint</a>. Amazon Cognito uses the registered number automatically. Otherwise, Amazon Cognito users who must receive SMS messages might not be able to sign up, activate their accounts, or sign in.</p>
/// <p>If you have never used SMS text messages with Amazon Cognito or any other Amazon Web Services service, Amazon Simple Notification Service might place your account in the SMS sandbox. In <i> <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">sandbox mode</a> </i>, you can send messages only to verified phone numbers. After you test your app while in the sandbox environment, you can move out of the sandbox and into production. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-sms-settings.html"> SMS message settings for Amazon Cognito user pools</a> in the <i>Amazon Cognito Developer Guide</i>.</p>
/// </note> <note>
/// <p>Amazon Cognito evaluates Identity and Access Management (IAM) policies in requests for this API operation. For this operation, you must use IAM credentials to authorize requests, and you must grant yourself the corresponding IAM permission in a policy.</p>
/// <p class="title"><b>Learn more</b></p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-signing.html">Signing Amazon Web Services API Requests</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pools-API-operations.html">Using the Amazon Cognito user pools API and user pool endpoints</a></p></li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AdminCreateUserFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::admin_create_user::builders::AdminCreateUserInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::admin_create_user::AdminCreateUserOutput,
        crate::operation::admin_create_user::AdminCreateUserError,
    > for AdminCreateUserFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::admin_create_user::AdminCreateUserOutput,
            crate::operation::admin_create_user::AdminCreateUserError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AdminCreateUserFluentBuilder {
    /// Creates a new `AdminCreateUserFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AdminCreateUser as a reference.
    pub fn as_input(&self) -> &crate::operation::admin_create_user::builders::AdminCreateUserInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::admin_create_user::AdminCreateUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::admin_create_user::AdminCreateUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::admin_create_user::AdminCreateUser::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::admin_create_user::AdminCreateUser::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::admin_create_user::AdminCreateUserOutput,
        crate::operation::admin_create_user::AdminCreateUserError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ID of the user pool where you want to create a user.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The ID of the user pool where you want to create a user.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The ID of the user pool where you want to create a user.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_pool_id()
    }
    /// <p>The value that you want to set as the username sign-in attribute. The following conditions apply to the username parameter.</p>
    /// <ul>
    /// <li>
    /// <p>The username can't be a duplicate of another username in the same user pool.</p></li>
    /// <li>
    /// <p>You can't change the value of a username after you create it.</p></li>
    /// <li>
    /// <p>You can only provide a value if usernames are a valid sign-in attribute for your user pool. If your user pool only supports phone numbers or email addresses as sign-in attributes, Amazon Cognito automatically generates a username value. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html#user-pool-settings-aliases">Customizing sign-in attributes</a>.</p></li>
    /// </ul>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The value that you want to set as the username sign-in attribute. The following conditions apply to the username parameter.</p>
    /// <ul>
    /// <li>
    /// <p>The username can't be a duplicate of another username in the same user pool.</p></li>
    /// <li>
    /// <p>You can't change the value of a username after you create it.</p></li>
    /// <li>
    /// <p>You can only provide a value if usernames are a valid sign-in attribute for your user pool. If your user pool only supports phone numbers or email addresses as sign-in attributes, Amazon Cognito automatically generates a username value. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html#user-pool-settings-aliases">Customizing sign-in attributes</a>.</p></li>
    /// </ul>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
    /// <p>The value that you want to set as the username sign-in attribute. The following conditions apply to the username parameter.</p>
    /// <ul>
    /// <li>
    /// <p>The username can't be a duplicate of another username in the same user pool.</p></li>
    /// <li>
    /// <p>You can't change the value of a username after you create it.</p></li>
    /// <li>
    /// <p>You can only provide a value if usernames are a valid sign-in attribute for your user pool. If your user pool only supports phone numbers or email addresses as sign-in attributes, Amazon Cognito automatically generates a username value. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html#user-pool-settings-aliases">Customizing sign-in attributes</a>.</p></li>
    /// </ul>
    pub fn get_username(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_username()
    }
    ///
    /// Appends an item to `UserAttributes`.
    ///
    /// To override the contents of this collection use [`set_user_attributes`](Self::set_user_attributes).
    ///
    /// <p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (when creating a user pool or in the <b>Attributes</b> tab of the console) either you should supply (in your call to <code>AdminCreateUser</code>) or the user should supply (when they sign up in response to your welcome message).</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    /// <p>To send a message inviting the user to sign up, you must specify the user's email address or phone number. You can do this in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p>
    /// <p>You must also provide an email address or phone number when you expect the user to do passwordless sign-in with an email or SMS OTP. These attributes must be provided when passwordless options are the only available, or when you don't submit a <code>TemporaryPassword</code>.</p>
    /// <p>In your <code>AdminCreateUser</code> request, you can set the <code>email_verified</code> and <code>phone_number_verified</code> attributes to <code>true</code>. The following conditions apply:</p>
    /// <dl>
    /// <dt>
    /// email
    /// </dt>
    /// <dd>
    /// <p>The email address where you want the user to receive their confirmation code and username. You must provide a value for <code>email</code> when you want to set <code>email_verified</code> to <code>true</code>, or if you set <code>EMAIL</code> in the <code>DesiredDeliveryMediums</code> parameter.</p>
    /// </dd>
    /// <dt>
    /// phone_number
    /// </dt>
    /// <dd>
    /// <p>The phone number where you want the user to receive their confirmation code and username. You must provide a value for <code>phone_number</code> when you want to set <code>phone_number_verified</code> to <code>true</code>, or if you set <code>SMS</code> in the <code>DesiredDeliveryMediums</code> parameter.</p>
    /// </dd>
    /// </dl>
    pub fn user_attributes(mut self, input: crate::types::AttributeType) -> Self {
        self.inner = self.inner.user_attributes(input);
        self
    }
    /// <p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (when creating a user pool or in the <b>Attributes</b> tab of the console) either you should supply (in your call to <code>AdminCreateUser</code>) or the user should supply (when they sign up in response to your welcome message).</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    /// <p>To send a message inviting the user to sign up, you must specify the user's email address or phone number. You can do this in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p>
    /// <p>You must also provide an email address or phone number when you expect the user to do passwordless sign-in with an email or SMS OTP. These attributes must be provided when passwordless options are the only available, or when you don't submit a <code>TemporaryPassword</code>.</p>
    /// <p>In your <code>AdminCreateUser</code> request, you can set the <code>email_verified</code> and <code>phone_number_verified</code> attributes to <code>true</code>. The following conditions apply:</p>
    /// <dl>
    /// <dt>
    /// email
    /// </dt>
    /// <dd>
    /// <p>The email address where you want the user to receive their confirmation code and username. You must provide a value for <code>email</code> when you want to set <code>email_verified</code> to <code>true</code>, or if you set <code>EMAIL</code> in the <code>DesiredDeliveryMediums</code> parameter.</p>
    /// </dd>
    /// <dt>
    /// phone_number
    /// </dt>
    /// <dd>
    /// <p>The phone number where you want the user to receive their confirmation code and username. You must provide a value for <code>phone_number</code> when you want to set <code>phone_number_verified</code> to <code>true</code>, or if you set <code>SMS</code> in the <code>DesiredDeliveryMediums</code> parameter.</p>
    /// </dd>
    /// </dl>
    pub fn set_user_attributes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AttributeType>>) -> Self {
        self.inner = self.inner.set_user_attributes(input);
        self
    }
    /// <p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (when creating a user pool or in the <b>Attributes</b> tab of the console) either you should supply (in your call to <code>AdminCreateUser</code>) or the user should supply (when they sign up in response to your welcome message).</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    /// <p>To send a message inviting the user to sign up, you must specify the user's email address or phone number. You can do this in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p>
    /// <p>You must also provide an email address or phone number when you expect the user to do passwordless sign-in with an email or SMS OTP. These attributes must be provided when passwordless options are the only available, or when you don't submit a <code>TemporaryPassword</code>.</p>
    /// <p>In your <code>AdminCreateUser</code> request, you can set the <code>email_verified</code> and <code>phone_number_verified</code> attributes to <code>true</code>. The following conditions apply:</p>
    /// <dl>
    /// <dt>
    /// email
    /// </dt>
    /// <dd>
    /// <p>The email address where you want the user to receive their confirmation code and username. You must provide a value for <code>email</code> when you want to set <code>email_verified</code> to <code>true</code>, or if you set <code>EMAIL</code> in the <code>DesiredDeliveryMediums</code> parameter.</p>
    /// </dd>
    /// <dt>
    /// phone_number
    /// </dt>
    /// <dd>
    /// <p>The phone number where you want the user to receive their confirmation code and username. You must provide a value for <code>phone_number</code> when you want to set <code>phone_number_verified</code> to <code>true</code>, or if you set <code>SMS</code> in the <code>DesiredDeliveryMediums</code> parameter.</p>
    /// </dd>
    /// </dl>
    pub fn get_user_attributes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AttributeType>> {
        self.inner.get_user_attributes()
    }
    ///
    /// Appends an item to `ValidationData`.
    ///
    /// To override the contents of this collection use [`set_validation_data`](Self::set_validation_data).
    ///
    /// <p>Temporary user attributes that contribute to the outcomes of your pre sign-up Lambda trigger. This set of key-value pairs are for custom validation of information that you collect from your users but don't need to retain.</p>
    /// <p>Your Lambda function can analyze this additional data and act on it. Your function can automatically confirm and verify select users or perform external API operations like logging user attributes and validation data to Amazon CloudWatch Logs.</p>
    /// <p>For more information about the pre sign-up Lambda trigger, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-lambda-pre-sign-up.html">Pre sign-up Lambda trigger</a>.</p>
    pub fn validation_data(mut self, input: crate::types::AttributeType) -> Self {
        self.inner = self.inner.validation_data(input);
        self
    }
    /// <p>Temporary user attributes that contribute to the outcomes of your pre sign-up Lambda trigger. This set of key-value pairs are for custom validation of information that you collect from your users but don't need to retain.</p>
    /// <p>Your Lambda function can analyze this additional data and act on it. Your function can automatically confirm and verify select users or perform external API operations like logging user attributes and validation data to Amazon CloudWatch Logs.</p>
    /// <p>For more information about the pre sign-up Lambda trigger, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-lambda-pre-sign-up.html">Pre sign-up Lambda trigger</a>.</p>
    pub fn set_validation_data(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AttributeType>>) -> Self {
        self.inner = self.inner.set_validation_data(input);
        self
    }
    /// <p>Temporary user attributes that contribute to the outcomes of your pre sign-up Lambda trigger. This set of key-value pairs are for custom validation of information that you collect from your users but don't need to retain.</p>
    /// <p>Your Lambda function can analyze this additional data and act on it. Your function can automatically confirm and verify select users or perform external API operations like logging user attributes and validation data to Amazon CloudWatch Logs.</p>
    /// <p>For more information about the pre sign-up Lambda trigger, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-lambda-pre-sign-up.html">Pre sign-up Lambda trigger</a>.</p>
    pub fn get_validation_data(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AttributeType>> {
        self.inner.get_validation_data()
    }
    /// <p>The user's temporary password. This password must conform to the password policy that you specified when you created the user pool.</p>
    /// <p>The exception to the requirement for a password is when your user pool supports passwordless sign-in with email or SMS OTPs. To create a user with no password, omit this parameter or submit a blank value. You can only create a passwordless user when passwordless sign-in is available.</p>
    /// <p>The temporary password is valid only once. To complete the Admin Create User flow, the user must enter the temporary password in the sign-in page, along with a new password to be used in all future sign-ins.</p>
    /// <p>If you don't specify a value, Amazon Cognito generates one for you unless you have passwordless options active for your user pool.</p>
    /// <p>The temporary password can only be used until the user account expiration limit that you set for your user pool. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again and specify <code>RESEND</code> for the <code>MessageAction</code> parameter.</p>
    pub fn temporary_password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.temporary_password(input.into());
        self
    }
    /// <p>The user's temporary password. This password must conform to the password policy that you specified when you created the user pool.</p>
    /// <p>The exception to the requirement for a password is when your user pool supports passwordless sign-in with email or SMS OTPs. To create a user with no password, omit this parameter or submit a blank value. You can only create a passwordless user when passwordless sign-in is available.</p>
    /// <p>The temporary password is valid only once. To complete the Admin Create User flow, the user must enter the temporary password in the sign-in page, along with a new password to be used in all future sign-ins.</p>
    /// <p>If you don't specify a value, Amazon Cognito generates one for you unless you have passwordless options active for your user pool.</p>
    /// <p>The temporary password can only be used until the user account expiration limit that you set for your user pool. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again and specify <code>RESEND</code> for the <code>MessageAction</code> parameter.</p>
    pub fn set_temporary_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_temporary_password(input);
        self
    }
    /// <p>The user's temporary password. This password must conform to the password policy that you specified when you created the user pool.</p>
    /// <p>The exception to the requirement for a password is when your user pool supports passwordless sign-in with email or SMS OTPs. To create a user with no password, omit this parameter or submit a blank value. You can only create a passwordless user when passwordless sign-in is available.</p>
    /// <p>The temporary password is valid only once. To complete the Admin Create User flow, the user must enter the temporary password in the sign-in page, along with a new password to be used in all future sign-ins.</p>
    /// <p>If you don't specify a value, Amazon Cognito generates one for you unless you have passwordless options active for your user pool.</p>
    /// <p>The temporary password can only be used until the user account expiration limit that you set for your user pool. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again and specify <code>RESEND</code> for the <code>MessageAction</code> parameter.</p>
    pub fn get_temporary_password(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_temporary_password()
    }
    /// <p>This parameter is used only if the <code>phone_number_verified</code> or <code>email_verified</code> attribute is set to <code>True</code>. Otherwise, it is ignored.</p>
    /// <p>If this parameter is set to <code>True</code> and the phone number or email address specified in the <code>UserAttributes</code> parameter already exists as an alias with a different user, this request migrates the alias from the previous user to the newly-created user. The previous user will no longer be able to log in using that alias.</p>
    /// <p>If this parameter is set to <code>False</code>, the API throws an <code>AliasExistsException</code> error if the alias already exists. The default value is <code>False</code>.</p>
    pub fn force_alias_creation(mut self, input: bool) -> Self {
        self.inner = self.inner.force_alias_creation(input);
        self
    }
    /// <p>This parameter is used only if the <code>phone_number_verified</code> or <code>email_verified</code> attribute is set to <code>True</code>. Otherwise, it is ignored.</p>
    /// <p>If this parameter is set to <code>True</code> and the phone number or email address specified in the <code>UserAttributes</code> parameter already exists as an alias with a different user, this request migrates the alias from the previous user to the newly-created user. The previous user will no longer be able to log in using that alias.</p>
    /// <p>If this parameter is set to <code>False</code>, the API throws an <code>AliasExistsException</code> error if the alias already exists. The default value is <code>False</code>.</p>
    pub fn set_force_alias_creation(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force_alias_creation(input);
        self
    }
    /// <p>This parameter is used only if the <code>phone_number_verified</code> or <code>email_verified</code> attribute is set to <code>True</code>. Otherwise, it is ignored.</p>
    /// <p>If this parameter is set to <code>True</code> and the phone number or email address specified in the <code>UserAttributes</code> parameter already exists as an alias with a different user, this request migrates the alias from the previous user to the newly-created user. The previous user will no longer be able to log in using that alias.</p>
    /// <p>If this parameter is set to <code>False</code>, the API throws an <code>AliasExistsException</code> error if the alias already exists. The default value is <code>False</code>.</p>
    pub fn get_force_alias_creation(&self) -> &::std::option::Option<bool> {
        self.inner.get_force_alias_creation()
    }
    /// <p>Set to <code>RESEND</code> to resend the invitation message to a user that already exists, and to reset the temporary-password duration with a new temporary password. Set to <code>SUPPRESS</code> to suppress sending the message. You can specify only one value.</p>
    pub fn message_action(mut self, input: crate::types::MessageActionType) -> Self {
        self.inner = self.inner.message_action(input);
        self
    }
    /// <p>Set to <code>RESEND</code> to resend the invitation message to a user that already exists, and to reset the temporary-password duration with a new temporary password. Set to <code>SUPPRESS</code> to suppress sending the message. You can specify only one value.</p>
    pub fn set_message_action(mut self, input: ::std::option::Option<crate::types::MessageActionType>) -> Self {
        self.inner = self.inner.set_message_action(input);
        self
    }
    /// <p>Set to <code>RESEND</code> to resend the invitation message to a user that already exists, and to reset the temporary-password duration with a new temporary password. Set to <code>SUPPRESS</code> to suppress sending the message. You can specify only one value.</p>
    pub fn get_message_action(&self) -> &::std::option::Option<crate::types::MessageActionType> {
        self.inner.get_message_action()
    }
    ///
    /// Appends an item to `DesiredDeliveryMediums`.
    ///
    /// To override the contents of this collection use [`set_desired_delivery_mediums`](Self::set_desired_delivery_mediums).
    ///
    /// <p>Specify <code>EMAIL</code> if email will be used to send the welcome message. Specify <code>SMS</code> if the phone number will be used. The default value is <code>SMS</code>. You can specify more than one value.</p>
    pub fn desired_delivery_mediums(mut self, input: crate::types::DeliveryMediumType) -> Self {
        self.inner = self.inner.desired_delivery_mediums(input);
        self
    }
    /// <p>Specify <code>EMAIL</code> if email will be used to send the welcome message. Specify <code>SMS</code> if the phone number will be used. The default value is <code>SMS</code>. You can specify more than one value.</p>
    pub fn set_desired_delivery_mediums(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DeliveryMediumType>>) -> Self {
        self.inner = self.inner.set_desired_delivery_mediums(input);
        self
    }
    /// <p>Specify <code>EMAIL</code> if email will be used to send the welcome message. Specify <code>SMS</code> if the phone number will be used. The default value is <code>SMS</code>. You can specify more than one value.</p>
    pub fn get_desired_delivery_mediums(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DeliveryMediumType>> {
        self.inner.get_desired_delivery_mediums()
    }
    ///
    /// Adds a key-value pair to `ClientMetadata`.
    ///
    /// To override the contents of this collection use [`set_client_metadata`](Self::set_client_metadata).
    ///
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the AdminCreateUser API action, Amazon Cognito invokes the function that is assigned to the <i>pre sign-up</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>ClientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminCreateUser request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Using Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>
    /// <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>
    /// <li>
    /// <p>Validate the <code>ClientMetadata</code> value.</p></li>
    /// <li>
    /// <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>
    /// </ul>
    /// </note>
    pub fn client_metadata(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_metadata(k.into(), v.into());
        self
    }
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the AdminCreateUser API action, Amazon Cognito invokes the function that is assigned to the <i>pre sign-up</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>ClientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminCreateUser request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Using Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>
    /// <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>
    /// <li>
    /// <p>Validate the <code>ClientMetadata</code> value.</p></li>
    /// <li>
    /// <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>
    /// </ul>
    /// </note>
    pub fn set_client_metadata(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_client_metadata(input);
        self
    }
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the AdminCreateUser API action, Amazon Cognito invokes the function that is assigned to the <i>pre sign-up</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>ClientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminCreateUser request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Using Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>
    /// <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>
    /// <li>
    /// <p>Validate the <code>ClientMetadata</code> value.</p></li>
    /// <li>
    /// <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>
    /// </ul>
    /// </note>
    pub fn get_client_metadata(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_client_metadata()
    }
}
