# \AccountApi

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

> ::models::MoviePaginated get_account_favorite_movies_paginated(ctx, account_id, session_id, optional)
Get Favorite Movies

Get the list of your favorite movies.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account_id** | **i32**|  | 
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **i32**|  | 
 **session_id** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **sort_by** | **String**| Sort the results. | 

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_favorite_tv_paginated

> ::models::TvPaginated get_account_favorite_tv_paginated(ctx, account_id, session_id, optional)
Get Favorite TV Shows

Get the list of your favorite TV shows.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account_id** | **i32**|  | 
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **i32**|  | 
 **session_id** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **sort_by** | **String**| Sort the results. | 

### Return type

[**::models::TvPaginated**](tv-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_rated_movies_paginated

> ::models::MoviePaginated get_account_rated_movies_paginated(ctx, account_id, session_id, optional)
Get Rated Movies

Get a list of all the movies you have rated.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account_id** | **i32**|  | 
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **i32**|  | 
 **session_id** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **sort_by** | **String**| Sort the results. | 

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_rated_tv_episodes_paginated

> ::models::TvEpisodesPaginated get_account_rated_tv_episodes_paginated(ctx, account_id, session_id, optional)
Get Rated TV Episodes

Get a list of all the TV episodes you have rated.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account_id** | **String**|  | 
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **String**|  | 
 **session_id** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **sort_by** | **String**| Sort the results. | 

### Return type

[**::models::TvEpisodesPaginated**](tv-episodes-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_rated_tv_paginated

> ::models::TvPaginated get_account_rated_tv_paginated(ctx, account_id, session_id, optional)
Get Rated TV Shows

Get a list of all the TV shows you have rated.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account_id** | **i32**|  | 
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **i32**|  | 
 **session_id** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **sort_by** | **String**| Sort the results. | 

### Return type

[**::models::TvPaginated**](tv-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_watchlist_movies_paginated

> ::models::MoviePaginated get_account_watchlist_movies_paginated(ctx, account_id, session_id, optional)
Get Movie Watchlist

Get a list of all the movies you have added to your watchlist.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account_id** | **i32**|  | 
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **i32**|  | 
 **session_id** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **sort_by** | **String**| Sort the results. | 

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_watchlist_tv_paginated

> ::models::TvPaginated get_account_watchlist_tv_paginated(ctx, account_id, session_id, optional)
Get TV Show Watchlist

Get a list of all the TV shows you have added to your watchlist.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account_id** | **i32**|  | 
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **i32**|  | 
 **session_id** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **sort_by** | **String**| Sort the results. | 

### Return type

[**::models::TvPaginated**](tv-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_account_details

> ::models::AccountDetails get_current_account_details(ctx, session_id)
Get Details

Get your account details.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **session_id** | **String**|  | 

### Return type

[**::models::AccountDetails**](account-details.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_account_lists_paginated

> ::models::ListsPaginated get_current_account_lists_paginated(ctx, account_id, session_id, optional)
Get Created Lists

Get all of the lists created by an account. Will invlude private lists if you are the owner.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account_id** | **i32**|  | 
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **i32**|  | 
 **session_id** | **String**|  | 
 **api_key** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::ListsPaginated**](lists-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_account_favorite

> ::models::InlineResponse401 post_account_favorite(ctx, account_id, session_id, content_type, optional)
Mark as Favorite

This method allows you to mark a movie or TV show as a favorite item.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account_id** | **i32**|  | 
  **session_id** | **String**|  | 
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **i32**|  | 
 **session_id** | **String**|  | 
 **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **body** | [**InlineObject**](InlineObject.md)|  | 

### Return type

[**::models::InlineResponse401**](inline_response_401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_account_watchlist

> ::models::InlineResponse401 post_account_watchlist(ctx, account_id, content_type, session_id, optional)
Add to Watchlist

Add a movie or TV show to your watchlist.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account_id** | **i32**|  | 
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **i32**|  | 
 **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **session_id** | **String**|  | 
 **body** | [**InlineObject1**](InlineObject1.md)|  | 

### Return type

[**::models::InlineResponse401**](inline_response_401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

