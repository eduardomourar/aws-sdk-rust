// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_describe_addon_versions_output_output_next_token(
    input: &crate::operation::describe_addon_versions::DescribeAddonVersionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_describe_cluster_versions_output_output_next_token(
    input: &crate::operation::describe_cluster_versions::DescribeClusterVersionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_access_entries_output_output_next_token(
    input: &crate::operation::list_access_entries::ListAccessEntriesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_access_policies_output_output_next_token(
    input: &crate::operation::list_access_policies::ListAccessPoliciesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_addons_output_output_next_token(
    input: &crate::operation::list_addons::ListAddonsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_associated_access_policies_output_output_next_token(
    input: &crate::operation::list_associated_access_policies::ListAssociatedAccessPoliciesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_clusters_output_output_next_token(
    input: &crate::operation::list_clusters::ListClustersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_eks_anywhere_subscriptions_output_output_next_token(
    input: &crate::operation::list_eks_anywhere_subscriptions::ListEksAnywhereSubscriptionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_fargate_profiles_output_output_next_token(
    input: &crate::operation::list_fargate_profiles::ListFargateProfilesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_identity_provider_configs_output_output_next_token(
    input: &crate::operation::list_identity_provider_configs::ListIdentityProviderConfigsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_insights_output_output_next_token(
    input: &crate::operation::list_insights::ListInsightsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_nodegroups_output_output_next_token(
    input: &crate::operation::list_nodegroups::ListNodegroupsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_pod_identity_associations_output_output_next_token(
    input: &crate::operation::list_pod_identity_associations::ListPodIdentityAssociationsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_updates_output_output_next_token(
    input: &crate::operation::list_updates::ListUpdatesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_describe_addon_versions_output_output_addons(
    input: crate::operation::describe_addon_versions::DescribeAddonVersionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AddonInfo>> {
    let input = match input.addons {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_describe_cluster_versions_output_output_cluster_versions(
    input: crate::operation::describe_cluster_versions::DescribeClusterVersionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ClusterVersionInformation>> {
    let input = match input.cluster_versions {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_access_entries_output_output_access_entries(
    input: crate::operation::list_access_entries::ListAccessEntriesOutput,
) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
    let input = match input.access_entries {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_access_policies_output_output_access_policies(
    input: crate::operation::list_access_policies::ListAccessPoliciesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AccessPolicy>> {
    let input = match input.access_policies {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_addons_output_output_addons(
    input: crate::operation::list_addons::ListAddonsOutput,
) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
    let input = match input.addons {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_associated_access_policies_output_output_associated_access_policies(
    input: crate::operation::list_associated_access_policies::ListAssociatedAccessPoliciesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AssociatedAccessPolicy>> {
    let input = match input.associated_access_policies {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_clusters_output_output_clusters(
    input: crate::operation::list_clusters::ListClustersOutput,
) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
    let input = match input.clusters {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_eks_anywhere_subscriptions_output_output_subscriptions(
    input: crate::operation::list_eks_anywhere_subscriptions::ListEksAnywhereSubscriptionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::EksAnywhereSubscription>> {
    let input = match input.subscriptions {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_fargate_profiles_output_output_fargate_profile_names(
    input: crate::operation::list_fargate_profiles::ListFargateProfilesOutput,
) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
    let input = match input.fargate_profile_names {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_identity_provider_configs_output_output_identity_provider_configs(
    input: crate::operation::list_identity_provider_configs::ListIdentityProviderConfigsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::IdentityProviderConfig>> {
    let input = match input.identity_provider_configs {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_insights_output_output_insights(
    input: crate::operation::list_insights::ListInsightsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::InsightSummary>> {
    let input = match input.insights {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_nodegroups_output_output_nodegroups(
    input: crate::operation::list_nodegroups::ListNodegroupsOutput,
) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
    let input = match input.nodegroups {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_pod_identity_associations_output_output_associations(
    input: crate::operation::list_pod_identity_associations::ListPodIdentityAssociationsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::PodIdentityAssociationSummary>> {
    let input = match input.associations {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_updates_output_output_update_ids(
    input: crate::operation::list_updates::ListUpdatesOutput,
) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
    let input = match input.update_ids {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}
