// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_test_invoke_method_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::test_invoke_method::TestInvokeMethodInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.body {
        object.key("body").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_certificate_id {
        object.key("clientCertificateId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.headers {
        #[allow(unused_mut)]
        let mut object_4 = object.key("headers").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.multi_value_headers {
        #[allow(unused_mut)]
        let mut object_8 = object.key("multiValueHeaders").start_object();
        for (key_9, value_10) in var_7 {
            {
                let mut array_11 = object_8.key(key_9.as_str()).start_array();
                for item_12 in value_10 {
                    {
                        array_11.value().string(item_12.as_str());
                    }
                }
                array_11.finish();
            }
        }
        object_8.finish();
    }
    if let Some(var_13) = &input.path_with_query_string {
        object.key("pathWithQueryString").string(var_13.as_str());
    }
    if let Some(var_14) = &input.stage_variables {
        #[allow(unused_mut)]
        let mut object_15 = object.key("stageVariables").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16.as_str()).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    Ok(())
}
