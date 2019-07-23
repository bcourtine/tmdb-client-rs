# GenresApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_movie_genres_list**](GenresApi.md#get_all_movie_genres_list) | **get** /genre/movie/list | Get Movie List
[**get_all_tv_genres_list**](GenresApi.md#get_all_tv_genres_list) | **get** /genre/tv/list | Get TV List
[**get_movies_by_genre_paginated**](GenresApi.md#get_movies_by_genre_paginated) | **get** /genre/{genre_id}/movies | Get Movies



## get_all_movie_genres_list

> crate::models::GenresList get_all_movie_genres_list(language)
Get Movie List

Get the list of official genres for movies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** |  |  | 

### Return type

[**crate::models::GenresList**](GenresList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_tv_genres_list

> crate::models::GenresList get_all_tv_genres_list(language)
Get TV List

Get the list of official genres for TV shows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::GenresList**](GenresList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movies_by_genre_paginated

> crate::models::MoviePaginated get_movies_by_genre_paginated(genre_id, language, include_adult, sort_by)
Get Movies

Get a list of movies by genre id.  We highly recommend using [movie discover](#endpoint:p5NyoR7dM842S8G9j) instead of this method as it is much more flexible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**genre_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**include_adult** | **bool** | Choose whether to inlcude adult (pornography) content in the results. |  | [default to false]
**sort_by** | **String** | Sort the results. |  | 

### Return type

[**crate::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

