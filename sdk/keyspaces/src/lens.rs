// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_keyspaces_output_output_next_token(
    input: &crate::operation::list_keyspaces::ListKeyspacesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_tables_output_output_next_token(
    input: &crate::operation::list_tables::ListTablesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_tags_for_resource_output_output_next_token(
    input: &crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_types_output_output_next_token(
    input: &crate::operation::list_types::ListTypesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_keyspaces_output_output_keyspaces(
    input: crate::operation::list_keyspaces::ListKeyspacesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::KeyspaceSummary>> {
    let input = input.keyspaces;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_tables_output_output_tables(
    input: crate::operation::list_tables::ListTablesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::TableSummary>> {
    let input = input.tables?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_tags_for_resource_output_output_tags(
    input: crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
    let input = input.tags?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_types_output_output_types(
    input: crate::operation::list_types::ListTypesOutput,
) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
    let input = input.types;
    ::std::option::Option::Some(input)
}
