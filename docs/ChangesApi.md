# ChangesApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_movie_changes_paginated**](ChangesApi.md#get_movie_changes_paginated) | **get** /movie/changes | Get Movie Change List
[**get_person_changes_paginated**](ChangesApi.md#get_person_changes_paginated) | **get** /person/changes | Get Person Change List
[**get_tv_changes_paginated**](ChangesApi.md#get_tv_changes_paginated) | **get** /tv/changes | Get TV Change List



## get_movie_changes_paginated

> crate::models::ChangesPaginated get_movie_changes_paginated(start_date, end_date)
Get Movie Change List

Get  a list of all of the movie ids that have been changed in the past 24 hours.  You can query it for up to 14 days worth of changed IDs at a time with the `start_date` and `end_date` query parameters. 100 items are returned per page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** | Filter the results with a start date. |  | 
**end_date** | **String** | Filter the results with a end date. |  | 
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::ChangesPaginated**](ChangesPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_changes_paginated

> crate::models::ChangesPaginated get_person_changes_paginated(start_date, end_date)
Get Person Change List

Get a list of all of the person ids that have been changed in the past 24 hours.  You can query it for up to 14 days worth of changed IDs at a time with the `start_date` and `end_date` query parameters. 100 items are returned per page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** | Filter the results with a start date. |  | 
**end_date** | **String** | Filter the results with a end date. |  | 
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::ChangesPaginated**](ChangesPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_changes_paginated

> crate::models::ChangesPaginated get_tv_changes_paginated(start_date, end_date)
Get TV Change List

Get a list of all of the TV show ids that have been changed in the past 24 hours.  You can query it for up to 14 days worth of changed IDs at a time with the `start_date` and `end_date` query parameters. 100 items are returned per page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** | Filter the results with a start date. |  | 
**end_date** | **String** | Filter the results with a end date. |  | 
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::ChangesPaginated**](ChangesPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

