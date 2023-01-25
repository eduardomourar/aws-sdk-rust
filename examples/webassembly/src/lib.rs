/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_config::environment::credentials::EnvironmentVariableCredentialsProvider;
use aws_config::meta::region::RegionProviderChain;
use aws_config::retry::RetryConfig;
use aws_sdk_s3::{Client, Region, PKG_VERSION};
use aws_smithy_client::erase::DynConnector;
use aws_smithy_http::body::SdkBody;
use aws_smithy_types::timeout::TimeoutConfig;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async extern "C" fn _start() {
    tracing_subscriber::fmt::init();
    let Opt { region, verbose } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new)).or_else(Region::new("us-west-2"));
    println!();

    if verbose {
        println!("S3 client version:   {}", PKG_VERSION);
        println!(
            "Region:               {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!();
    }

    let shared_config = aws_config::from_env()
        .region(region_provider)
        .credentials_provider(EnvironmentVariableCredentialsProvider::new())
        .timeout_config(TimeoutConfig::disabled())
        .retry_config(RetryConfig::standard().with_max_attempts(1))
        .http_connector(DynConnector::new(Adapter::default()))
        .load()
        .await;
    let client = Client::new(&shared_config);

    let resp = client.list_buckets().send().await.unwrap();
    let buckets = resp.buckets().unwrap_or_default();
    let num_buckets = buckets.len();

    for bucket in buckets {
        println!("{}", bucket.name().unwrap_or_default());
    }

    println!();
    println!("Found {} buckets in all regions.", num_buckets);
}

#[derive(Default, Debug, Clone)]
struct Adapter {}

impl tower::Service<http::Request<SdkBody>> for Adapter {
    type Response = http::Response<SdkBody>;

    type Error = aws_smithy_http::result::ConnectorError;

    #[allow(clippy::type_complexity)]
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<SdkBody>) -> Self::Future {
        let (parts, sdk_body) = req.into_parts();
        println!("Sending request to {}", parts.uri);
        println!("Request body: {:?}", sdk_body);
        println!();

        // Consumers here would pass the HTTP request to
        // the Wasm host in order to get the response back
        let body = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<ListAllMyBucketsResult>
   <Buckets>
      <Bucket>
         <CreationDate>2023-01-23T11:59:03.575496Z</CreationDate>
         <Name>doc-example-bucket</Name>
      </Bucket>
      <Bucket>
         <CreationDate>2023-01-23T23:32:13.125238Z</CreationDate>
         <Name>doc-example-bucket2</Name>
      </Bucket>
   </Buckets>
   <Owner>
      <DisplayName>account-name</DisplayName>
      <ID>a3a42310-42d0-46d1-9745-0cee9f4fb851</ID>
   </Owner>
</ListAllMyBucketsResult>";
        let res = http::Response::new(SdkBody::from(body));

        Box::pin(async move { Ok(res) })
    }
}
