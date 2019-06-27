# \KeywordsApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_keyword_details**](KeywordsApi.md#get_keyword_details) | **get** /keyword/{keyword_id} | Get Details
[**get_movies_by_keyword_paginated**](KeywordsApi.md#get_movies_by_keyword_paginated) | **get** /keyword/{keyword_id}/movies | Get Movies



## get_keyword_details

> ::models::Keyword get_keyword_details(ctx, keyword_id)
Get Details

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **keyword_id** | **i32**|  | 

### Return type

[**::models::Keyword**](Keyword.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movies_by_keyword_paginated

> ::models::MoviePaginated get_movies_by_keyword_paginated(ctx, keyword_id, optional)
Get Movies

Get the movies that belong to a keyword.  We highly recommend using [movie discover](#endpoint:p5NyoR7dM842S8G9j) instead of this method as it is much more flexible.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **keyword_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **keyword_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **include_adult** | **bool**| Choose whether to inlcude adult (pornography) content in the results. | [default to false]

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

