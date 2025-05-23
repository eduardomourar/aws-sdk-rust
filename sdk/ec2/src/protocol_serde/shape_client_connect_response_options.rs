// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_client_connect_response_options(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ClientConnectResponseOptions, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ClientConnectResponseOptions::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("enabled") /* Enabled com.amazonaws.ec2#ClientConnectResponseOptions$Enabled */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_enabled(var_1);
            }
            ,
            s if s.matches("lambdaFunctionArn") /* LambdaFunctionArn com.amazonaws.ec2#ClientConnectResponseOptions$LambdaFunctionArn */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_lambda_function_arn(var_2);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2#ClientConnectResponseOptions$Status */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_client_vpn_endpoint_attribute_status::de_client_vpn_endpoint_attribute_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
