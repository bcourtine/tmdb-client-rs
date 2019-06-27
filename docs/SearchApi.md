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

> ::models::SearchCollectionResultsPaginated get_search_collection_paginated(ctx, query, optional)
Search Collections

Search for collections.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::SearchCollectionResultsPaginated**](SearchCollectionResultsPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_company_paginated

> ::models::SearchCompanyResultsPaginated get_search_company_paginated(ctx, query, optional)
Search Companies

Search for companies.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::SearchCompanyResultsPaginated**](SearchCompanyResultsPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_keyword_paginated

> ::models::SearchKeywordResultsPaginated get_search_keyword_paginated(ctx, query, optional)
Search Keywords

Search for keywords.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::SearchKeywordResultsPaginated**](SearchKeywordResultsPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_movie_paginated

> ::models::MoviePaginated get_search_movie_paginated(ctx, query, optional)
Search Movies

Search for movies.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **year** | **i32**|  | 
 **primary_release_year** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]
 **include_adult** | **bool**| Choose whether to inlcude adult (pornography) content in the results. | [default to false]
 **region** | **String**| Specify a ISO 3166-1 code to filter release dates. | 

### Return type

[**::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_multi_paginated

> ::models::SearchMultiResultsPaginated get_search_multi_paginated(ctx, query, optional)
Multi Search

Search multiple models in a single request. Multi search currently supports searching for movies, tv shows and people in a single request.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]
 **include_adult** | **bool**| Choose whether to inlcude adult (pornography) content in the results. | [default to false]
 **region** | **String**| Specify a ISO 3166-1 code to filter release dates. | 

### Return type

[**::models::SearchMultiResultsPaginated**](SearchMultiResultsPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_person_paginated

> ::models::SearchPersonResultsPaginated get_search_person_paginated(ctx, query, optional)
Search People

Search for people.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]
 **include_adult** | **bool**| Choose whether to inlcude adult (pornography) content in the results. | [default to false]
 **region** | **String**| Specify a ISO 3166-1 code to filter release dates. | 

### Return type

[**::models::SearchPersonResultsPaginated**](SearchPersonResultsPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_tv_paginated

> ::models::TvPaginated get_search_tv_paginated(ctx, query, optional)
Search TV Shows

Search for a TV show.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**| Pass a text query to search. This value should be URI encoded. | 
 **first_air_date_year** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
