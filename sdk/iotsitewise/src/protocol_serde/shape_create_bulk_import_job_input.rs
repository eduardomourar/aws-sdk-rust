// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_bulk_import_job_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_bulk_import_job::CreateBulkImportJobInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.adaptive_ingestion {
        object.key("adaptiveIngestion").boolean(*var_1);
    }
    if let Some(var_2) = &input.delete_files_after_import {
        object.key("deleteFilesAfterImport").boolean(*var_2);
    }
    if let Some(var_3) = &input.error_report_location {
        #[allow(unused_mut)]
        let mut object_4 = object.key("errorReportLocation").start_object();
        crate::protocol_serde::shape_error_report_location::ser_error_report_location(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.files {
        let mut array_6 = object.key("files").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_file::ser_file(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.job_configuration {
        #[allow(unused_mut)]
        let mut object_10 = object.key("jobConfiguration").start_object();
        crate::protocol_serde::shape_job_configuration::ser_job_configuration(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.job_name {
        object.key("jobName").string(var_11.as_str());
    }
    if let Some(var_12) = &input.job_role_arn {
        object.key("jobRoleArn").string(var_12.as_str());
    }
    Ok(())
}
