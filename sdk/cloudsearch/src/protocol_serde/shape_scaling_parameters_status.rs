// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_scaling_parameters_status(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ScalingParametersStatus, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ScalingParametersStatus::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Options") /* Options com.amazonaws.cloudsearch#ScalingParametersStatus$Options */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_scaling_parameters::de_scaling_parameters(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_options(var_1);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.cloudsearch#ScalingParametersStatus$Status */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_option_status::de_option_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::scaling_parameters_status_correct_errors(builder).build())
}
