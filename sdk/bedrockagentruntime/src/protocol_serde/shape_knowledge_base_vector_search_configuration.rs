// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_knowledge_base_vector_search_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::KnowledgeBaseVectorSearchConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("numberOfResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.number_of_results).into()),
        );
    }
    if let Some(var_1) = &input.override_search_type {
        object.key("overrideSearchType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filter {
        #[allow(unused_mut)]
        let mut object_3 = object.key("filter").start_object();
        crate::protocol_serde::shape_retrieval_filter::ser_retrieval_filter(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.reranking_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("rerankingConfiguration").start_object();
        crate::protocol_serde::shape_vector_search_reranking_configuration::ser_vector_search_reranking_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.implicit_filter_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("implicitFilterConfiguration").start_object();
        crate::protocol_serde::shape_implicit_filter_configuration::ser_implicit_filter_configuration(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}
