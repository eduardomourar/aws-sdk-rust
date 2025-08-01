// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies a target that writes to an Iceberg data source in Amazon S3.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3IcebergDirectTarget {
    /// <p>Specifies the unique identifier for the Iceberg target node in your data pipeline.</p>
    pub name: ::std::string::String,
    /// <p>Defines the single input source that provides data to this Iceberg target.</p>
    pub inputs: ::std::vec::Vec<::std::string::String>,
    /// <p>Specifies the columns used to partition the Iceberg table data in S3.</p>
    pub partition_keys: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
    /// <p>Defines the S3 location where the Iceberg table data will be stored.</p>
    pub path: ::std::string::String,
    /// <p>Specifies the file format used for storing Iceberg table data (e.g., Parquet, ORC).</p>
    pub format: crate::types::TargetFormat,
    /// <p>Provides additional configuration options for customizing the Iceberg table behavior.</p>
    pub additional_options: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>Defines how schema changes are handled when writing data to the Iceberg table.</p>
    pub schema_change_policy: ::std::option::Option<crate::types::DirectSchemaChangePolicy>,
    /// <p>Specifies configuration options for automatic data quality evaluation in Glue jobs. This structure enables automated data quality checks and monitoring during ETL operations, helping to ensure data integrity and reliability without manual intervention.</p>
    pub auto_data_quality: ::std::option::Option<crate::types::AutoDataQuality>,
    /// <p>Specifies the compression codec used for Iceberg table files in S3.</p>
    pub compression: crate::types::IcebergTargetCompressionType,
    /// <p>Sets the number of target partitions for distributing Iceberg table files across S3.</p>
    pub number_target_partitions: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the data schema for the S3 Iceberg direct target.</p>
    pub output_schemas: ::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>>,
}
impl S3IcebergDirectTarget {
    /// <p>Specifies the unique identifier for the Iceberg target node in your data pipeline.</p>
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// <p>Defines the single input source that provides data to this Iceberg target.</p>
    pub fn inputs(&self) -> &[::std::string::String] {
        use std::ops::Deref;
        self.inputs.deref()
    }
    /// <p>Specifies the columns used to partition the Iceberg table data in S3.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.partition_keys.is_none()`.
    pub fn partition_keys(&self) -> &[::std::vec::Vec<::std::string::String>] {
        self.partition_keys.as_deref().unwrap_or_default()
    }
    /// <p>Defines the S3 location where the Iceberg table data will be stored.</p>
    pub fn path(&self) -> &str {
        use std::ops::Deref;
        self.path.deref()
    }
    /// <p>Specifies the file format used for storing Iceberg table data (e.g., Parquet, ORC).</p>
    pub fn format(&self) -> &crate::types::TargetFormat {
        &self.format
    }
    /// <p>Provides additional configuration options for customizing the Iceberg table behavior.</p>
    pub fn additional_options(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.additional_options.as_ref()
    }
    /// <p>Defines how schema changes are handled when writing data to the Iceberg table.</p>
    pub fn schema_change_policy(&self) -> ::std::option::Option<&crate::types::DirectSchemaChangePolicy> {
        self.schema_change_policy.as_ref()
    }
    /// <p>Specifies configuration options for automatic data quality evaluation in Glue jobs. This structure enables automated data quality checks and monitoring during ETL operations, helping to ensure data integrity and reliability without manual intervention.</p>
    pub fn auto_data_quality(&self) -> ::std::option::Option<&crate::types::AutoDataQuality> {
        self.auto_data_quality.as_ref()
    }
    /// <p>Specifies the compression codec used for Iceberg table files in S3.</p>
    pub fn compression(&self) -> &crate::types::IcebergTargetCompressionType {
        &self.compression
    }
    /// <p>Sets the number of target partitions for distributing Iceberg table files across S3.</p>
    pub fn number_target_partitions(&self) -> ::std::option::Option<&str> {
        self.number_target_partitions.as_deref()
    }
    /// <p>Specifies the data schema for the S3 Iceberg direct target.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.output_schemas.is_none()`.
    pub fn output_schemas(&self) -> &[crate::types::GlueSchema] {
        self.output_schemas.as_deref().unwrap_or_default()
    }
}
impl S3IcebergDirectTarget {
    /// Creates a new builder-style object to manufacture [`S3IcebergDirectTarget`](crate::types::S3IcebergDirectTarget).
    pub fn builder() -> crate::types::builders::S3IcebergDirectTargetBuilder {
        crate::types::builders::S3IcebergDirectTargetBuilder::default()
    }
}

