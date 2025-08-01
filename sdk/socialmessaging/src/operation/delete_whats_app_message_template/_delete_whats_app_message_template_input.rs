// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteWhatsAppMessageTemplateInput {
    /// <p>The numeric ID of the template assigned by Meta.</p>
    pub meta_template_id: ::std::option::Option<::std::string::String>,
    /// <p>If true, deletes all language versions of the template.</p>
    pub delete_all_languages: ::std::option::Option<bool>,
    /// <p>The ID of the WhatsApp Business Account associated with this template.</p>
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the template to delete.</p>
    pub template_name: ::std::option::Option<::std::string::String>,
}
impl DeleteWhatsAppMessageTemplateInput {
    /// <p>The numeric ID of the template assigned by Meta.</p>
    pub fn meta_template_id(&self) -> ::std::option::Option<&str> {
        self.meta_template_id.as_deref()
    }
    /// <p>If true, deletes all language versions of the template.</p>
    pub fn delete_all_languages(&self) -> ::std::option::Option<bool> {
        self.delete_all_languages
    }
    /// <p>The ID of the WhatsApp Business Account associated with this template.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The name of the template to delete.</p>
    pub fn template_name(&self) -> ::std::option::Option<&str> {
        self.template_name.as_deref()
    }
}
impl DeleteWhatsAppMessageTemplateInput {
    /// Creates a new builder-style object to manufacture [`DeleteWhatsAppMessageTemplateInput`](crate::operation::delete_whats_app_message_template::DeleteWhatsAppMessageTemplateInput).
    pub fn builder() -> crate::operation::delete_whats_app_message_template::builders::DeleteWhatsAppMessageTemplateInputBuilder {
        crate::operation::delete_whats_app_message_template::builders::DeleteWhatsAppMessageTemplateInputBuilder::default()
    }
}

/// A builder for [`DeleteWhatsAppMessageTemplateInput`](crate::operation::delete_whats_app_message_template::DeleteWhatsAppMessageTemplateInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteWhatsAppMessageTemplateInputBuilder {
    pub(crate) meta_template_id: ::std::option::Option<::std::string::String>,
    pub(crate) delete_all_languages: ::std::option::Option<bool>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) template_name: ::std::option::Option<::std::string::String>,
}
impl DeleteWhatsAppMessageTemplateInputBuilder {
    /// <p>The numeric ID of the template assigned by Meta.</p>
    pub fn meta_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.meta_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The numeric ID of the template assigned by Meta.</p>
    pub fn set_meta_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.meta_template_id = input;
        self
    }
    /// <p>The numeric ID of the template assigned by Meta.</p>
    pub fn get_meta_template_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.meta_template_id
    }
    /// <p>If true, deletes all language versions of the template.</p>
    pub fn delete_all_languages(mut self, input: bool) -> Self {
        self.delete_all_languages = ::std::option::Option::Some(input);
        self
    }
    /// <p>If true, deletes all language versions of the template.</p>
    pub fn set_delete_all_languages(mut self, input: ::std::option::Option<bool>) -> Self {
        self.delete_all_languages = input;
        self
    }
    /// <p>If true, deletes all language versions of the template.</p>
    pub fn get_delete_all_languages(&self) -> &::std::option::Option<bool> {
        &self.delete_all_languages
    }
    /// <p>The ID of the WhatsApp Business Account associated with this template.</p>
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the WhatsApp Business Account associated with this template.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ID of the WhatsApp Business Account associated with this template.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// <p>The name of the template to delete.</p>
    /// This field is required.
    pub fn template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the template to delete.</p>
    pub fn set_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.template_name = input;
        self
    }
    /// <p>The name of the template to delete.</p>
    pub fn get_template_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.template_name
    }
    /// Consumes the builder and constructs a [`DeleteWhatsAppMessageTemplateInput`](crate::operation::delete_whats_app_message_template::DeleteWhatsAppMessageTemplateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_whats_app_message_template::DeleteWhatsAppMessageTemplateInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_whats_app_message_template::DeleteWhatsAppMessageTemplateInput {
            meta_template_id: self.meta_template_id,
            delete_all_languages: self.delete_all_languages,
            id: self.id,
            template_name: self.template_name,
        })
    }
}
