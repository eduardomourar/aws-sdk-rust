// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configures user pool SMS messages for multi-factor authentication (MFA). Sets the message template and the SMS message sending configuration for Amazon SNS.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SmsMfaConfigType {
    /// <p>The SMS message that your user pool sends to users with an MFA code. The message must contain the <code>{####}</code> placeholder. In the message, Amazon Cognito replaces this placeholder with the code. If you don't provide this parameter, Amazon Cognito sends messages in the default format.</p>
    pub sms_authentication_message: ::std::option::Option<::std::string::String>,
    /// <p>The SMS configuration with the settings that your Amazon Cognito user pool must use to send an SMS message from your Amazon Web Services account through Amazon Simple Notification Service. To request Amazon SNS in the Amazon Web Services Region that you want, the Amazon Cognito user pool uses an Identity and Access Management (IAM) role that you provide for your Amazon Web Services account.</p>
    pub sms_configuration: ::std::option::Option<crate::types::SmsConfigurationType>,
}
impl SmsMfaConfigType {
    /// <p>The SMS message that your user pool sends to users with an MFA code. The message must contain the <code>{####}</code> placeholder. In the message, Amazon Cognito replaces this placeholder with the code. If you don't provide this parameter, Amazon Cognito sends messages in the default format.</p>
    pub fn sms_authentication_message(&self) -> ::std::option::Option<&str> {
        self.sms_authentication_message.as_deref()
    }
    /// <p>The SMS configuration with the settings that your Amazon Cognito user pool must use to send an SMS message from your Amazon Web Services account through Amazon Simple Notification Service. To request Amazon SNS in the Amazon Web Services Region that you want, the Amazon Cognito user pool uses an Identity and Access Management (IAM) role that you provide for your Amazon Web Services account.</p>
    pub fn sms_configuration(&self) -> ::std::option::Option<&crate::types::SmsConfigurationType> {
        self.sms_configuration.as_ref()
    }
}
impl SmsMfaConfigType {
    /// Creates a new builder-style object to manufacture [`SmsMfaConfigType`](crate::types::SmsMfaConfigType).
    pub fn builder() -> crate::types::builders::SmsMfaConfigTypeBuilder {
        crate::types::builders::SmsMfaConfigTypeBuilder::default()
    }
}

/// A builder for [`SmsMfaConfigType`](crate::types::SmsMfaConfigType).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SmsMfaConfigTypeBuilder {
    pub(crate) sms_authentication_message: ::std::option::Option<::std::string::String>,
    pub(crate) sms_configuration: ::std::option::Option<crate::types::SmsConfigurationType>,
}
impl SmsMfaConfigTypeBuilder {
    /// <p>The SMS message that your user pool sends to users with an MFA code. The message must contain the <code>{####}</code> placeholder. In the message, Amazon Cognito replaces this placeholder with the code. If you don't provide this parameter, Amazon Cognito sends messages in the default format.</p>
    pub fn sms_authentication_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sms_authentication_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The SMS message that your user pool sends to users with an MFA code. The message must contain the <code>{####}</code> placeholder. In the message, Amazon Cognito replaces this placeholder with the code. If you don't provide this parameter, Amazon Cognito sends messages in the default format.</p>
    pub fn set_sms_authentication_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sms_authentication_message = input;
        self
    }
    /// <p>The SMS message that your user pool sends to users with an MFA code. The message must contain the <code>{####}</code> placeholder. In the message, Amazon Cognito replaces this placeholder with the code. If you don't provide this parameter, Amazon Cognito sends messages in the default format.</p>
    pub fn get_sms_authentication_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.sms_authentication_message
    }
    /// <p>The SMS configuration with the settings that your Amazon Cognito user pool must use to send an SMS message from your Amazon Web Services account through Amazon Simple Notification Service. To request Amazon SNS in the Amazon Web Services Region that you want, the Amazon Cognito user pool uses an Identity and Access Management (IAM) role that you provide for your Amazon Web Services account.</p>
    pub fn sms_configuration(mut self, input: crate::types::SmsConfigurationType) -> Self {
        self.sms_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The SMS configuration with the settings that your Amazon Cognito user pool must use to send an SMS message from your Amazon Web Services account through Amazon Simple Notification Service. To request Amazon SNS in the Amazon Web Services Region that you want, the Amazon Cognito user pool uses an Identity and Access Management (IAM) role that you provide for your Amazon Web Services account.</p>
    pub fn set_sms_configuration(mut self, input: ::std::option::Option<crate::types::SmsConfigurationType>) -> Self {
        self.sms_configuration = input;
        self
    }
    /// <p>The SMS configuration with the settings that your Amazon Cognito user pool must use to send an SMS message from your Amazon Web Services account through Amazon Simple Notification Service. To request Amazon SNS in the Amazon Web Services Region that you want, the Amazon Cognito user pool uses an Identity and Access Management (IAM) role that you provide for your Amazon Web Services account.</p>
    pub fn get_sms_configuration(&self) -> &::std::option::Option<crate::types::SmsConfigurationType> {
        &self.sms_configuration
    }
    /// Consumes the builder and constructs a [`SmsMfaConfigType`](crate::types::SmsMfaConfigType).
    pub fn build(self) -> crate::types::SmsMfaConfigType {
        crate::types::SmsMfaConfigType {
            sms_authentication_message: self.sms_authentication_message,
            sms_configuration: self.sms_configuration,
        }
    }
}
