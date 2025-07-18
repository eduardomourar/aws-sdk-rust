// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The tier that your guardrail uses for content filters.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct GuardrailContentFiltersTier {
    /// <p>The tier that your guardrail uses for content filters. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>CLASSIC</code> tier – Provides established guardrails functionality supporting English, French, and Spanish languages.</p></li>
    /// <li>
    /// <p><code>STANDARD</code> tier – Provides a more robust solution than the <code>CLASSIC</code> tier and has more comprehensive language support. This tier requires that your guardrail use <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-cross-region.html">cross-Region inference</a>.</p></li>
    /// </ul>
    pub tier_name: crate::types::GuardrailContentFiltersTierName,
}
impl GuardrailContentFiltersTier {
    /// <p>The tier that your guardrail uses for content filters. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>CLASSIC</code> tier – Provides established guardrails functionality supporting English, French, and Spanish languages.</p></li>
    /// <li>
    /// <p><code>STANDARD</code> tier – Provides a more robust solution than the <code>CLASSIC</code> tier and has more comprehensive language support. This tier requires that your guardrail use <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-cross-region.html">cross-Region inference</a>.</p></li>
    /// </ul>
    pub fn tier_name(&self) -> &crate::types::GuardrailContentFiltersTierName {
        &self.tier_name
    }
}
impl ::std::fmt::Debug for GuardrailContentFiltersTier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GuardrailContentFiltersTier");
        formatter.field("tier_name", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl GuardrailContentFiltersTier {
    /// Creates a new builder-style object to manufacture [`GuardrailContentFiltersTier`](crate::types::GuardrailContentFiltersTier).
    pub fn builder() -> crate::types::builders::GuardrailContentFiltersTierBuilder {
        crate::types::builders::GuardrailContentFiltersTierBuilder::default()
    }
}

/// A builder for [`GuardrailContentFiltersTier`](crate::types::GuardrailContentFiltersTier).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct GuardrailContentFiltersTierBuilder {
    pub(crate) tier_name: ::std::option::Option<crate::types::GuardrailContentFiltersTierName>,
}
impl GuardrailContentFiltersTierBuilder {
    /// <p>The tier that your guardrail uses for content filters. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>CLASSIC</code> tier – Provides established guardrails functionality supporting English, French, and Spanish languages.</p></li>
    /// <li>
    /// <p><code>STANDARD</code> tier – Provides a more robust solution than the <code>CLASSIC</code> tier and has more comprehensive language support. This tier requires that your guardrail use <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-cross-region.html">cross-Region inference</a>.</p></li>
    /// </ul>
    /// This field is required.
    pub fn tier_name(mut self, input: crate::types::GuardrailContentFiltersTierName) -> Self {
        self.tier_name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The tier that your guardrail uses for content filters. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>CLASSIC</code> tier – Provides established guardrails functionality supporting English, French, and Spanish languages.</p></li>
    /// <li>
    /// <p><code>STANDARD</code> tier – Provides a more robust solution than the <code>CLASSIC</code> tier and has more comprehensive language support. This tier requires that your guardrail use <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-cross-region.html">cross-Region inference</a>.</p></li>
    /// </ul>
    pub fn set_tier_name(mut self, input: ::std::option::Option<crate::types::GuardrailContentFiltersTierName>) -> Self {
        self.tier_name = input;
        self
    }
    /// <p>The tier that your guardrail uses for content filters. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>CLASSIC</code> tier – Provides established guardrails functionality supporting English, French, and Spanish languages.</p></li>
    /// <li>
    /// <p><code>STANDARD</code> tier – Provides a more robust solution than the <code>CLASSIC</code> tier and has more comprehensive language support. This tier requires that your guardrail use <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-cross-region.html">cross-Region inference</a>.</p></li>
    /// </ul>
    pub fn get_tier_name(&self) -> &::std::option::Option<crate::types::GuardrailContentFiltersTierName> {
        &self.tier_name
    }
    /// Consumes the builder and constructs a [`GuardrailContentFiltersTier`](crate::types::GuardrailContentFiltersTier).
    /// This method will fail if any of the following fields are not set:
    /// - [`tier_name`](crate::types::builders::GuardrailContentFiltersTierBuilder::tier_name)
    pub fn build(self) -> ::std::result::Result<crate::types::GuardrailContentFiltersTier, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::GuardrailContentFiltersTier {
            tier_name: self.tier_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "tier_name",
                    "tier_name was not specified but it is required when building GuardrailContentFiltersTier",
                )
            })?,
        })
    }
}
impl ::std::fmt::Debug for GuardrailContentFiltersTierBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GuardrailContentFiltersTierBuilder");
        formatter.field("tier_name", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
