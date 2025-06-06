// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_route_leg<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::RouteLeg>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RouteLegBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "FerryLegDetails" => {
                            builder = builder
                                .set_ferry_leg_details(crate::protocol_serde::shape_route_ferry_leg_details::de_route_ferry_leg_details(tokens)?);
                        }
                        "Geometry" => {
                            builder = builder.set_geometry(crate::protocol_serde::shape_route_leg_geometry::de_route_leg_geometry(tokens)?);
                        }
                        "Language" => {
                            builder = builder.set_language(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PedestrianLegDetails" => {
                            builder = builder.set_pedestrian_leg_details(
                                crate::protocol_serde::shape_route_pedestrian_leg_details::de_route_pedestrian_leg_details(tokens)?,
                            );
                        }
                        "TravelMode" => {
                            builder = builder.set_travel_mode(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::RouteLegTravelMode::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "Type" => {
                            builder = builder.set_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::RouteLegType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "VehicleLegDetails" => {
                            builder = builder.set_vehicle_leg_details(
                                crate::protocol_serde::shape_route_vehicle_leg_details::de_route_vehicle_leg_details(tokens)?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(crate::serde_util::route_leg_correct_errors(builder).build().map_err(|err| {
                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
            })?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
