// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a specific GPU accelerator required for an Amazon Elastic Compute Cloud worker host.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AcceleratorSelection {
    /// <p>The name of the chip used by the GPU accelerator.</p>
    /// <p>If you specify <code>l4</code> as the name of the accelerator, you must specify <code>latest</code> or <code>grid:r570</code> as the runtime.</p>
    /// <p>The available GPU accelerators are:</p>
    /// <ul>
    /// <li>
    /// <p><code>t4</code> - NVIDIA T4 Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>a10g</code> - NVIDIA A10G Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>l4</code> - NVIDIA L4 Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>l40s</code> - NVIDIA L40S Tensor Core GPU</p></li>
    /// </ul>
    pub name: crate::types::AcceleratorName,
    /// <p>Specifies the runtime driver to use for the GPU accelerator. You must use the same runtime for all GPUs.</p>
    /// <p>You can choose from the following runtimes:</p>
    /// <ul>
    /// <li>
    /// <p><code>latest</code> - Use the latest runtime available for the chip. If you specify <code>latest</code> and a new version of the runtime is released, the new version of the runtime is used.</p></li>
    /// <li>
    /// <p><code>grid:r570</code> - <a href="https://docs.nvidia.com/vgpu/18.0/index.html">NVIDIA vGPU software 18</a></p></li>
    /// <li>
    /// <p><code>grid:r535</code> - <a href="https://docs.nvidia.com/vgpu/16.0/index.html">NVIDIA vGPU software 16</a></p></li>
    /// </ul>
    /// <p>If you don't specify a runtime, Deadline Cloud uses <code>latest</code> as the default. However, if you have multiple accelerators and specify <code>latest</code> for some and leave others blank, Deadline Cloud raises an exception.</p>
    pub runtime: ::std::string::String,
}
impl AcceleratorSelection {
    /// <p>The name of the chip used by the GPU accelerator.</p>
    /// <p>If you specify <code>l4</code> as the name of the accelerator, you must specify <code>latest</code> or <code>grid:r570</code> as the runtime.</p>
    /// <p>The available GPU accelerators are:</p>
    /// <ul>
    /// <li>
    /// <p><code>t4</code> - NVIDIA T4 Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>a10g</code> - NVIDIA A10G Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>l4</code> - NVIDIA L4 Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>l40s</code> - NVIDIA L40S Tensor Core GPU</p></li>
    /// </ul>
    pub fn name(&self) -> &crate::types::AcceleratorName {
        &self.name
    }
    /// <p>Specifies the runtime driver to use for the GPU accelerator. You must use the same runtime for all GPUs.</p>
    /// <p>You can choose from the following runtimes:</p>
    /// <ul>
    /// <li>
    /// <p><code>latest</code> - Use the latest runtime available for the chip. If you specify <code>latest</code> and a new version of the runtime is released, the new version of the runtime is used.</p></li>
    /// <li>
    /// <p><code>grid:r570</code> - <a href="https://docs.nvidia.com/vgpu/18.0/index.html">NVIDIA vGPU software 18</a></p></li>
    /// <li>
    /// <p><code>grid:r535</code> - <a href="https://docs.nvidia.com/vgpu/16.0/index.html">NVIDIA vGPU software 16</a></p></li>
    /// </ul>
    /// <p>If you don't specify a runtime, Deadline Cloud uses <code>latest</code> as the default. However, if you have multiple accelerators and specify <code>latest</code> for some and leave others blank, Deadline Cloud raises an exception.</p>
    pub fn runtime(&self) -> &str {
        use std::ops::Deref;
        self.runtime.deref()
    }
}
impl AcceleratorSelection {
    /// Creates a new builder-style object to manufacture [`AcceleratorSelection`](crate::types::AcceleratorSelection).
    pub fn builder() -> crate::types::builders::AcceleratorSelectionBuilder {
        crate::types::builders::AcceleratorSelectionBuilder::default()
    }
}

