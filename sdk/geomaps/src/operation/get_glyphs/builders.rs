// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_glyphs::_get_glyphs_output::GetGlyphsOutputBuilder;

pub use crate::operation::get_glyphs::_get_glyphs_input::GetGlyphsInputBuilder;

impl crate::operation::get_glyphs::builders::GetGlyphsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_glyphs::GetGlyphsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_glyphs::GetGlyphsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_glyphs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetGlyphs`.
///
/// <p><code>GetGlyphs</code> returns the map's glyphs.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetGlyphsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_glyphs::builders::GetGlyphsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::get_glyphs::GetGlyphsOutput, crate::operation::get_glyphs::GetGlyphsError>
    for GetGlyphsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::get_glyphs::GetGlyphsOutput, crate::operation::get_glyphs::GetGlyphsError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetGlyphsFluentBuilder {
    /// Creates a new `GetGlyphsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetGlyphs as a reference.
    pub fn as_input(&self) -> &crate::operation::get_glyphs::builders::GetGlyphsInputBuilder {
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
        crate::operation::get_glyphs::GetGlyphsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_glyphs::GetGlyphsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_glyphs::GetGlyphs::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_glyphs::GetGlyphs::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_glyphs::GetGlyphsOutput,
        crate::operation::get_glyphs::GetGlyphsError,
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
    /// <p>Name of the <code>FontStack</code> to retrieve.</p>
    /// <p>Example: <code>Amazon Ember Bold,Noto Sans Bold</code>.</p>
    /// <p>The supported font stacks are as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Amazon Ember Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Bold Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Bold,Noto Sans Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Bold,Noto Sans Bold,Noto Sans Arabic Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC BdItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold,Noto Sans Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold,Noto Sans Bold,Noto Sans Arabic Condensed Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Light</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Light Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC LtItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular,Noto Sans Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular,Noto Sans Regular,Noto Sans Arabic Condensed Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC RgItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC ThItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Thin</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Thin Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Heavy</p></li>
    /// <li>
    /// <p>Amazon Ember Heavy Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Light</p></li>
    /// <li>
    /// <p>Amazon Ember Light Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Medium</p></li>
    /// <li>
    /// <p>Amazon Ember Medium Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Medium,Noto Sans Medium</p></li>
    /// <li>
    /// <p>Amazon Ember Medium,Noto Sans Medium,Noto Sans Arabic Medium</p></li>
    /// <li>
    /// <p>Amazon Ember Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Regular Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Regular Italic,Noto Sans Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Regular Italic,Noto Sans Italic,Noto Sans Arabic Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Regular,Noto Sans Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Regular,Noto Sans Regular,Noto Sans Arabic Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Thin</p></li>
    /// <li>
    /// <p>Amazon Ember Thin Italic</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Bd</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_BdIt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Lt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_LtIt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Rg</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_RgIt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Th</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_ThIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Bd</p></li>
    /// <li>
    /// <p>AmazonEmber_BdIt</p></li>
    /// <li>
    /// <p>AmazonEmber_He</p></li>
    /// <li>
    /// <p>AmazonEmber_HeIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Lt</p></li>
    /// <li>
    /// <p>AmazonEmber_LtIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Md</p></li>
    /// <li>
    /// <p>AmazonEmber_MdIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Rg</p></li>
    /// <li>
    /// <p>AmazonEmber_RgIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Th</p></li>
    /// <li>
    /// <p>AmazonEmber_ThIt</p></li>
    /// <li>
    /// <p>Noto Sans Black</p></li>
    /// <li>
    /// <p>Noto Sans Black Italic</p></li>
    /// <li>
    /// <p>Noto Sans Bold</p></li>
    /// <li>
    /// <p>Noto Sans Bold Italic</p></li>
    /// <li>
    /// <p>Noto Sans Extra Bold</p></li>
    /// <li>
    /// <p>Noto Sans Extra Bold Italic</p></li>
    /// <li>
    /// <p>Noto Sans Extra Light</p></li>
    /// <li>
    /// <p>Noto Sans Extra Light Italic</p></li>
    /// <li>
    /// <p>Noto Sans Italic</p></li>
    /// <li>
    /// <p>Noto Sans Light</p></li>
    /// <li>
    /// <p>Noto Sans Light Italic</p></li>
    /// <li>
    /// <p>Noto Sans Medium</p></li>
    /// <li>
    /// <p>Noto Sans Medium Italic</p></li>
    /// <li>
    /// <p>Noto Sans Regular</p></li>
    /// <li>
    /// <p>Noto Sans Semi Bold</p></li>
    /// <li>
    /// <p>Noto Sans Semi Bold Italic</p></li>
    /// <li>
    /// <p>Noto Sans Thin</p></li>
    /// <li>
    /// <p>Noto Sans Thin Italic</p></li>
    /// <li>
    /// <p>NotoSans-Bold</p></li>
    /// <li>
    /// <p>NotoSans-Italic</p></li>
    /// <li>
    /// <p>NotoSans-Medium</p></li>
    /// <li>
    /// <p>NotoSans-Regular</p></li>
    /// <li>
    /// <p>Open Sans Regular,Arial Unicode MS Regular</p></li>
    /// </ul>
    pub fn font_stack(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.font_stack(input.into());
        self
    }
    /// <p>Name of the <code>FontStack</code> to retrieve.</p>
    /// <p>Example: <code>Amazon Ember Bold,Noto Sans Bold</code>.</p>
    /// <p>The supported font stacks are as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Amazon Ember Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Bold Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Bold,Noto Sans Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Bold,Noto Sans Bold,Noto Sans Arabic Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC BdItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold,Noto Sans Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold,Noto Sans Bold,Noto Sans Arabic Condensed Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Light</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Light Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC LtItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular,Noto Sans Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular,Noto Sans Regular,Noto Sans Arabic Condensed Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC RgItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC ThItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Thin</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Thin Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Heavy</p></li>
    /// <li>
    /// <p>Amazon Ember Heavy Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Light</p></li>
    /// <li>
    /// <p>Amazon Ember Light Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Medium</p></li>
    /// <li>
    /// <p>Amazon Ember Medium Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Medium,Noto Sans Medium</p></li>
    /// <li>
    /// <p>Amazon Ember Medium,Noto Sans Medium,Noto Sans Arabic Medium</p></li>
    /// <li>
    /// <p>Amazon Ember Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Regular Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Regular Italic,Noto Sans Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Regular Italic,Noto Sans Italic,Noto Sans Arabic Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Regular,Noto Sans Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Regular,Noto Sans Regular,Noto Sans Arabic Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Thin</p></li>
    /// <li>
    /// <p>Amazon Ember Thin Italic</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Bd</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_BdIt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Lt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_LtIt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Rg</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_RgIt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Th</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_ThIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Bd</p></li>
    /// <li>
    /// <p>AmazonEmber_BdIt</p></li>
    /// <li>
    /// <p>AmazonEmber_He</p></li>
    /// <li>
    /// <p>AmazonEmber_HeIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Lt</p></li>
    /// <li>
    /// <p>AmazonEmber_LtIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Md</p></li>
    /// <li>
    /// <p>AmazonEmber_MdIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Rg</p></li>
    /// <li>
    /// <p>AmazonEmber_RgIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Th</p></li>
    /// <li>
    /// <p>AmazonEmber_ThIt</p></li>
    /// <li>
    /// <p>Noto Sans Black</p></li>
    /// <li>
    /// <p>Noto Sans Black Italic</p></li>
    /// <li>
    /// <p>Noto Sans Bold</p></li>
    /// <li>
    /// <p>Noto Sans Bold Italic</p></li>
    /// <li>
    /// <p>Noto Sans Extra Bold</p></li>
    /// <li>
    /// <p>Noto Sans Extra Bold Italic</p></li>
    /// <li>
    /// <p>Noto Sans Extra Light</p></li>
    /// <li>
    /// <p>Noto Sans Extra Light Italic</p></li>
    /// <li>
    /// <p>Noto Sans Italic</p></li>
    /// <li>
    /// <p>Noto Sans Light</p></li>
    /// <li>
    /// <p>Noto Sans Light Italic</p></li>
    /// <li>
    /// <p>Noto Sans Medium</p></li>
    /// <li>
    /// <p>Noto Sans Medium Italic</p></li>
    /// <li>
    /// <p>Noto Sans Regular</p></li>
    /// <li>
    /// <p>Noto Sans Semi Bold</p></li>
    /// <li>
    /// <p>Noto Sans Semi Bold Italic</p></li>
    /// <li>
    /// <p>Noto Sans Thin</p></li>
    /// <li>
    /// <p>Noto Sans Thin Italic</p></li>
    /// <li>
    /// <p>NotoSans-Bold</p></li>
    /// <li>
    /// <p>NotoSans-Italic</p></li>
    /// <li>
    /// <p>NotoSans-Medium</p></li>
    /// <li>
    /// <p>NotoSans-Regular</p></li>
    /// <li>
    /// <p>Open Sans Regular,Arial Unicode MS Regular</p></li>
    /// </ul>
    pub fn set_font_stack(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_font_stack(input);
        self
    }
    /// <p>Name of the <code>FontStack</code> to retrieve.</p>
    /// <p>Example: <code>Amazon Ember Bold,Noto Sans Bold</code>.</p>
    /// <p>The supported font stacks are as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Amazon Ember Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Bold Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Bold,Noto Sans Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Bold,Noto Sans Bold,Noto Sans Arabic Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC BdItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold,Noto Sans Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Bold,Noto Sans Bold,Noto Sans Arabic Condensed Bold</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Light</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Light Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC LtItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular,Noto Sans Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Regular,Noto Sans Regular,Noto Sans Arabic Condensed Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC RgItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC ThItalic</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Thin</p></li>
    /// <li>
    /// <p>Amazon Ember Condensed RC Thin Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Heavy</p></li>
    /// <li>
    /// <p>Amazon Ember Heavy Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Light</p></li>
    /// <li>
    /// <p>Amazon Ember Light Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Medium</p></li>
    /// <li>
    /// <p>Amazon Ember Medium Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Medium,Noto Sans Medium</p></li>
    /// <li>
    /// <p>Amazon Ember Medium,Noto Sans Medium,Noto Sans Arabic Medium</p></li>
    /// <li>
    /// <p>Amazon Ember Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Regular Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Regular Italic,Noto Sans Italic</p></li>
    /// <li>
    /// <p>Amazon Ember Regular Italic,Noto Sans Italic,Noto Sans Arabic Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Regular,Noto Sans Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Regular,Noto Sans Regular,Noto Sans Arabic Regular</p></li>
    /// <li>
    /// <p>Amazon Ember Thin</p></li>
    /// <li>
    /// <p>Amazon Ember Thin Italic</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Bd</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_BdIt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Lt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_LtIt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Rg</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_RgIt</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_Th</p></li>
    /// <li>
    /// <p>AmazonEmberCdRC_ThIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Bd</p></li>
    /// <li>
    /// <p>AmazonEmber_BdIt</p></li>
    /// <li>
    /// <p>AmazonEmber_He</p></li>
    /// <li>
    /// <p>AmazonEmber_HeIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Lt</p></li>
    /// <li>
    /// <p>AmazonEmber_LtIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Md</p></li>
    /// <li>
    /// <p>AmazonEmber_MdIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Rg</p></li>
    /// <li>
    /// <p>AmazonEmber_RgIt</p></li>
    /// <li>
    /// <p>AmazonEmber_Th</p></li>
    /// <li>
    /// <p>AmazonEmber_ThIt</p></li>
    /// <li>
    /// <p>Noto Sans Black</p></li>
    /// <li>
    /// <p>Noto Sans Black Italic</p></li>
    /// <li>
    /// <p>Noto Sans Bold</p></li>
    /// <li>
    /// <p>Noto Sans Bold Italic</p></li>
    /// <li>
    /// <p>Noto Sans Extra Bold</p></li>
    /// <li>
    /// <p>Noto Sans Extra Bold Italic</p></li>
    /// <li>
    /// <p>Noto Sans Extra Light</p></li>
    /// <li>
    /// <p>Noto Sans Extra Light Italic</p></li>
    /// <li>
    /// <p>Noto Sans Italic</p></li>
    /// <li>
    /// <p>Noto Sans Light</p></li>
    /// <li>
    /// <p>Noto Sans Light Italic</p></li>
    /// <li>
    /// <p>Noto Sans Medium</p></li>
    /// <li>
    /// <p>Noto Sans Medium Italic</p></li>
    /// <li>
    /// <p>Noto Sans Regular</p></li>
    /// <li>
    /// <p>Noto Sans Semi Bold</p></li>
    /// <li>
    /// <p>Noto Sans Semi Bold Italic</p></li>
    /// <li>
    /// <p>Noto Sans Thin</p></li>
    /// <li>
    /// <p>Noto Sans Thin Italic</p></li>
    /// <li>
    /// <p>NotoSans-Bold</p></li>
    /// <li>
    /// <p>NotoSans-Italic</p></li>
    /// <li>
    /// <p>NotoSans-Medium</p></li>
    /// <li>
    /// <p>NotoSans-Regular</p></li>
    /// <li>
    /// <p>Open Sans Regular,Arial Unicode MS Regular</p></li>
    /// </ul>
    pub fn get_font_stack(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_font_stack()
    }
    /// <p>A Unicode range of characters to download glyphs for. This must be aligned to multiples of 256.</p>
    /// <p>Example: <code>0-255.pdf</code></p>
    pub fn font_unicode_range(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.font_unicode_range(input.into());
        self
    }
    /// <p>A Unicode range of characters to download glyphs for. This must be aligned to multiples of 256.</p>
    /// <p>Example: <code>0-255.pdf</code></p>
    pub fn set_font_unicode_range(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_font_unicode_range(input);
        self
    }
    /// <p>A Unicode range of characters to download glyphs for. This must be aligned to multiples of 256.</p>
    /// <p>Example: <code>0-255.pdf</code></p>
    pub fn get_font_unicode_range(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_font_unicode_range()
    }
}
