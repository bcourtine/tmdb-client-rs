# \SearchApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_search_collection_paginated**](SearchApi.md#get_search_collection_paginated) | **get** /search/collection | Search Collections
[**get_search_company_paginated**](SearchApi.md#get_search_company_paginated) | **get** /search/company | Search Companies
[**get_search_keyword_paginated**](SearchApi.md#get_search_keyword_paginated) | **get** /search/keyword | Search Keywords
[**get_search_movie_paginated**](SearchApi.md#get_search_movie_paginated) | **get** /search/movie | Search Movies
[**get_search_multi_paginated**](SearchApi.md#get_search_multi_paginated) | **get** /search/multi | Multi Search
[**get_search_person_paginated**](SearchApi.md#get_search_person_paginated) | **get** /search/person | Search People
[**get_search_tv_paginated**](SearchApi.md#get_search_tv_paginated) | **get** /search/tv | Search TV Shows



## get_search_collection_paginated

> crate::models::CollectionPaginated get_search_collection_paginated(query, language, page)
Search Collections

Search for collections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Pass a text query to search. This value should be URI encoded. | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::CollectionPaginated**](CollectionPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_company_paginated

> crate::models::CompanyPaginated get_search_company_paginated(query, page)
Search Companies

Search for companies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Pass a text query to search. This value should be URI encoded. | Required | 
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::CompanyPaginated**](CompanyPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_keyword_paginated

> crate::models::KeywordPaginated get_search_keyword_paginated(query, page)
Search Keywords

Search for keywords.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Pass a text query to search. This value should be URI encoded. | Required | 
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::KeywordPaginated**](KeywordPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_movie_paginated

> crate::models::MoviePaginated get_search_movie_paginated(query, year, primary_release_year, language, page, include_adult, region)
Search Movies

Search for movies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Pass a text query to search. This value should be URI encoded. | Required | 
**year** | **i32** |  |  | 
**primary_release_year** | **i32** |  |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]
**include_adult** | **bool** | Choose whether to inlcude adult (pornography) content in the results. |  | [default to false]
**region** | **String** | Specify a ISO 3166-1 code to filter release dates. |  | 

### Return type

[**crate::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_multi_paginated

> crate::models::SearchMultiResultsPaginated get_search_multi_paginated(query, language, page, include_adult, region)
Multi Search

Search multiple models in a single request. Multi search currently supports searching for movies, tv shows and people in a single request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Pass a text query to search. This value should be URI encoded. | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]
**include_adult** | **bool** | Choose whether to inlcude adult (pornography) content in the results. |  | [default to false]
**region** | **String** | Specify a ISO 3166-1 code to filter release dates. |  | 

### Return type

[**crate::models::SearchMultiResultsPaginated**](SearchMultiResultsPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_person_paginated

> crate::models::PersonPaginated get_search_person_paginated(query, language, page, include_adult, region)
Search People

Search for people.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Pass a text query to search. This value should be URI encoded. | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]
**include_adult** | **bool** | Choose whether to inlcude adult (pornography) content in the results. |  | [default to false]
**region** | **String** | Specify a ISO 3166-1 code to filter release dates. |  | 

### Return type

[**crate::models::PersonPaginated**](PersonPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_tv_paginated

> crate::models::TvPaginated get_search_tv_paginated(query, first_air_date_year, language, page)
Search TV Shows

Search for a TV show.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Pass a text query to search. This value should be URI encoded. | Required | 
**first_air_date_year** | **i32** |  |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

