// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_graph_data_summary<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::GraphDataSummary>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::GraphDataSummaryBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "numNodes" => {
                            builder = builder.set_num_nodes(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "numEdges" => {
                            builder = builder.set_num_edges(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "numNodeLabels" => {
                            builder = builder.set_num_node_labels(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "numEdgeLabels" => {
                            builder = builder.set_num_edge_labels(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "nodeLabels" => {
                            builder = builder.set_node_labels(crate::protocol_serde::shape_node_labels::de_node_labels(tokens)?);
                        }
                        "edgeLabels" => {
                            builder = builder.set_edge_labels(crate::protocol_serde::shape_edge_labels::de_edge_labels(tokens)?);
                        }
                        "numNodeProperties" => {
                            builder = builder.set_num_node_properties(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "numEdgeProperties" => {
                            builder = builder.set_num_edge_properties(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "nodeProperties" => {
                            builder =
                                builder.set_node_properties(crate::protocol_serde::shape_long_valued_map_list::de_long_valued_map_list(tokens)?);
                        }
                        "edgeProperties" => {
                            builder =
                                builder.set_edge_properties(crate::protocol_serde::shape_long_valued_map_list::de_long_valued_map_list(tokens)?);
                        }
                        "totalNodePropertyValues" => {
                            builder = builder.set_total_node_property_values(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "totalEdgePropertyValues" => {
                            builder = builder.set_total_edge_property_values(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "nodeStructures" => {
                            builder = builder.set_node_structures(crate::protocol_serde::shape_node_structures::de_node_structures(tokens)?);
                        }
                        "edgeStructures" => {
                            builder = builder.set_edge_structures(crate::protocol_serde::shape_edge_structures::de_edge_structures(tokens)?);
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
