// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a peering connection between a VPC on one of your Amazon Web Services accounts and the VPC for your Amazon GameLift Servers fleets. This record may be for an active peering connection or a pending connection that has not yet been established.</p>
/// <p><b>Related actions</b></p>
/// <p><a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/reference-awssdk.html#reference-awssdk-resources-fleets">All APIs by task</a></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpcPeeringConnection {
    /// <p>A unique identifier for the fleet. This ID determines the ID of the Amazon GameLift Servers VPC for your fleet.</p>
    pub fleet_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) associated with the GameLift fleet resource for this connection.</p>
    pub fleet_arn: ::std::option::Option<::std::string::String>,
    /// <p>CIDR block of IPv4 addresses assigned to the VPC peering connection for the GameLift VPC. The peered VPC also has an IPv4 CIDR block associated with it; these blocks cannot overlap or the peering connection cannot be created.</p>
    pub ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier that is automatically assigned to the connection record. This ID is referenced in VPC peering connection events, and is used when deleting a connection.</p>
    pub vpc_peering_connection_id: ::std::option::Option<::std::string::String>,
    /// <p>The status information about the connection. Status indicates if a connection is pending, successful, or failed.</p>
    pub status: ::std::option::Option<crate::types::VpcPeeringConnectionStatus>,
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p>
    pub peer_vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier for the VPC that contains the Amazon GameLift Servers fleet for this connection. This VPC is managed by Amazon GameLift Servers and does not appear in your Amazon Web Services account.</p>
    pub game_lift_vpc_id: ::std::option::Option<::std::string::String>,
}
impl VpcPeeringConnection {
    /// <p>A unique identifier for the fleet. This ID determines the ID of the Amazon GameLift Servers VPC for your fleet.</p>
    pub fn fleet_id(&self) -> ::std::option::Option<&str> {
        self.fleet_id.as_deref()
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) associated with the GameLift fleet resource for this connection.</p>
    pub fn fleet_arn(&self) -> ::std::option::Option<&str> {
        self.fleet_arn.as_deref()
    }
    /// <p>CIDR block of IPv4 addresses assigned to the VPC peering connection for the GameLift VPC. The peered VPC also has an IPv4 CIDR block associated with it; these blocks cannot overlap or the peering connection cannot be created.</p>
    pub fn ipv4_cidr_block(&self) -> ::std::option::Option<&str> {
        self.ipv4_cidr_block.as_deref()
    }
    /// <p>A unique identifier that is automatically assigned to the connection record. This ID is referenced in VPC peering connection events, and is used when deleting a connection.</p>
    pub fn vpc_peering_connection_id(&self) -> ::std::option::Option<&str> {
        self.vpc_peering_connection_id.as_deref()
    }
    /// <p>The status information about the connection. Status indicates if a connection is pending, successful, or failed.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::VpcPeeringConnectionStatus> {
        self.status.as_ref()
    }
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p>
    pub fn peer_vpc_id(&self) -> ::std::option::Option<&str> {
        self.peer_vpc_id.as_deref()
    }
    /// <p>A unique identifier for the VPC that contains the Amazon GameLift Servers fleet for this connection. This VPC is managed by Amazon GameLift Servers and does not appear in your Amazon Web Services account.</p>
    pub fn game_lift_vpc_id(&self) -> ::std::option::Option<&str> {
        self.game_lift_vpc_id.as_deref()
    }
}
impl VpcPeeringConnection {
    /// Creates a new builder-style object to manufacture [`VpcPeeringConnection`](crate::types::VpcPeeringConnection).
    pub fn builder() -> crate::types::builders::VpcPeeringConnectionBuilder {
        crate::types::builders::VpcPeeringConnectionBuilder::default()
    }
}

