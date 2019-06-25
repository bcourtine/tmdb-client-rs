# \GenresApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_movie_genres_list**](GenresApi.md#get_all_movie_genres_list) | **get** /genre/movie/list | Get Movie List
[**get_all_tv_genres_list**](GenresApi.md#get_all_tv_genres_list) | **get** /genre/tv/list | Get TV List
[**get_movies_by_genre_paginated**](GenresApi.md#get_movies_by_genre_paginated) | **get** /genre/{genre_id}/movies | Get Movies



## get_all_movie_genres_list

> ::models::GenresList get_all_movie_genres_list(ctx, optional)
Get Movie List

Get the list of official genres for movies.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **language** | **String**|  | 

### Return type

[**::models::GenresList**](genres-list.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_tv_genres_list

> ::models::GenresList get_all_tv_genres_list(ctx, optional)
Get TV List

Get the list of official genres for TV shows.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::GenresList**](genres-list.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movies_by_genre_paginated

> ::models::MoviePaginated get_movies_by_genre_paginated(ctx, genre_id, optional)
Get Movies

Get a list of movies by genre id.  We highly recommend using [movie discover](#endpoint:p5NyoR7dM842S8G9j) instead of this method as it is much more flexible.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **genre_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **genre_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **include_adult** | **bool**| Choose whether to inlcude adult (pornography) content in the results. | [default to false]
 **sort_by** | **String**| Sort the results. | 

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

