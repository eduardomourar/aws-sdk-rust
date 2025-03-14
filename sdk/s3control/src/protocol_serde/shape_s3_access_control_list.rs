// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_access_control_list(
    input: &crate::types::S3AccessControlList,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.owner {
        let inner_writer = scope.start_el("Owner");
        crate::protocol_serde::shape_s3_object_owner::ser_s3_object_owner(var_1, inner_writer)?
    }
    if let Some(var_2) = &input.grants {
        let mut inner_writer = scope.start_el("Grants").finish();
        for list_item_3 in var_2 {
            {
                let inner_writer = inner_writer.start_el("member");
                crate::protocol_serde::shape_s3_grant::ser_s3_grant(list_item_3, inner_writer)?
            }
        }
    }
    scope.finish();
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_s3_access_control_list(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::S3AccessControlList, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::S3AccessControlList::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Owner") /* Owner com.amazonaws.s3control#S3AccessControlList$Owner */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_s3_object_owner::de_s3_object_owner(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_owner(var_4);
            }
            ,
            s if s.matches("Grants") /* Grants com.amazonaws.s3control#S3AccessControlList$Grants */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_s3_grant_list::de_s3_grant_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_grants(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::s3_access_control_list_correct_errors(builder).build())
}
