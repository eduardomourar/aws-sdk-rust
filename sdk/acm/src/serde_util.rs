// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn renewal_summary_correct_errors(
    mut builder: crate::types::builders::RenewalSummaryBuilder,
) -> crate::types::builders::RenewalSummaryBuilder {
    if builder.renewal_status.is_none() {
        builder.renewal_status = "no value was set".parse::<crate::types::RenewalStatus>().ok()
    }
    if builder.domain_validation_options.is_none() {
        builder.domain_validation_options = Some(Default::default())
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    builder
}

pub(crate) fn domain_validation_correct_errors(
    mut builder: crate::types::builders::DomainValidationBuilder,
) -> crate::types::builders::DomainValidationBuilder {
    if builder.domain_name.is_none() {
        builder.domain_name = Some(Default::default())
    }
    builder
}

pub(crate) fn resource_record_correct_errors(
    mut builder: crate::types::builders::ResourceRecordBuilder,
) -> crate::types::builders::ResourceRecordBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::RecordType>().ok()
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}
