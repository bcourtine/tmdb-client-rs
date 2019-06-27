# \ChangesApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_movie_changes_paginated**](ChangesApi.md#get_movie_changes_paginated) | **get** /movie/changes | Get Movie Change List
[**get_person_changes_paginated**](ChangesApi.md#get_person_changes_paginated) | **get** /person/changes | Get Person Change List
[**get_tv_changes_paginated**](ChangesApi.md#get_tv_changes_paginated) | **get** /tv/changes | Get TV Change List



## get_movie_changes_paginated

> ::models::ChangesPaginated get_movie_changes_paginated(ctx, optional)
Get Movie Change List

Get  a list of all of the movie ids that have been changed in the past 24 hours.  You can query it for up to 14 days worth of changed IDs at a time with the `start_date` and `end_date` query parameters. 100 items are returned per page.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **start_date** | **String**| Filter the results with a start date. | 
 **end_date** | **String**| Filter the results with a end date. | 

### Return type

[**::models::ChangesPaginated**](ChangesPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_changes_paginated

> ::models::ChangesPaginated get_person_changes_paginated(ctx, optional)
Get Person Change List

Get a list of all of the person ids that have been changed in the past 24 hours.  You can query it for up to 14 days worth of changed IDs at a time with the `start_date` and `end_date` query parameters. 100 items are returned per page.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **start_date** | **String**| Filter the results with a start date. | 
 **end_date** | **String**| Filter the results with a end date. | 

### Return type

[**::models::ChangesPaginated**](ChangesPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_changes_paginated

> ::models::ChangesPaginated get_tv_changes_paginated(ctx, optional)
Get TV Change List

Get a list of all of the TV show ids that have been changed in the past 24 hours.  You can query it for up to 14 days worth of changed IDs at a time with the `start_date` and `end_date` query parameters. 100 items are returned per page.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **start_date** | **String**| Filter the results with a start date. | 
 **end_date** | **String**| Filter the results with a end date. | 

### Return type

[**::models::ChangesPaginated**](ChangesPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
