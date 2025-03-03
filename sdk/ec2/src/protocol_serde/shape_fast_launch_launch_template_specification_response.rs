// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_fast_launch_launch_template_specification_response(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::FastLaunchLaunchTemplateSpecificationResponse, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::FastLaunchLaunchTemplateSpecificationResponse::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("launchTemplateId") /* LaunchTemplateId com.amazonaws.ec2#FastLaunchLaunchTemplateSpecificationResponse$LaunchTemplateId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_launch_template_id(var_1);
            }
            ,
            s if s.matches("launchTemplateName") /* LaunchTemplateName com.amazonaws.ec2#FastLaunchLaunchTemplateSpecificationResponse$LaunchTemplateName */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_launch_template_name(var_2);
            }
            ,
            s if s.matches("version") /* Version com.amazonaws.ec2#FastLaunchLaunchTemplateSpecificationResponse$Version */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_version(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
