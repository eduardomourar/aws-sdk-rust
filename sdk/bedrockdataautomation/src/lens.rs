// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_blueprints_output_output_next_token(
    input: &crate::operation::list_blueprints::ListBlueprintsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_data_automation_projects_output_output_next_token(
    input: &crate::operation::list_data_automation_projects::ListDataAutomationProjectsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_blueprints_output_output_blueprints(
    input: crate::operation::list_blueprints::ListBlueprintsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::BlueprintSummary>> {
    let input = input.blueprints;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_data_automation_projects_output_output_projects(
    input: crate::operation::list_data_automation_projects::ListDataAutomationProjectsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::DataAutomationProjectSummary>> {
    let input = input.projects;
    ::std::option::Option::Some(input)
}
