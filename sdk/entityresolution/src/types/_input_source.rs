// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object containing <code>inputSourceARN</code>, <code>schemaName</code>, and <code>applyNormalization</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InputSource {
    /// <p>An Glue table Amazon Resource Name (ARN) for the input source table.</p>
    pub input_source_arn: ::std::string::String,
    /// <p>The name of the schema to be retrieved.</p>
    pub schema_name: ::std::string::String,
    /// <p>Normalizes the attributes defined in the schema in the input data. For example, if an attribute has an <code>AttributeType</code> of <code>PHONE_NUMBER</code>, and the data in the input table is in a format of 1234567890, Entity Resolution will normalize this field in the output to (123)-456-7890.</p>
    pub apply_normalization: ::std::option::Option<bool>,
}
impl InputSource {
    /// <p>An Glue table Amazon Resource Name (ARN) for the input source table.</p>
    pub fn input_source_arn(&self) -> &str {
        use std::ops::Deref;
        self.input_source_arn.deref()
    }
    /// <p>The name of the schema to be retrieved.</p>
    pub fn schema_name(&self) -> &str {
        use std::ops::Deref;
        self.schema_name.deref()
    }
    /// <p>Normalizes the attributes defined in the schema in the input data. For example, if an attribute has an <code>AttributeType</code> of <code>PHONE_NUMBER</code>, and the data in the input table is in a format of 1234567890, Entity Resolution will normalize this field in the output to (123)-456-7890.</p>
    pub fn apply_normalization(&self) -> ::std::option::Option<bool> {
        self.apply_normalization
    }
}
impl InputSource {
    /// Creates a new builder-style object to manufacture [`InputSource`](crate::types::InputSource).
    pub fn builder() -> crate::types::builders::InputSourceBuilder {
        crate::types::builders::InputSourceBuilder::default()
    }
}

/// A builder for [`InputSource`](crate::types::InputSource).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InputSourceBuilder {
    pub(crate) input_source_arn: ::std::option::Option<::std::string::String>,
    pub(crate) schema_name: ::std::option::Option<::std::string::String>,
    pub(crate) apply_normalization: ::std::option::Option<bool>,
}
impl InputSourceBuilder {
    /// <p>An Glue table Amazon Resource Name (ARN) for the input source table.</p>
    /// This field is required.
    pub fn input_source_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.input_source_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An Glue table Amazon Resource Name (ARN) for the input source table.</p>
    pub fn set_input_source_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.input_source_arn = input;
        self
    }
    /// <p>An Glue table Amazon Resource Name (ARN) for the input source table.</p>
    pub fn get_input_source_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.input_source_arn
    }
    /// <p>The name of the schema to be retrieved.</p>
    /// This field is required.
    pub fn schema_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.schema_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the schema to be retrieved.</p>
    pub fn set_schema_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.schema_name = input;
        self
    }
    /// <p>The name of the schema to be retrieved.</p>
    pub fn get_schema_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.schema_name
    }
    /// <p>Normalizes the attributes defined in the schema in the input data. For example, if an attribute has an <code>AttributeType</code> of <code>PHONE_NUMBER</code>, and the data in the input table is in a format of 1234567890, Entity Resolution will normalize this field in the output to (123)-456-7890.</p>
    pub fn apply_normalization(mut self, input: bool) -> Self {
        self.apply_normalization = ::std::option::Option::Some(input);
        self
    }
    /// <p>Normalizes the attributes defined in the schema in the input data. For example, if an attribute has an <code>AttributeType</code> of <code>PHONE_NUMBER</code>, and the data in the input table is in a format of 1234567890, Entity Resolution will normalize this field in the output to (123)-456-7890.</p>
    pub fn set_apply_normalization(mut self, input: ::std::option::Option<bool>) -> Self {
        self.apply_normalization = input;
        self
    }
    /// <p>Normalizes the attributes defined in the schema in the input data. For example, if an attribute has an <code>AttributeType</code> of <code>PHONE_NUMBER</code>, and the data in the input table is in a format of 1234567890, Entity Resolution will normalize this field in the output to (123)-456-7890.</p>
    pub fn get_apply_normalization(&self) -> &::std::option::Option<bool> {
        &self.apply_normalization
    }
    /// Consumes the builder and constructs a [`InputSource`](crate::types::InputSource).
    /// This method will fail if any of the following fields are not set:
    /// - [`input_source_arn`](crate::types::builders::InputSourceBuilder::input_source_arn)
    /// - [`schema_name`](crate::types::builders::InputSourceBuilder::schema_name)
    pub fn build(self) -> ::std::result::Result<crate::types::InputSource, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::InputSource {
            input_source_arn: self.input_source_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "input_source_arn",
                    "input_source_arn was not specified but it is required when building InputSource",
                )
            })?,
            schema_name: self.schema_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "schema_name",
                    "schema_name was not specified but it is required when building InputSource",
                )
            })?,
            apply_normalization: self.apply_normalization,
        })
    }
}