/// A builder for [`AcceleratorSelection`](crate::types::AcceleratorSelection).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AcceleratorSelectionBuilder {
    pub(crate) name: ::std::option::Option<crate::types::AcceleratorName>,
    pub(crate) runtime: ::std::option::Option<::std::string::String>,
}
impl AcceleratorSelectionBuilder {
    /// <p>The name of the chip used by the GPU accelerator.</p>
    /// <p>If you specify <code>l4</code> as the name of the accelerator, you must specify <code>latest</code> or <code>grid:r570</code> as the runtime.</p>
    /// <p>The available GPU accelerators are:</p>
    /// <ul>
    /// <li>
    /// <p><code>t4</code> - NVIDIA T4 Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>a10g</code> - NVIDIA A10G Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>l4</code> - NVIDIA L4 Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>l40s</code> - NVIDIA L40S Tensor Core GPU</p></li>
    /// </ul>
    /// This field is required.
    pub fn name(mut self, input: crate::types::AcceleratorName) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the chip used by the GPU accelerator.</p>
    /// <p>If you specify <code>l4</code> as the name of the accelerator, you must specify <code>latest</code> or <code>grid:r570</code> as the runtime.</p>
    /// <p>The available GPU accelerators are:</p>
    /// <ul>
    /// <li>
    /// <p><code>t4</code> - NVIDIA T4 Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>a10g</code> - NVIDIA A10G Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>l4</code> - NVIDIA L4 Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>l40s</code> - NVIDIA L40S Tensor Core GPU</p></li>
    /// </ul>
    pub fn set_name(mut self, input: ::std::option::Option<crate::types::AcceleratorName>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the chip used by the GPU accelerator.</p>
    /// <p>If you specify <code>l4</code> as the name of the accelerator, you must specify <code>latest</code> or <code>grid:r570</code> as the runtime.</p>
    /// <p>The available GPU accelerators are:</p>
    /// <ul>
    /// <li>
    /// <p><code>t4</code> - NVIDIA T4 Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>a10g</code> - NVIDIA A10G Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>l4</code> - NVIDIA L4 Tensor Core GPU</p></li>
    /// <li>
    /// <p><code>l40s</code> - NVIDIA L40S Tensor Core GPU</p></li>
    /// </ul>
    pub fn get_name(&self) -> &::std::option::Option<crate::types::AcceleratorName> {
        &self.name
    }
    /// <p>Specifies the runtime driver to use for the GPU accelerator. You must use the same runtime for all GPUs.</p>
    /// <p>You can choose from the following runtimes:</p>
    /// <ul>
    /// <li>
    /// <p><code>latest</code> - Use the latest runtime available for the chip. If you specify <code>latest</code> and a new version of the runtime is released, the new version of the runtime is used.</p></li>
    /// <li>
    /// <p><code>grid:r570</code> - <a href="https://docs.nvidia.com/vgpu/18.0/index.html">NVIDIA vGPU software 18</a></p></li>
    /// <li>
    /// <p><code>grid:r535</code> - <a href="https://docs.nvidia.com/vgpu/16.0/index.html">NVIDIA vGPU software 16</a></p></li>
    /// </ul>
    /// <p>If you don't specify a runtime, Deadline Cloud uses <code>latest</code> as the default. However, if you have multiple accelerators and specify <code>latest</code> for some and leave others blank, Deadline Cloud raises an exception.</p>
    pub fn runtime(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.runtime = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the runtime driver to use for the GPU accelerator. You must use the same runtime for all GPUs.</p>
    /// <p>You can choose from the following runtimes:</p>
    /// <ul>
    /// <li>
    /// <p><code>latest</code> - Use the latest runtime available for the chip. If you specify <code>latest</code> and a new version of the runtime is released, the new version of the runtime is used.</p></li>
    /// <li>
    /// <p><code>grid:r570</code> - <a href="https://docs.nvidia.com/vgpu/18.0/index.html">NVIDIA vGPU software 18</a></p></li>
    /// <li>
    /// <p><code>grid:r535</code> - <a href="https://docs.nvidia.com/vgpu/16.0/index.html">NVIDIA vGPU software 16</a></p></li>
    /// </ul>
    /// <p>If you don't specify a runtime, Deadline Cloud uses <code>latest</code> as the default. However, if you have multiple accelerators and specify <code>latest</code> for some and leave others blank, Deadline Cloud raises an exception.</p>
    pub fn set_runtime(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.runtime = input;
        self
    }
    /// <p>Specifies the runtime driver to use for the GPU accelerator. You must use the same runtime for all GPUs.</p>
    /// <p>You can choose from the following runtimes:</p>
    /// <ul>
    /// <li>
    /// <p><code>latest</code> - Use the latest runtime available for the chip. If you specify <code>latest</code> and a new version of the runtime is released, the new version of the runtime is used.</p></li>
    /// <li>
    /// <p><code>grid:r570</code> - <a href="https://docs.nvidia.com/vgpu/18.0/index.html">NVIDIA vGPU software 18</a></p></li>
    /// <li>
    /// <p><code>grid:r535</code> - <a href="https://docs.nvidia.com/vgpu/16.0/index.html">NVIDIA vGPU software 16</a></p></li>
    /// </ul>
    /// <p>If you don't specify a runtime, Deadline Cloud uses <code>latest</code> as the default. However, if you have multiple accelerators and specify <code>latest</code> for some and leave others blank, Deadline Cloud raises an exception.</p>
    pub fn get_runtime(&self) -> &::std::option::Option<::std::string::String> {
        &self.runtime
    }
    /// Consumes the builder and constructs a [`AcceleratorSelection`](crate::types::AcceleratorSelection).
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](crate::types::builders::AcceleratorSelectionBuilder::name)
    pub fn build(self) -> ::std::result::Result<crate::types::AcceleratorSelection, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::AcceleratorSelection {
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building AcceleratorSelection",
                )
            })?,
            runtime: self.runtime.unwrap_or_else(|| "latest".to_owned()),
        })
    }
}
