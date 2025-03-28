// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_reserved_nodes_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_reserved_nodes::DescribeReservedNodesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.reservation_id {
        object.key("ReservationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.reserved_nodes_offering_id {
        object.key("ReservedNodesOfferingId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.node_type {
        object.key("NodeType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.duration {
        object.key("Duration").string(var_4.as_str());
    }
    if let Some(var_5) = &input.offering_type {
        object.key("OfferingType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.next_token {
        object.key("NextToken").string(var_7.as_str());
    }
    Ok(())
}