/// A builder for [`VpcPeeringConnection`](crate::types::VpcPeeringConnection).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct VpcPeeringConnectionBuilder {
    pub(crate) fleet_id: ::std::option::Option<::std::string::String>,
    pub(crate) fleet_arn: ::std::option::Option<::std::string::String>,
    pub(crate) ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_peering_connection_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::VpcPeeringConnectionStatus>,
    pub(crate) peer_vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) game_lift_vpc_id: ::std::option::Option<::std::string::String>,
}
impl VpcPeeringConnectionBuilder {
    /// <p>A unique identifier for the fleet. This ID determines the ID of the Amazon GameLift Servers VPC for your fleet.</p>
    pub fn fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fleet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the fleet. This ID determines the ID of the Amazon GameLift Servers VPC for your fleet.</p>
    pub fn set_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fleet_id = input;
        self
    }
    /// <p>A unique identifier for the fleet. This ID determines the ID of the Amazon GameLift Servers VPC for your fleet.</p>
    pub fn get_fleet_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.fleet_id
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) associated with the GameLift fleet resource for this connection.</p>
    pub fn fleet_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fleet_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) associated with the GameLift fleet resource for this connection.</p>
    pub fn set_fleet_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fleet_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) associated with the GameLift fleet resource for this connection.</p>
    pub fn get_fleet_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.fleet_arn
    }
    /// <p>CIDR block of IPv4 addresses assigned to the VPC peering connection for the GameLift VPC. The peered VPC also has an IPv4 CIDR block associated with it; these blocks cannot overlap or the peering connection cannot be created.</p>
    pub fn ipv4_cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipv4_cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>CIDR block of IPv4 addresses assigned to the VPC peering connection for the GameLift VPC. The peered VPC also has an IPv4 CIDR block associated with it; these blocks cannot overlap or the peering connection cannot be created.</p>
    pub fn set_ipv4_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipv4_cidr_block = input;
        self
    }
    /// <p>CIDR block of IPv4 addresses assigned to the VPC peering connection for the GameLift VPC. The peered VPC also has an IPv4 CIDR block associated with it; these blocks cannot overlap or the peering connection cannot be created.</p>
    pub fn get_ipv4_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipv4_cidr_block
    }
    /// <p>A unique identifier that is automatically assigned to the connection record. This ID is referenced in VPC peering connection events, and is used when deleting a connection.</p>
    pub fn vpc_peering_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_peering_connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier that is automatically assigned to the connection record. This ID is referenced in VPC peering connection events, and is used when deleting a connection.</p>
    pub fn set_vpc_peering_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_peering_connection_id = input;
        self
    }
    /// <p>A unique identifier that is automatically assigned to the connection record. This ID is referenced in VPC peering connection events, and is used when deleting a connection.</p>
    pub fn get_vpc_peering_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_peering_connection_id
    }
    /// <p>The status information about the connection. Status indicates if a connection is pending, successful, or failed.</p>
    pub fn status(mut self, input: crate::types::VpcPeeringConnectionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status information about the connection. Status indicates if a connection is pending, successful, or failed.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::VpcPeeringConnectionStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status information about the connection. Status indicates if a connection is pending, successful, or failed.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::VpcPeeringConnectionStatus> {
        &self.status
    }
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p>
    pub fn peer_vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.peer_vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p>
    pub fn set_peer_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.peer_vpc_id = input;
        self
    }
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p>
    pub fn get_peer_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.peer_vpc_id
    }
    /// <p>A unique identifier for the VPC that contains the Amazon GameLift Servers fleet for this connection. This VPC is managed by Amazon GameLift Servers and does not appear in your Amazon Web Services account.</p>
    pub fn game_lift_vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.game_lift_vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the VPC that contains the Amazon GameLift Servers fleet for this connection. This VPC is managed by Amazon GameLift Servers and does not appear in your Amazon Web Services account.</p>
    pub fn set_game_lift_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.game_lift_vpc_id = input;
        self
    }
    /// <p>A unique identifier for the VPC that contains the Amazon GameLift Servers fleet for this connection. This VPC is managed by Amazon GameLift Servers and does not appear in your Amazon Web Services account.</p>
    pub fn get_game_lift_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.game_lift_vpc_id
    }
    /// Consumes the builder and constructs a [`VpcPeeringConnection`](crate::types::VpcPeeringConnection).
    pub fn build(self) -> crate::types::VpcPeeringConnection {
        crate::types::VpcPeeringConnection {
            fleet_id: self.fleet_id,
            fleet_arn: self.fleet_arn,
            ipv4_cidr_block: self.ipv4_cidr_block,
            vpc_peering_connection_id: self.vpc_peering_connection_id,
            status: self.status,
            peer_vpc_id: self.peer_vpc_id,
            game_lift_vpc_id: self.game_lift_vpc_id,
        }
    }
}
