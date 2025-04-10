// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_auto_deployment(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::AutoDeployment,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Enabled");
    if let Some(var_2) = &input.enabled {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("RetainStacksOnAccountRemoval");
    if let Some(var_4) = &input.retain_stacks_on_account_removal {
        scope_3.boolean(*var_4);
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_auto_deployment(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::AutoDeployment, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AutoDeployment::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Enabled") /* Enabled com.amazonaws.cloudformation#AutoDeployment$Enabled */ =>  {
                let var_5 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudformation#AutoDeploymentNullable`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_enabled(var_5);
            }
            ,
            s if s.matches("RetainStacksOnAccountRemoval") /* RetainStacksOnAccountRemoval com.amazonaws.cloudformation#AutoDeployment$RetainStacksOnAccountRemoval */ =>  {
                let var_6 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudformation#RetainStacksOnAccountRemovalNullable`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_retain_stacks_on_account_removal(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
