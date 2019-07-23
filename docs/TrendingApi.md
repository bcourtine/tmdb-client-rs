# TrendingApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_trending_paginated**](TrendingApi.md#get_trending_paginated) | **get** /trending/{media_type}/{time_window} | Get Trending



## get_trending_paginated

> crate::models::SearchMultiResultsPaginated get_trending_movies_paginated(media_type, time_window)
Get Trending

Get the daily or weekly trending items. The daily trending list tracks items over the period of a day while items have a 24 hour half life. The weekly list tracks items over a 7 day period, with a 7 day half life.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_type** | [**crate::models::MediaType**](MediaType.md) | all, movie, tv, person | Required | 
**time_window** | [**crate::models::TimeWindow**](TimeWindow.md) | day, week | Required | 

### Return type

[**crate::models::SearchMultiResultsPaginated**](SearchMultiResultsPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
