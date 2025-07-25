// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct SuggestInput {
    /// <p>The free-form text query to match addresses against. This is usually a partially typed address from an end user in an address box or form.</p><note>
    /// <p>The fields <code>QueryText</code>, and <code>QueryID</code> are mutually exclusive.</p>
    /// </note>
    pub query_text: ::std::option::Option<::std::string::String>,
    /// <p>An optional limit for the number of results returned in a single call.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>Maximum number of query terms to be returned for use with a search text query.</p>
    pub max_query_refinements: ::std::option::Option<i32>,
    /// <p>The position, in longitude and latitude, that the results should be close to. Typically, place results returned are ranked higher the closer they are to this position. Stored in <code>\[lng, lat\]</code> and in the WSG84 format.</p><note>
    /// <p>The fields <code>BiasPosition</code>, <code>FilterBoundingBox</code>, and <code>FilterCircle</code> are mutually exclusive.</p>
    /// </note>
    pub bias_position: ::std::option::Option<::std::vec::Vec<f64>>,
    /// <p>A structure which contains a set of inclusion/exclusion properties that results must possess in order to be returned as a result.</p>
    pub filter: ::std::option::Option<crate::types::SuggestFilter>,
    /// <p>A list of optional additional parameters, such as time zone, that can be requested for each result.</p>
    pub additional_features: ::std::option::Option<::std::vec::Vec<crate::types::SuggestAdditionalFeature>>,
    /// <p>A list of <a href="https://en.wikipedia.org/wiki/IETF_language_tag">BCP 47</a> compliant language codes for the results to be rendered in. If there is no data for the result in the requested language, data will be returned in the default language for the entry.</p>
    pub language: ::std::option::Option<::std::string::String>,
    /// <p>The alpha-2 or alpha-3 character code for the political view of a country. The political view applies to the results of the request to represent unresolved territorial claims through the point of view of the specified country.</p>
    pub political_view: ::std::option::Option<::std::string::String>,
    /// <p>Indicates if the results will be stored. Defaults to <code>SingleUse</code>, if left empty.</p>
    pub intended_use: ::std::option::Option<crate::types::SuggestIntendedUse>,
    /// <p>Optional: The API key to be used for authorization. Either an API key or valid SigV4 signature must be provided when making a request.</p>
    pub key: ::std::option::Option<::std::string::String>,
}
impl SuggestInput {
    /// <p>The free-form text query to match addresses against. This is usually a partially typed address from an end user in an address box or form.</p><note>
    /// <p>The fields <code>QueryText</code>, and <code>QueryID</code> are mutually exclusive.</p>
    /// </note>
    pub fn query_text(&self) -> ::std::option::Option<&str> {
        self.query_text.as_deref()
    }
    /// <p>An optional limit for the number of results returned in a single call.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>Maximum number of query terms to be returned for use with a search text query.</p>
    pub fn max_query_refinements(&self) -> ::std::option::Option<i32> {
        self.max_query_refinements
    }
    /// <p>The position, in longitude and latitude, that the results should be close to. Typically, place results returned are ranked higher the closer they are to this position. Stored in <code>\[lng, lat\]</code> and in the WSG84 format.</p><note>
    /// <p>The fields <code>BiasPosition</code>, <code>FilterBoundingBox</code>, and <code>FilterCircle</code> are mutually exclusive.</p>
    /// </note>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.bias_position.is_none()`.
    pub fn bias_position(&self) -> &[f64] {
        self.bias_position.as_deref().unwrap_or_default()
    }
    /// <p>A structure which contains a set of inclusion/exclusion properties that results must possess in order to be returned as a result.</p>
    pub fn filter(&self) -> ::std::option::Option<&crate::types::SuggestFilter> {
        self.filter.as_ref()
    }
    /// <p>A list of optional additional parameters, such as time zone, that can be requested for each result.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.additional_features.is_none()`.
    pub fn additional_features(&self) -> &[crate::types::SuggestAdditionalFeature] {
        self.additional_features.as_deref().unwrap_or_default()
    }
    /// <p>A list of <a href="https://en.wikipedia.org/wiki/IETF_language_tag">BCP 47</a> compliant language codes for the results to be rendered in. If there is no data for the result in the requested language, data will be returned in the default language for the entry.</p>
    pub fn language(&self) -> ::std::option::Option<&str> {
        self.language.as_deref()
    }
    /// <p>The alpha-2 or alpha-3 character code for the political view of a country. The political view applies to the results of the request to represent unresolved territorial claims through the point of view of the specified country.</p>
    pub fn political_view(&self) -> ::std::option::Option<&str> {
        self.political_view.as_deref()
    }
    /// <p>Indicates if the results will be stored. Defaults to <code>SingleUse</code>, if left empty.</p>
    pub fn intended_use(&self) -> ::std::option::Option<&crate::types::SuggestIntendedUse> {
        self.intended_use.as_ref()
    }
    /// <p>Optional: The API key to be used for authorization. Either an API key or valid SigV4 signature must be provided when making a request.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
}
impl ::std::fmt::Debug for SuggestInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SuggestInput");
        formatter.field("query_text", &"*** Sensitive Data Redacted ***");
        formatter.field("max_results", &self.max_results);
        formatter.field("max_query_refinements", &self.max_query_refinements);
        formatter.field("bias_position", &"*** Sensitive Data Redacted ***");
        formatter.field("filter", &self.filter);
        formatter.field("additional_features", &self.additional_features);
        formatter.field("language", &self.language);
        formatter.field("political_view", &"*** Sensitive Data Redacted ***");
        formatter.field("intended_use", &self.intended_use);
        formatter.field("key", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl SuggestInput {
    /// Creates a new builder-style object to manufacture [`SuggestInput`](crate::operation::suggest::SuggestInput).
    pub fn builder() -> crate::operation::suggest::builders::SuggestInputBuilder {
        crate::operation::suggest::builders::SuggestInputBuilder::default()
    }
}

/// A builder for [`SuggestInput`](crate::operation::suggest::SuggestInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct SuggestInputBuilder {
    pub(crate) query_text: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) max_query_refinements: ::std::option::Option<i32>,
    pub(crate) bias_position: ::std::option::Option<::std::vec::Vec<f64>>,
    pub(crate) filter: ::std::option::Option<crate::types::SuggestFilter>,
    pub(crate) additional_features: ::std::option::Option<::std::vec::Vec<crate::types::SuggestAdditionalFeature>>,
    pub(crate) language: ::std::option::Option<::std::string::String>,
    pub(crate) political_view: ::std::option::Option<::std::string::String>,
    pub(crate) intended_use: ::std::option::Option<crate::types::SuggestIntendedUse>,
    pub(crate) key: ::std::option::Option<::std::string::String>,
}
impl SuggestInputBuilder {
    /// <p>The free-form text query to match addresses against. This is usually a partially typed address from an end user in an address box or form.</p><note>
    /// <p>The fields <code>QueryText</code>, and <code>QueryID</code> are mutually exclusive.</p>
    /// </note>
    /// This field is required.
    pub fn query_text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.query_text = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The free-form text query to match addresses against. This is usually a partially typed address from an end user in an address box or form.</p><note>
    /// <p>The fields <code>QueryText</code>, and <code>QueryID</code> are mutually exclusive.</p>
    /// </note>
    pub fn set_query_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.query_text = input;
        self
    }
    /// <p>The free-form text query to match addresses against. This is usually a partially typed address from an end user in an address box or form.</p><note>
    /// <p>The fields <code>QueryText</code>, and <code>QueryID</code> are mutually exclusive.</p>
    /// </note>
    pub fn get_query_text(&self) -> &::std::option::Option<::std::string::String> {
        &self.query_text
    }
    /// <p>An optional limit for the number of results returned in a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>An optional limit for the number of results returned in a single call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>An optional limit for the number of results returned in a single call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>Maximum number of query terms to be returned for use with a search text query.</p>
    pub fn max_query_refinements(mut self, input: i32) -> Self {
        self.max_query_refinements = ::std::option::Option::Some(input);
        self
    }
    /// <p>Maximum number of query terms to be returned for use with a search text query.</p>
    pub fn set_max_query_refinements(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_query_refinements = input;
        self
    }
    /// <p>Maximum number of query terms to be returned for use with a search text query.</p>
    pub fn get_max_query_refinements(&self) -> &::std::option::Option<i32> {
        &self.max_query_refinements
    }
    /// Appends an item to `bias_position`.
    ///
    /// To override the contents of this collection use [`set_bias_position`](Self::set_bias_position).
    ///
    /// <p>The position, in longitude and latitude, that the results should be close to. Typically, place results returned are ranked higher the closer they are to this position. Stored in <code>\[lng, lat\]</code> and in the WSG84 format.</p><note>
    /// <p>The fields <code>BiasPosition</code>, <code>FilterBoundingBox</code>, and <code>FilterCircle</code> are mutually exclusive.</p>
    /// </note>
    pub fn bias_position(mut self, input: f64) -> Self {
        let mut v = self.bias_position.unwrap_or_default();
        v.push(input);
        self.bias_position = ::std::option::Option::Some(v);
        self
    }
    /// <p>The position, in longitude and latitude, that the results should be close to. Typically, place results returned are ranked higher the closer they are to this position. Stored in <code>\[lng, lat\]</code> and in the WSG84 format.</p><note>
    /// <p>The fields <code>BiasPosition</code>, <code>FilterBoundingBox</code>, and <code>FilterCircle</code> are mutually exclusive.</p>
    /// </note>
    pub fn set_bias_position(mut self, input: ::std::option::Option<::std::vec::Vec<f64>>) -> Self {
        self.bias_position = input;
        self
    }
    /// <p>The position, in longitude and latitude, that the results should be close to. Typically, place results returned are ranked higher the closer they are to this position. Stored in <code>\[lng, lat\]</code> and in the WSG84 format.</p><note>
    /// <p>The fields <code>BiasPosition</code>, <code>FilterBoundingBox</code>, and <code>FilterCircle</code> are mutually exclusive.</p>
    /// </note>
    pub fn get_bias_position(&self) -> &::std::option::Option<::std::vec::Vec<f64>> {
        &self.bias_position
    }
    /// <p>A structure which contains a set of inclusion/exclusion properties that results must possess in order to be returned as a result.</p>
    pub fn filter(mut self, input: crate::types::SuggestFilter) -> Self {
        self.filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure which contains a set of inclusion/exclusion properties that results must possess in order to be returned as a result.</p>
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::SuggestFilter>) -> Self {
        self.filter = input;
        self
    }
    /// <p>A structure which contains a set of inclusion/exclusion properties that results must possess in order to be returned as a result.</p>
    pub fn get_filter(&self) -> &::std::option::Option<crate::types::SuggestFilter> {
        &self.filter
    }
    /// Appends an item to `additional_features`.
    ///
    /// To override the contents of this collection use [`set_additional_features`](Self::set_additional_features).
    ///
    /// <p>A list of optional additional parameters, such as time zone, that can be requested for each result.</p>
    pub fn additional_features(mut self, input: crate::types::SuggestAdditionalFeature) -> Self {
        let mut v = self.additional_features.unwrap_or_default();
        v.push(input);
        self.additional_features = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of optional additional parameters, such as time zone, that can be requested for each result.</p>
    pub fn set_additional_features(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SuggestAdditionalFeature>>) -> Self {
        self.additional_features = input;
        self
    }
    /// <p>A list of optional additional parameters, such as time zone, that can be requested for each result.</p>
    pub fn get_additional_features(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SuggestAdditionalFeature>> {
        &self.additional_features
    }
    /// <p>A list of <a href="https://en.wikipedia.org/wiki/IETF_language_tag">BCP 47</a> compliant language codes for the results to be rendered in. If there is no data for the result in the requested language, data will be returned in the default language for the entry.</p>
    pub fn language(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.language = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A list of <a href="https://en.wikipedia.org/wiki/IETF_language_tag">BCP 47</a> compliant language codes for the results to be rendered in. If there is no data for the result in the requested language, data will be returned in the default language for the entry.</p>
    pub fn set_language(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.language = input;
        self
    }
    /// <p>A list of <a href="https://en.wikipedia.org/wiki/IETF_language_tag">BCP 47</a> compliant language codes for the results to be rendered in. If there is no data for the result in the requested language, data will be returned in the default language for the entry.</p>
    pub fn get_language(&self) -> &::std::option::Option<::std::string::String> {
        &self.language
    }
    /// <p>The alpha-2 or alpha-3 character code for the political view of a country. The political view applies to the results of the request to represent unresolved territorial claims through the point of view of the specified country.</p>
    pub fn political_view(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.political_view = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The alpha-2 or alpha-3 character code for the political view of a country. The political view applies to the results of the request to represent unresolved territorial claims through the point of view of the specified country.</p>
    pub fn set_political_view(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.political_view = input;
        self
    }
    /// <p>The alpha-2 or alpha-3 character code for the political view of a country. The political view applies to the results of the request to represent unresolved territorial claims through the point of view of the specified country.</p>
    pub fn get_political_view(&self) -> &::std::option::Option<::std::string::String> {
        &self.political_view
    }
    /// <p>Indicates if the results will be stored. Defaults to <code>SingleUse</code>, if left empty.</p>
    pub fn intended_use(mut self, input: crate::types::SuggestIntendedUse) -> Self {
        self.intended_use = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates if the results will be stored. Defaults to <code>SingleUse</code>, if left empty.</p>
    pub fn set_intended_use(mut self, input: ::std::option::Option<crate::types::SuggestIntendedUse>) -> Self {
        self.intended_use = input;
        self
    }
    /// <p>Indicates if the results will be stored. Defaults to <code>SingleUse</code>, if left empty.</p>
    pub fn get_intended_use(&self) -> &::std::option::Option<crate::types::SuggestIntendedUse> {
        &self.intended_use
    }
    /// <p>Optional: The API key to be used for authorization. Either an API key or valid SigV4 signature must be provided when making a request.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Optional: The API key to be used for authorization. Either an API key or valid SigV4 signature must be provided when making a request.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>Optional: The API key to be used for authorization. Either an API key or valid SigV4 signature must be provided when making a request.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    /// Consumes the builder and constructs a [`SuggestInput`](crate::operation::suggest::SuggestInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::suggest::SuggestInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::suggest::SuggestInput {
            query_text: self.query_text,
            max_results: self.max_results,
            max_query_refinements: self.max_query_refinements,
            bias_position: self.bias_position,
            filter: self.filter,
            additional_features: self.additional_features,
            language: self.language,
            political_view: self.political_view,
            intended_use: self.intended_use,
            key: self.key,
        })
    }
}
impl ::std::fmt::Debug for SuggestInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SuggestInputBuilder");
        formatter.field("query_text", &"*** Sensitive Data Redacted ***");
        formatter.field("max_results", &self.max_results);
        formatter.field("max_query_refinements", &self.max_query_refinements);
        formatter.field("bias_position", &"*** Sensitive Data Redacted ***");
        formatter.field("filter", &self.filter);
        formatter.field("additional_features", &self.additional_features);
        formatter.field("language", &self.language);
        formatter.field("political_view", &"*** Sensitive Data Redacted ***");
        formatter.field("intended_use", &self.intended_use);
        formatter.field("key", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
