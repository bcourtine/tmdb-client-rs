# \GuestSessionsApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_guest_session_rated_movies_paginated**](GuestSessionsApi.md#get_guest_session_rated_movies_paginated) | **get** /guest_session/{guest_session_id}/rated/movies | Get Rated Movies
[**get_guest_session_rated_tv_episodes_paginated**](GuestSessionsApi.md#get_guest_session_rated_tv_episodes_paginated) | **get** /guest_session/{guest_session_id}/rated/tv/episodes | Get Rated TV Episodes
[**get_guest_session_rated_tv_paginated**](GuestSessionsApi.md#get_guest_session_rated_tv_paginated) | **get** /guest_session/{guest_session_id}/rated/tv | Get Rated TV Shows



## get_guest_session_rated_movies_paginated

> crate::models::MoviePaginated get_guest_session_rated_movies_paginated(guest_session_id, language, sort_by)
Get Rated Movies

Get the rated movies for a guest session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guest_session_id** | **String** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**sort_by** | **String** | Sort the results. |  | 

### Return type

[**crate::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guest_session_rated_tv_episodes_paginated

> crate::models::TvEpisodesPaginated get_guest_session_rated_tv_episodes_paginated(guest_session_id, language, sort_by)
Get Rated TV Episodes

Get the rated TV episodes for a guest session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guest_session_id** | **String** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**sort_by** | **String** | Sort the results. |  | 

### Return type

[**crate::models::TvEpisodesPaginated**](TvEpisodesPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guest_session_rated_tv_paginated

> crate::models::TvPaginated get_guest_session_rated_tv_paginated(guest_session_id, language, sort_by)
Get Rated TV Shows

Get the rated TV shows for a guest session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guest_session_id** | **String** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**sort_by** | **String** | Sort the results. |  | 

### Return type

[**crate::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

