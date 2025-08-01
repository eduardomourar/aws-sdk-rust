# aws-sdk-codeartifact

CodeArtifact is a fully managed artifact repository compatible with language-native package managers and build tools such as npm, Apache Maven, pip, and dotnet. You can use CodeArtifact to share packages with development teams and pull packages. Packages can be pulled from both public and CodeArtifact repositories. You can also create an upstream relationship between a CodeArtifact repository and another repository, which effectively merges their contents from the point of view of a package manager client.

__CodeArtifact concepts__
  - __Repository__: A CodeArtifact repository contains a set of [package versions](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html#welcome-concepts-package-version), each of which maps to a set of assets, or files. Repositories are polyglot, so a single repository can contain packages of any supported type. Each repository exposes endpoints for fetching and publishing packages using tools such as the __ npm __ CLI or the Maven CLI (__ mvn __). For a list of supported package managers, see the [CodeArtifact User Guide](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html).
  - __Domain__: Repositories are aggregated into a higher-level entity known as a _domain_. All package assets and metadata are stored in the domain, but are consumed through repositories. A given package asset, such as a Maven JAR file, is stored once per domain, no matter how many repositories it's present in. All of the assets and metadata in a domain are encrypted with the same customer master key (CMK) stored in Key Management Service (KMS). Each repository is a member of a single domain and can't be moved to a different domain. The domain allows organizational policy to be applied across multiple repositories, such as which accounts can access repositories in the domain, and which public repositories can be used as sources of packages. Although an organization can have multiple domains, we recommend a single production domain that contains all published artifacts so that teams can find and share packages across their organization.
  - __Package__: A _package_ is a bundle of software and the metadata required to resolve dependencies and install the software. CodeArtifact supports npm, PyPI, Maven, NuGet, Swift, Ruby, Cargo, and generic package formats. For more information about the supported package formats and how to use CodeArtifact with them, see the [CodeArtifact User Guide](https://docs.aws.amazon.com/codeartifact/latest/ug/welcome.html). In CodeArtifact, a package consists of:
    - A _name_ (for example, webpack is the name of a popular npm package)
    - An optional namespace (for example, @types in @types/node)
    - A set of versions (for example, 1.0.0, 1.0.1, 1.0.2, etc.)
    - Package-level metadata (for example, npm tags)

  - __Package group__: A group of packages that match a specified definition. Package groups can be used to apply configuration to multiple packages that match a defined pattern using package format, package namespace, and package name. You can use package groups to more conveniently configure package origin controls for multiple packages. Package origin controls are used to block or allow ingestion or publishing of new package versions, which protects users from malicious actions known as dependency substitution attacks.
  - __Package version__: A version of a package, such as @types/node 12.6.9. The version number format and semantics vary for different package formats. For example, npm package versions must conform to the [Semantic Versioning specification](https://semver.org/). In CodeArtifact, a package version consists of the version identifier, metadata at the package version level, and a set of assets.
  - __Upstream repository__: One repository is _upstream_ of another when the package versions in it can be accessed from the repository endpoint of the downstream repository, effectively merging the contents of the two repositories from the point of view of a client. CodeArtifact allows creating an upstream relationship between two repositories.
  - __Asset__: An individual file stored in CodeArtifact associated with a package version, such as an npm .tgz file or Maven POM and JAR files.

__CodeArtifact supported API operations__
  - AssociateExternalConnection: Adds an existing external connection to a repository.
  - CopyPackageVersions: Copies package versions from one repository to another repository in the same domain.
  - CreateDomain: Creates a domain.
  - CreatePackageGroup: Creates a package group.
  - CreateRepository: Creates a CodeArtifact repository in a domain.
  - DeleteDomain: Deletes a domain. You cannot delete a domain that contains repositories.
  - DeleteDomainPermissionsPolicy: Deletes the resource policy that is set on a domain.
  - DeletePackage: Deletes a package and all associated package versions.
  - DeletePackageGroup: Deletes a package group. Does not delete packages or package versions that are associated with a package group.
  - DeletePackageVersions: Deletes versions of a package. After a package has been deleted, it can be republished, but its assets and metadata cannot be restored because they have been permanently removed from storage.
  - DeleteRepository: Deletes a repository.
  - DeleteRepositoryPermissionsPolicy: Deletes the resource policy that is set on a repository.
  - DescribeDomain: Returns a DomainDescription object that contains information about the requested domain.
  - DescribePackage: Returns a [PackageDescription](https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageDescription.html) object that contains details about a package.
  - DescribePackageGroup: Returns a [PackageGroup](https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageGroup.html) object that contains details about a package group.
  - DescribePackageVersion: Returns a [PackageVersionDescription](https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageVersionDescription.html) object that contains details about a package version.
  - DescribeRepository: Returns a RepositoryDescription object that contains detailed information about the requested repository.
  - DisposePackageVersions: Disposes versions of a package. A package version with the status Disposed cannot be restored because they have been permanently removed from storage.
  - DisassociateExternalConnection: Removes an existing external connection from a repository.
  - GetAssociatedPackageGroup: Returns the most closely associated package group to the specified package.
  - GetAuthorizationToken: Generates a temporary authorization token for accessing repositories in the domain. The token expires the authorization period has passed. The default authorization period is 12 hours and can be customized to any length with a maximum of 12 hours.
  - GetDomainPermissionsPolicy: Returns the policy of a resource that is attached to the specified domain.
  - GetPackageVersionAsset: Returns the contents of an asset that is in a package version.
  - GetPackageVersionReadme: Gets the readme file or descriptive text for a package version.
  - GetRepositoryEndpoint: Returns the endpoint of a repository for a specific package format. A repository has one endpoint for each package format:
    - cargo
    - generic
    - maven
    - npm
    - nuget
    - pypi
    - ruby
    - swift

  - GetRepositoryPermissionsPolicy: Returns the resource policy that is set on a repository.
  - ListAllowedRepositoriesForGroup: Lists the allowed repositories for a package group that has origin configuration set to ALLOW_SPECIFIC_REPOSITORIES.
  - ListAssociatedPackages: Returns a list of packages associated with the requested package group.
  - ListDomains: Returns a list of DomainSummary objects. Each returned DomainSummary object contains information about a domain.
  - ListPackages: Lists the packages in a repository.
  - ListPackageGroups: Returns a list of package groups in the requested domain.
  - ListPackageVersionAssets: Lists the assets for a given package version.
  - ListPackageVersionDependencies: Returns a list of the direct dependencies for a package version.
  - ListPackageVersions: Returns a list of package versions for a specified package in a repository.
  - ListRepositories: Returns a list of repositories owned by the Amazon Web Services account that called this method.
  - ListRepositoriesInDomain: Returns a list of the repositories in a domain.
  - ListSubPackageGroups: Returns a list of direct children of the specified package group.
  - PublishPackageVersion: Creates a new package version containing one or more assets.
  - PutDomainPermissionsPolicy: Attaches a resource policy to a domain.
  - PutPackageOriginConfiguration: Sets the package origin configuration for a package, which determine how new versions of the package can be added to a specific repository.
  - PutRepositoryPermissionsPolicy: Sets the resource policy on a repository that specifies permissions to access it.
  - UpdatePackageGroup: Updates a package group. This API cannot be used to update a package group's origin configuration or pattern.
  - UpdatePackageGroupOriginConfiguration: Updates the package origin configuration for a package group.
  - UpdatePackageVersionsStatus: Updates the status of one or more versions of a package.
  - UpdateRepository: Updates the properties of a repository.

## Getting Started

> Examples are available for many services and operations, check out the
> [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).

The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-codeartifact` to
your project, add the following to your **Cargo.toml** file:

```toml
[dependencies]
aws-config = { version = "1.1.7", features = ["behavior-version-latest"] }
aws-sdk-codeartifact = "1.81.0"
tokio = { version = "1", features = ["full"] }
```

Then in code, a client can be created with the following:

```rust,no_run
use aws_sdk_codeartifact as codeartifact;

#[::tokio::main]
async fn main() -> Result<(), codeartifact::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_codeartifact::Client::new(&config);

    // ... make some calls with the client

    Ok(())
}
```

See the [client documentation](https://docs.rs/aws-sdk-codeartifact/latest/aws_sdk_codeartifact/client/struct.Client.html)
for information on what calls can be made, and the inputs and outputs for each of those calls.

## Using the SDK

Until the SDK is released, we will be adding information about using the SDK to the
[Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
additional sections for the guide by opening an issue and describing what you are trying to do.

## Getting Help

* [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
* [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) - For bug reports & feature requests
* [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
* [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples)

## License

This project is licensed under the Apache-2.0 License.

