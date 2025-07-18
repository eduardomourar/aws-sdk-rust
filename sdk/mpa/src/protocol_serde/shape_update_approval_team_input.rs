// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_approval_team_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_approval_team::UpdateApprovalTeamInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.approval_strategy {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ApprovalStrategy").start_object();
        crate::protocol_serde::shape_approval_strategy::ser_approval_strategy(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.approvers {
        let mut array_4 = object.key("Approvers").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_approval_team_request_approver::ser_approval_team_request_approver(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.description {
        object.key("Description").string(var_7.as_str());
    }
    Ok(())
}
