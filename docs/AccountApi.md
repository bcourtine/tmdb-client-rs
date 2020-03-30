# AccountApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_favorite_movies_paginated**](AccountApi.md#get_account_favorite_movies_paginated) | **get** /account/{account_id}/favorite/movies | Get Favorite Movies
[**get_account_favorite_tv_paginated**](AccountApi.md#get_account_favorite_tv_paginated) | **get** /account/{account_id}/favorite/tv | Get Favorite TV Shows
[**get_account_rated_movies_paginated**](AccountApi.md#get_account_rated_movies_paginated) | **get** /account/{account_id}/rated/movies | Get Rated Movies
[**get_account_rated_tv_episodes_paginated**](AccountApi.md#get_account_rated_tv_episodes_paginated) | **get** /account/{account_id}/rated/tv/episodes | Get Rated TV Episodes
[**get_account_rated_tv_paginated**](AccountApi.md#get_account_rated_tv_paginated) | **get** /account/{account_id}/rated/tv | Get Rated TV Shows
[**get_account_watchlist_movies_paginated**](AccountApi.md#get_account_watchlist_movies_paginated) | **get** /account/{account_id}/watchlist/movies | Get Movie Watchlist
[**get_account_watchlist_tv_paginated**](AccountApi.md#get_account_watchlist_tv_paginated) | **get** /account/{account_id}/watchlist/tv | Get TV Show Watchlist
[**get_current_account_details**](AccountApi.md#get_current_account_details) | **get** /account | Get Details
[**get_current_account_lists_paginated**](AccountApi.md#get_current_account_lists_paginated) | **get** /account/{account_id}/lists | Get Created Lists
[**post_account_favorite**](AccountApi.md#post_account_favorite) | **post** /account/{account_id}/favorite | Mark as Favorite
[**post_account_watchlist**](AccountApi.md#post_account_watchlist) | **post** /account/{account_id}/watchlist | Add to Watchlist



## get_account_favorite_movies_paginated

> crate::models::MoviePaginated get_account_favorite_movies_paginated(account_id, session_id, language, sort_by)
Get Favorite Movies

Get the list of your favorite movies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** |  | Required | 
**session_id** | **String** |  | Required | 
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


## get_account_favorite_tv_paginated

> crate::models::TvPaginated get_account_favorite_tv_paginated(account_id, session_id, language, sort_by)
Get Favorite TV Shows

Get the list of your favorite TV shows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** |  | Required | 
**session_id** | **String** |  | Required | 
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


## get_account_rated_movies_paginated

> crate::models::MoviePaginated get_account_rated_movies_paginated(account_id, session_id, language, sort_by)
Get Rated Movies

Get a list of all the movies you have rated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** |  | Required | 
**session_id** | **String** |  | Required | 
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


## get_account_rated_tv_episodes_paginated

> crate::models::TvEpisodesPaginated get_account_rated_tv_episodes_paginated(account_id, session_id, language, sort_by)
Get Rated TV Episodes

Get a list of all the TV episodes you have rated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | Required | 
**session_id** | **String** |  | Required | 
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


## get_account_rated_tv_paginated

> crate::models::TvPaginated get_account_rated_tv_paginated(account_id, session_id, language, sort_by)
Get Rated TV Shows

Get a list of all the TV shows you have rated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** |  | Required | 
**session_id** | **String** |  | Required | 
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


## get_account_watchlist_movies_paginated

> crate::models::MoviePaginated get_account_watchlist_movies_paginated(account_id, session_id, language, sort_by)
Get Movie Watchlist

Get a list of all the movies you have added to your watchlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** |  | Required | 
**session_id** | **String** |  | Required | 
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


## get_account_watchlist_tv_paginated

> crate::models::TvPaginated get_account_watchlist_tv_paginated(account_id, session_id, language, sort_by)
Get TV Show Watchlist

Get a list of all the TV shows you have added to your watchlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** |  | Required | 
**session_id** | **String** |  | Required | 
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


## get_current_account_details

> crate::models::AccountDetails get_current_account_details(session_id)
Get Details

Get your account details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | Required | 

### Return type

[**crate::models::AccountDetails**](AccountDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_account_lists_paginated

> crate::models::ListsPaginated get_current_account_lists_paginated(account_id, session_id, language)
Get Created Lists

Get all of the lists created by an account. Will invlude private lists if you are the owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** |  | Required | 
**session_id** | **String** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::ListsPaginated**](ListsPaginated.md.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_account_favorite

> crate::models::StatusCodeMessage post_account_favorite(account_id, session_id, content_type, body)
Mark as Favorite

This method allows you to mark a movie or TV show as a favorite item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** |  | Required | 
**session_id** | **String** |  | Required | 
**content_type** | **String** |  | Required | [default to application/json;charset=utf-8]
**body** | [**MediaFavoriteBody**](MediaFavoriteBody.md) |  |  | 

### Return type

[**crate::models::StatusCodeMessage**](StatusCodeMessage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_account_watchlist

> crate::models::StatusCodeMessage post_account_watchlist(account_id, content_type, session_id, body)
Add to Watchlist

Add a movie or TV show to your watchlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** |  | Required | 
**content_type** | **String** |  | Required | [default to application/json;charset=utf-8]
**session_id** | **String** |  | Required | 
**body** | [**MediaWatchlistBody**](MediaWatchlistBody.md) |  |  | 

### Return type

[**crate::models::StatusCodeMessage**](StatusCodeMessage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

