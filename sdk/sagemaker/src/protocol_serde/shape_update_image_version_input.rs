// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_image_version_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_image_version::UpdateImageVersionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.image_name {
        object.key("ImageName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.alias {
        object.key("Alias").string(var_2.as_str());
    }
    if let Some(var_3) = &input.version {
        object.key("Version").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.aliases_to_add {
        let mut array_5 = object.key("AliasesToAdd").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.aliases_to_delete {
        let mut array_8 = object.key("AliasesToDelete").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.vendor_guidance {
        object.key("VendorGuidance").string(var_10.as_str());
    }
    if let Some(var_11) = &input.job_type {
        object.key("JobType").string(var_11.as_str());
    }
    if let Some(var_12) = &input.ml_framework {
        object.key("MLFramework").string(var_12.as_str());
    }
    if let Some(var_13) = &input.programming_lang {
        object.key("ProgrammingLang").string(var_13.as_str());
    }
    if let Some(var_14) = &input.processor {
        object.key("Processor").string(var_14.as_str());
    }
    if let Some(var_15) = &input.horovod {
        object.key("Horovod").boolean(*var_15);
    }
    if let Some(var_16) = &input.release_notes {
        object.key("ReleaseNotes").string(var_16.as_str());
    }
    Ok(())
}
