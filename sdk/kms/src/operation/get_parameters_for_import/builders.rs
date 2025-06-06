// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_parameters_for_import::_get_parameters_for_import_output::GetParametersForImportOutputBuilder;

pub use crate::operation::get_parameters_for_import::_get_parameters_for_import_input::GetParametersForImportInputBuilder;

impl crate::operation::get_parameters_for_import::builders::GetParametersForImportInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_parameters_for_import::GetParametersForImportOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_parameters_for_import::GetParametersForImportError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_parameters_for_import();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetParametersForImport`.
///
/// <p>Returns the public key and an import token you need to import or reimport key material for a KMS key.</p>
/// <p>By default, KMS keys are created with key material that KMS generates. This operation supports <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing key material</a>, an advanced feature that lets you generate and import the cryptographic key material for a KMS key.</p>
/// <p>Before calling <code>GetParametersForImport</code>, use the <code>CreateKey</code> operation with an <code>Origin</code> value of <code>EXTERNAL</code> to create a KMS key with no key material. You can import key material for a symmetric encryption KMS key, HMAC KMS key, asymmetric encryption KMS key, or asymmetric signing KMS key. You can also import key material into a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">multi-Region key</a> of any supported type. However, you can't import key material into a KMS key in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. You can also use <code>GetParametersForImport</code> to get a public key and import token to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-import-key-material.html#reimport-key-material">reimport the original key material</a> into a KMS key whose key material expired or was deleted.</p>
/// <p><code>GetParametersForImport</code> returns the items that you need to import your key material.</p>
/// <ul>
/// <li>
/// <p>The public key (or "wrapping key") of an RSA key pair that KMS generates.</p>
/// <p>You will use this public key to encrypt ("wrap") your key material while it's in transit to KMS.</p></li>
/// <li>
/// <p>A import token that ensures that KMS can decrypt your key material and associate it with the correct KMS key.</p></li>
/// </ul>
/// <p>The public key and its import token are permanently linked and must be used together. Each public key and import token set is valid for 24 hours. The expiration date and time appear in the <code>ParametersValidTo</code> field in the <code>GetParametersForImport</code> response. You cannot use an expired public key or import token in an <code>ImportKeyMaterial</code> request. If your key and token expire, send another <code>GetParametersForImport</code> request.</p>
/// <p><code>GetParametersForImport</code> requires the following information:</p>
/// <ul>
/// <li>
/// <p>The key ID of the KMS key for which you are importing the key material.</p></li>
/// <li>
/// <p>The key spec of the public key ("wrapping key") that you will use to encrypt your key material during import.</p></li>
/// <li>
/// <p>The wrapping algorithm that you will use with the public key to encrypt your key material.</p></li>
/// </ul>
/// <p>You can use the same or a different public key spec and wrapping algorithm each time you import or reimport the same key material.</p>
/// <p>The KMS key that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p><b>Cross-account use</b>: No. You cannot perform this operation on a KMS key in a different Amazon Web Services account.</p>
/// <p><b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GetParametersForImport</a> (key policy)</p>
/// <p><b>Related operations:</b></p>
/// <ul>
/// <li>
/// <p><code>ImportKeyMaterial</code></p></li>
/// <li>
/// <p><code>DeleteImportedKeyMaterial</code></p></li>
/// </ul>
/// <p><b>Eventual consistency</b>: The KMS API follows an eventual consistency model. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/accessing-kms.html#programming-eventual-consistency">KMS eventual consistency</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetParametersForImportFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_parameters_for_import::builders::GetParametersForImportInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_parameters_for_import::GetParametersForImportOutput,
        crate::operation::get_parameters_for_import::GetParametersForImportError,
    > for GetParametersForImportFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_parameters_for_import::GetParametersForImportOutput,
            crate::operation::get_parameters_for_import::GetParametersForImportError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetParametersForImportFluentBuilder {
    /// Creates a new `GetParametersForImportFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetParametersForImport as a reference.
    pub fn as_input(&self) -> &crate::operation::get_parameters_for_import::builders::GetParametersForImportInputBuilder {
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
        crate::operation::get_parameters_for_import::GetParametersForImportOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_parameters_for_import::GetParametersForImportError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_parameters_for_import::GetParametersForImport::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_parameters_for_import::GetParametersForImport::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_parameters_for_import::GetParametersForImportOutput,
        crate::operation::get_parameters_for_import::GetParametersForImportError,
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
    /// <p>The identifier of the KMS key that will be associated with the imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p>
    /// <p>All KMS key types are supported, including multi-Region keys. However, you cannot import key material into a KMS key in a custom key store.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_id(input.into());
        self
    }
    /// <p>The identifier of the KMS key that will be associated with the imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p>
    /// <p>All KMS key types are supported, including multi-Region keys. However, you cannot import key material into a KMS key in a custom key store.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_id(input);
        self
    }
    /// <p>The identifier of the KMS key that will be associated with the imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p>
    /// <p>All KMS key types are supported, including multi-Region keys. However, you cannot import key material into a KMS key in a custom key store.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_id()
    }
    /// <p>The algorithm you will use with the RSA public key (<code>PublicKey</code>) in the response to protect your key material during import. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-get-public-key-and-token.html#select-wrapping-algorithm">Select a wrapping algorithm</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>For RSA_AES wrapping algorithms, you encrypt your key material with an AES key that you generate, then encrypt your AES key with the RSA public key from KMS. For RSAES wrapping algorithms, you encrypt your key material directly with the RSA public key from KMS.</p>
    /// <p>The wrapping algorithms that you can use depend on the type of key material that you are importing. To import an RSA private key, you must use an RSA_AES wrapping algorithm.</p>
    /// <ul>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_256</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_1</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_256</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_256 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_1</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_1 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_PKCS1_V1_5</b> (Deprecated) — As of October 10, 2023, KMS does not support the RSAES_PKCS1_V1_5 wrapping algorithm.</p></li>
    /// </ul>
    pub fn wrapping_algorithm(mut self, input: crate::types::AlgorithmSpec) -> Self {
        self.inner = self.inner.wrapping_algorithm(input);
        self
    }
    /// <p>The algorithm you will use with the RSA public key (<code>PublicKey</code>) in the response to protect your key material during import. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-get-public-key-and-token.html#select-wrapping-algorithm">Select a wrapping algorithm</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>For RSA_AES wrapping algorithms, you encrypt your key material with an AES key that you generate, then encrypt your AES key with the RSA public key from KMS. For RSAES wrapping algorithms, you encrypt your key material directly with the RSA public key from KMS.</p>
    /// <p>The wrapping algorithms that you can use depend on the type of key material that you are importing. To import an RSA private key, you must use an RSA_AES wrapping algorithm.</p>
    /// <ul>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_256</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_1</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_256</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_256 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_1</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_1 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_PKCS1_V1_5</b> (Deprecated) — As of October 10, 2023, KMS does not support the RSAES_PKCS1_V1_5 wrapping algorithm.</p></li>
    /// </ul>
    pub fn set_wrapping_algorithm(mut self, input: ::std::option::Option<crate::types::AlgorithmSpec>) -> Self {
        self.inner = self.inner.set_wrapping_algorithm(input);
        self
    }
    /// <p>The algorithm you will use with the RSA public key (<code>PublicKey</code>) in the response to protect your key material during import. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-get-public-key-and-token.html#select-wrapping-algorithm">Select a wrapping algorithm</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>For RSA_AES wrapping algorithms, you encrypt your key material with an AES key that you generate, then encrypt your AES key with the RSA public key from KMS. For RSAES wrapping algorithms, you encrypt your key material directly with the RSA public key from KMS.</p>
    /// <p>The wrapping algorithms that you can use depend on the type of key material that you are importing. To import an RSA private key, you must use an RSA_AES wrapping algorithm.</p>
    /// <ul>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_256</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_1</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_256</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_256 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_1</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_1 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_PKCS1_V1_5</b> (Deprecated) — As of October 10, 2023, KMS does not support the RSAES_PKCS1_V1_5 wrapping algorithm.</p></li>
    /// </ul>
    pub fn get_wrapping_algorithm(&self) -> &::std::option::Option<crate::types::AlgorithmSpec> {
        self.inner.get_wrapping_algorithm()
    }
    /// <p>The type of RSA public key to return in the response. You will use this wrapping key with the specified wrapping algorithm to protect your key material during import.</p>
    /// <p>Use the longest RSA wrapping key that is practical.</p>
    /// <p>You cannot use an RSA_2048 public key to directly wrap an ECC_NIST_P521 private key. Instead, use an RSA_AES wrapping algorithm or choose a longer RSA public key.</p>
    pub fn wrapping_key_spec(mut self, input: crate::types::WrappingKeySpec) -> Self {
        self.inner = self.inner.wrapping_key_spec(input);
        self
    }
    /// <p>The type of RSA public key to return in the response. You will use this wrapping key with the specified wrapping algorithm to protect your key material during import.</p>
    /// <p>Use the longest RSA wrapping key that is practical.</p>
    /// <p>You cannot use an RSA_2048 public key to directly wrap an ECC_NIST_P521 private key. Instead, use an RSA_AES wrapping algorithm or choose a longer RSA public key.</p>
    pub fn set_wrapping_key_spec(mut self, input: ::std::option::Option<crate::types::WrappingKeySpec>) -> Self {
        self.inner = self.inner.set_wrapping_key_spec(input);
        self
    }
    /// <p>The type of RSA public key to return in the response. You will use this wrapping key with the specified wrapping algorithm to protect your key material during import.</p>
    /// <p>Use the longest RSA wrapping key that is practical.</p>
    /// <p>You cannot use an RSA_2048 public key to directly wrap an ECC_NIST_P521 private key. Instead, use an RSA_AES wrapping algorithm or choose a longer RSA public key.</p>
    pub fn get_wrapping_key_spec(&self) -> &::std::option::Option<crate::types::WrappingKeySpec> {
        self.inner.get_wrapping_key_spec()
    }
}
