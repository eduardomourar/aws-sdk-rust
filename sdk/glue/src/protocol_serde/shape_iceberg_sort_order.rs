// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_iceberg_sort_order(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::IcebergSortOrder,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("OrderId").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.order_id).into()),
        );
    }
    {
        let mut array_1 = object.key("Fields").start_array();
        for item_2 in &input.fields {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_iceberg_sort_field::ser_iceberg_sort_field(&mut object_3, item_2)?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    Ok(())
}
