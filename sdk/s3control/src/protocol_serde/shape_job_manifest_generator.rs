// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_job_manifest_generator(
    input: &crate::types::JobManifestGenerator,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    let mut scope_writer = writer.finish();
    match input {
        crate::types::JobManifestGenerator::S3JobManifestGenerator(inner) => {
            let inner_writer = scope_writer.start_el("S3JobManifestGenerator");
            crate::protocol_serde::shape_s3_job_manifest_generator::ser_s3_job_manifest_generator(inner, inner_writer)?
        }
        crate::types::JobManifestGenerator::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "JobManifestGenerator",
            ))
        }
    }
    Ok(())
}

pub fn de_job_manifest_generator(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::JobManifestGenerator, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut base: Option<crate::types::JobManifestGenerator> = None;
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("S3JobManifestGenerator") /* S3JobManifestGenerator com.amazonaws.s3control#JobManifestGenerator$S3JobManifestGenerator */ =>  {
                let tmp =
                    crate::protocol_serde::shape_s3_job_manifest_generator::de_s3_job_manifest_generator(&mut tag)
                    ?
                ;
                base = Some(crate::types::JobManifestGenerator::S3JobManifestGenerator(tmp));
            }
            ,
            _unknown => base = Some(crate::types::JobManifestGenerator::Unknown),
        }
    }
    base.ok_or_else(|| ::aws_smithy_xml::decode::XmlDecodeError::custom("expected union, got nothing"))
}
