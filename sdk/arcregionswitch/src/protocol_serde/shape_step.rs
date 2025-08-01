// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_step(
    encoder: &mut ::aws_smithy_cbor::Encoder,
    #[allow(unused)] input: &crate::types::Step,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    encoder.begin_map();
    {
        encoder.str("name").str(input.name.as_str());
    }
    if let Some(var_1) = &input.description {
        encoder.str("description").str(var_1.as_str());
    }
    if let Some(var_2) = &input.execution_block_configuration {
        encoder.str("executionBlockConfiguration");
        crate::protocol_serde::shape_execution_block_configuration::ser_execution_block_configuration(encoder, var_2)?;
    }
    {
        encoder.str("executionBlockType").str(input.execution_block_type.as_str());
    }
    encoder.end();
    Ok(())
}

pub(crate) fn de_step(
    decoder: &mut ::aws_smithy_cbor::Decoder,
) -> ::std::result::Result<crate::types::Step, ::aws_smithy_cbor::decode::DeserializeError> {
    #[allow(clippy::match_single_binding)]
    fn pair(
        mut builder: crate::types::builders::StepBuilder,
        decoder: &mut ::aws_smithy_cbor::Decoder,
    ) -> ::std::result::Result<crate::types::builders::StepBuilder, ::aws_smithy_cbor::decode::DeserializeError> {
        builder = match decoder.str()?.as_ref() {
            "name" => builder.set_name(Some(decoder.string()?)),
            "description" => {
                ::aws_smithy_cbor::decode::set_optional(builder, decoder, |builder, decoder| Ok(builder.set_description(Some(decoder.string()?))))?
            }
            "executionBlockConfiguration" => builder.set_execution_block_configuration(Some(
                crate::protocol_serde::shape_execution_block_configuration::de_execution_block_configuration(decoder)?,
            )),
            "executionBlockType" => {
                builder.set_execution_block_type(Some(decoder.string().map(|s| crate::types::ExecutionBlockType::from(s.as_ref()))?))
            }
            _ => {
                decoder.skip()?;
                builder
            }
        };
        Ok(builder)
    }

    let mut builder = crate::types::builders::StepBuilder::default();

    match decoder.map()? {
        None => loop {
            match decoder.datatype()? {
                ::aws_smithy_cbor::data::Type::Break => {
                    decoder.skip()?;
                    break;
                }
                _ => {
                    builder = pair(builder, decoder)?;
                }
            };
        },
        Some(n) => {
            for _ in 0..n {
                builder = pair(builder, decoder)?;
            }
        }
    };
    #[allow(clippy::needless_question_mark)]
    {
        return Ok(crate::serde_util::step_correct_errors(builder)
            .build()
            .map_err(|err| ::aws_smithy_cbor::decode::DeserializeError::custom(err.to_string(), decoder.position()))?);
    }
}