/// A builder for [`S3IcebergDirectTarget`](crate::types::S3IcebergDirectTarget).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct S3IcebergDirectTargetBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) partition_keys: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
    pub(crate) path: ::std::option::Option<::std::string::String>,
    pub(crate) format: ::std::option::Option<crate::types::TargetFormat>,
    pub(crate) additional_options: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) schema_change_policy: ::std::option::Option<crate::types::DirectSchemaChangePolicy>,
    pub(crate) auto_data_quality: ::std::option::Option<crate::types::AutoDataQuality>,
    pub(crate) compression: ::std::option::Option<crate::types::IcebergTargetCompressionType>,
    pub(crate) number_target_partitions: ::std::option::Option<::std::string::String>,
    pub(crate) output_schemas: ::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>>,
}
impl S3IcebergDirectTargetBuilder {
    /// <p>Specifies the unique identifier for the Iceberg target node in your data pipeline.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the unique identifier for the Iceberg target node in your data pipeline.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Specifies the unique identifier for the Iceberg target node in your data pipeline.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Appends an item to `inputs`.
    ///
    /// To override the contents of this collection use [`set_inputs`](Self::set_inputs).
    ///
    /// <p>Defines the single input source that provides data to this Iceberg target.</p>
    pub fn inputs(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.inputs.unwrap_or_default();
        v.push(input.into());
        self.inputs = ::std::option::Option::Some(v);
        self
    }
    /// <p>Defines the single input source that provides data to this Iceberg target.</p>
    pub fn set_inputs(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inputs = input;
        self
    }
    /// <p>Defines the single input source that provides data to this Iceberg target.</p>
    pub fn get_inputs(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.inputs
    }
    /// Appends an item to `partition_keys`.
    ///
    /// To override the contents of this collection use [`set_partition_keys`](Self::set_partition_keys).
    ///
    /// <p>Specifies the columns used to partition the Iceberg table data in S3.</p>
    pub fn partition_keys(mut self, input: ::std::vec::Vec<::std::string::String>) -> Self {
        let mut v = self.partition_keys.unwrap_or_default();
        v.push(input);
        self.partition_keys = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the columns used to partition the Iceberg table data in S3.</p>
    pub fn set_partition_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>) -> Self {
        self.partition_keys = input;
        self
    }
    /// <p>Specifies the columns used to partition the Iceberg table data in S3.</p>
    pub fn get_partition_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>> {
        &self.partition_keys
    }
    /// <p>Defines the S3 location where the Iceberg table data will be stored.</p>
    /// This field is required.
    pub fn path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Defines the S3 location where the Iceberg table data will be stored.</p>
    pub fn set_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.path = input;
        self
    }
    /// <p>Defines the S3 location where the Iceberg table data will be stored.</p>
    pub fn get_path(&self) -> &::std::option::Option<::std::string::String> {
        &self.path
    }
    /// <p>Specifies the file format used for storing Iceberg table data (e.g., Parquet, ORC).</p>
    /// This field is required.
    pub fn format(mut self, input: crate::types::TargetFormat) -> Self {
        self.format = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the file format used for storing Iceberg table data (e.g., Parquet, ORC).</p>
    pub fn set_format(mut self, input: ::std::option::Option<crate::types::TargetFormat>) -> Self {
        self.format = input;
        self
    }
    /// <p>Specifies the file format used for storing Iceberg table data (e.g., Parquet, ORC).</p>
    pub fn get_format(&self) -> &::std::option::Option<crate::types::TargetFormat> {
        &self.format
    }
    /// Adds a key-value pair to `additional_options`.
    ///
    /// To override the contents of this collection use [`set_additional_options`](Self::set_additional_options).
    ///
    /// <p>Provides additional configuration options for customizing the Iceberg table behavior.</p>
    pub fn additional_options(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.additional_options.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.additional_options = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Provides additional configuration options for customizing the Iceberg table behavior.</p>
    pub fn set_additional_options(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.additional_options = input;
        self
    }
    /// <p>Provides additional configuration options for customizing the Iceberg table behavior.</p>
    pub fn get_additional_options(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.additional_options
    }
    /// <p>Defines how schema changes are handled when writing data to the Iceberg table.</p>
    pub fn schema_change_policy(mut self, input: crate::types::DirectSchemaChangePolicy) -> Self {
        self.schema_change_policy = ::std::option::Option::Some(input);
        self
    }
    /// <p>Defines how schema changes are handled when writing data to the Iceberg table.</p>
    pub fn set_schema_change_policy(mut self, input: ::std::option::Option<crate::types::DirectSchemaChangePolicy>) -> Self {
        self.schema_change_policy = input;
        self
    }
    /// <p>Defines how schema changes are handled when writing data to the Iceberg table.</p>
    pub fn get_schema_change_policy(&self) -> &::std::option::Option<crate::types::DirectSchemaChangePolicy> {
        &self.schema_change_policy
    }
    /// <p>Specifies configuration options for automatic data quality evaluation in Glue jobs. This structure enables automated data quality checks and monitoring during ETL operations, helping to ensure data integrity and reliability without manual intervention.</p>
    pub fn auto_data_quality(mut self, input: crate::types::AutoDataQuality) -> Self {
        self.auto_data_quality = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies configuration options for automatic data quality evaluation in Glue jobs. This structure enables automated data quality checks and monitoring during ETL operations, helping to ensure data integrity and reliability without manual intervention.</p>
    pub fn set_auto_data_quality(mut self, input: ::std::option::Option<crate::types::AutoDataQuality>) -> Self {
        self.auto_data_quality = input;
        self
    }
    /// <p>Specifies configuration options for automatic data quality evaluation in Glue jobs. This structure enables automated data quality checks and monitoring during ETL operations, helping to ensure data integrity and reliability without manual intervention.</p>
    pub fn get_auto_data_quality(&self) -> &::std::option::Option<crate::types::AutoDataQuality> {
        &self.auto_data_quality
    }
    /// <p>Specifies the compression codec used for Iceberg table files in S3.</p>
    /// This field is required.
    pub fn compression(mut self, input: crate::types::IcebergTargetCompressionType) -> Self {
        self.compression = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the compression codec used for Iceberg table files in S3.</p>
    pub fn set_compression(mut self, input: ::std::option::Option<crate::types::IcebergTargetCompressionType>) -> Self {
        self.compression = input;
        self
    }
    /// <p>Specifies the compression codec used for Iceberg table files in S3.</p>
    pub fn get_compression(&self) -> &::std::option::Option<crate::types::IcebergTargetCompressionType> {
        &self.compression
    }
    /// <p>Sets the number of target partitions for distributing Iceberg table files across S3.</p>
    pub fn number_target_partitions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.number_target_partitions = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Sets the number of target partitions for distributing Iceberg table files across S3.</p>
    pub fn set_number_target_partitions(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.number_target_partitions = input;
        self
    }
    /// <p>Sets the number of target partitions for distributing Iceberg table files across S3.</p>
    pub fn get_number_target_partitions(&self) -> &::std::option::Option<::std::string::String> {
        &self.number_target_partitions
    }
    /// Appends an item to `output_schemas`.
    ///
    /// To override the contents of this collection use [`set_output_schemas`](Self::set_output_schemas).
    ///
    /// <p>Specifies the data schema for the S3 Iceberg direct target.</p>
    pub fn output_schemas(mut self, input: crate::types::GlueSchema) -> Self {
        let mut v = self.output_schemas.unwrap_or_default();
        v.push(input);
        self.output_schemas = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the data schema for the S3 Iceberg direct target.</p>
    pub fn set_output_schemas(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>>) -> Self {
        self.output_schemas = input;
        self
    }
    /// <p>Specifies the data schema for the S3 Iceberg direct target.</p>
    pub fn get_output_schemas(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>> {
        &self.output_schemas
    }
    /// Consumes the builder and constructs a [`S3IcebergDirectTarget`](crate::types::S3IcebergDirectTarget).
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](crate::types::builders::S3IcebergDirectTargetBuilder::name)
    /// - [`inputs`](crate::types::builders::S3IcebergDirectTargetBuilder::inputs)
    /// - [`path`](crate::types::builders::S3IcebergDirectTargetBuilder::path)
    /// - [`format`](crate::types::builders::S3IcebergDirectTargetBuilder::format)
    /// - [`compression`](crate::types::builders::S3IcebergDirectTargetBuilder::compression)
    pub fn build(self) -> ::std::result::Result<crate::types::S3IcebergDirectTarget, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::S3IcebergDirectTarget {
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building S3IcebergDirectTarget",
                )
            })?,
            inputs: self.inputs.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "inputs",
                    "inputs was not specified but it is required when building S3IcebergDirectTarget",
                )
            })?,
            partition_keys: self.partition_keys,
            path: self.path.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "path",
                    "path was not specified but it is required when building S3IcebergDirectTarget",
                )
            })?,
            format: self.format.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "format",
                    "format was not specified but it is required when building S3IcebergDirectTarget",
                )
            })?,
            additional_options: self.additional_options,
            schema_change_policy: self.schema_change_policy,
            auto_data_quality: self.auto_data_quality,
            compression: self.compression.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "compression",
                    "compression was not specified but it is required when building S3IcebergDirectTarget",
                )
            })?,
            number_target_partitions: self.number_target_partitions,
            output_schemas: self.output_schemas,
        })
    }
}
