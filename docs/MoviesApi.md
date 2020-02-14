# MoviesApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_movie_rating**](MoviesApi.md#delete_movie_rating) | **delete** /movie/{movie_id}/rating | Delete Rating
[**get_movie_account_states**](MoviesApi.md#get_movie_account_states) | **get** /movie/{movie_id}/account_states | Get Account States
[**get_movie_alternative_titles_list**](MoviesApi.md#get_movie_alternative_titles_list) | **get** /movie/{movie_id}/alternative_titles | Get Alternative Titles
[**get_movie_changes_list**](MoviesApi.md#get_movie_changes_list) | **get** /movie/{movie_id}/changes | Get Changes
[**get_movie_credits**](MoviesApi.md#get_movie_credits) | **get** /movie/{movie_id}/credits | Get Credits
[**get_movie_details**](MoviesApi.md#get_movie_details) | **get** /movie/{movie_id} | Get Details
[**get_movie_external_ids**](MoviesApi.md#get_movie_external_ids) | **get** /movie/{movie_id}/external_ids | Get External IDs
[**get_movie_images**](MoviesApi.md#get_movie_images) | **get** /movie/{movie_id}/images | Get Images
[**get_movie_keywords_list**](MoviesApi.md#get_movie_keywords_list) | **get** /movie/{movie_id}/keywords | Get Keywords
[**get_movie_latest_details**](MoviesApi.md#get_movie_latest_details) | **get** /movie/latest | Get Latest
[**get_movie_lists_paginated**](MoviesApi.md#get_movie_lists_paginated) | **get** /movie/{movie_id}/lists | Get Lists
[**get_movie_now_playing_paginated**](MoviesApi.md#get_movie_now_playing_paginated) | **get** /movie/now_playing | Get Now Playing
[**get_movie_popular_paginated**](MoviesApi.md#get_movie_popular_paginated) | **get** /movie/popular | Get Popular
[**get_movie_recommendations_paginated**](MoviesApi.md#get_movie_recommendations_paginated) | **get** /movie/{movie_id}/recommendations | Get Recommendations
[**get_movie_release_dates**](MoviesApi.md#get_movie_release_dates) | **get** /movie/{movie_id}/release_dates | Get Release Dates
[**get_movie_reviews_paginated**](MoviesApi.md#get_movie_reviews_paginated) | **get** /movie/{movie_id}/reviews | Get Reviews
[**get_movie_similar_paginated**](MoviesApi.md#get_movie_similar_paginated) | **get** /movie/{movie_id}/similar | Get Similar Movies
[**get_movie_top_rated_paginated**](MoviesApi.md#get_movie_top_rated_paginated) | **get** /movie/top_rated | Get Top Rated
[**get_movie_translations_list**](MoviesApi.md#get_movie_translations_list) | **get** /movie/{movie_id}/translations | Get Translations
[**get_movie_upcoming_paginated**](MoviesApi.md#get_movie_upcoming_paginated) | **get** /movie/upcoming | Get Upcoming
[**get_movie_videos_list**](MoviesApi.md#get_movie_videos_list) | **get** /movie/{movie_id}/videos | Get Videos
[**post_movie_rating**](MoviesApi.md#post_movie_rating) | **post** /movie/{movie_id}/rating | Rate Movie



## delete_movie_rating

> crate::models::StatusCodeMessage delete_movie_rating(movie_id, content_type, guest_session_id, session_id)
Delete Rating

Remove your rating for a movie.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**content_type** | **String** |  | Required | [default to application/json;charset=utf-8]
**guest_session_id** | **String** |  |  | 
**session_id** | **String** |  |  | 

### Return type

[**crate::models::StatusCodeMessage**](StatusCodeMessage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_account_states

> crate::models::AccountStates get_movie_account_states(movie_id, session_id, guest_session_id)
Get Account States

Grab the following account states for a session:  - Movie rating - If it belongs to your watchlist - If it belongs to your favourite list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **String** |  | Required | 
**session_id** | **String** |  |  | 
**guest_session_id** | **String** |  |  | 

### Return type

[**crate::models::AccountStates**](AccountStates.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_alternative_titles_list

> crate::models::AlternativeTitlesList get_movie_alternative_titles_list(movie_id, country)
Get Alternative Titles

Get all of the alternative titles for a movie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**country** | **String** |  |  | 

### Return type

[**crate::models::MovieAlternativeTitlesList**](MovieAlternativeTitlesList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_changes_list

> crate::models::ChangeDetails get_movie_changes_list(movie_id, start_date, end_date, page)
Get Changes

Get the changes for a movie. By default only the last 24 hours are returned.  You can query up to 14 days in a single query by using the `start_date` and `end_date` query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **String** |  | Required | 
**start_date** | **String** | Filter the results with a start date. |  | 
**end_date** | **String** | Filter the results with a end date. |  | 
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::ChangeDetails**](ChangeDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_credits

> crate::models::Credits get_movie_credits(movie_id)
Get Credits

Get the cast and crew for a movie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 

### Return type

[**crate::models::Credits**](Credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_details

> crate::models::MovieDetails get_movie_details(movie_id, include_image_language, language, append_to_response)
Get Details

Get the primary information about a movie.  Supports `append_to_response`. Read more about this [here](#docTextSection:JdZq8ctmcxNqyLQjp).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**include_image_language** | **String** | Pass a ISO 639-1 value to get additional images (cf. https://developers.themoviedb.org/3/getting-started/image-languages). |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**append_to_response** | **String** | Append requests within the same namespace to the response. |  | 

### Return type

[**crate::models::MovieDetails**](MovieDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_external_ids

> crate::models::ExternalIds get_movie_external_ids(movie_id)
Get External IDs

Get the external ids for a movie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 

### Return type

[**crate::models::ExternalIds**](ExternalIds.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_images

> crate::models::Images get_movie_images(movie_id, include_image_language, language)
Get Images

Get the images that belong to a movie.  Querying images with a `language` parameter will filter the results. If you want to include a fallback language (especially useful for backdrops) you can use the `include_image_language` parameter. This should be a comma seperated value like so: `include_image_language=en,null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**include_image_language** | **String** | Pass a ISO 639-1 value to get additional images (cf. https://developers.themoviedb.org/3/getting-started/image-languages). |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::Images**](Images.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_keywords_list

> crate::models::KeywordsList get_movie_keywords_list(movie_id, api_key)
Get Keywords

Get the keywords that have been added to a movie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**api_key** | **String** |  |  | 

### Return type

[**crate::models::KeywordsList**](KeywordsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_latest_details

> crate::models::MovieDetails get_movie_latest_details(language)
Get Latest

Get the most newly created movie. This is a live response and will continuously change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::MovieDetails**](MovieDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_lists_paginated

> crate::models::ListsPaginated get_movie_lists_paginated(movie_id, api_key, language, page)
Get Lists

Get a list of lists that this movie belongs to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**api_key** | **String** |  |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::ListsPaginated**](ListsPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_now_playing_paginated

> crate::models::MoviePaginated get_movie_now_playing_paginated(language, page, region)
Get Now Playing

Get a list of movies in theatres. This is a release type query that looks for all movies that have a release type of 2 or 3 within the specified date range.  You can optionally specify a `region` prameter which will narrow the search to only look for theatrical release dates within the specified country.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]
**region** | **String** | Specify a ISO 3166-1 code to filter release dates. |  | 

### Return type

[**crate::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_popular_paginated

> crate::models::MoviePaginated get_movie_popular_paginated(language, page, region)
Get Popular

Get a list of the current popular movies on TMDb. This list updates daily.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]
**region** | **String** | Specify a ISO 3166-1 code to filter release dates. |  | 

### Return type

[**crate::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_recommendations_paginated

> crate::models::MoviePaginated get_movie_recommendations_paginated(movie_id, api_key, language, page)
Get Recommendations

Get a list of recommended movies for a movie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**api_key** | **String** |  |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_release_dates

> crate::models::ReleaseDatesList get_movie_release_dates(movie_id, api_key)
Get Release Dates

Get the release date along with the certification for a movie.  Release dates support different types:  1. Premiere 2. Theatrical (limited) 3. Theatrical 4. Digital 5. Physical 6. TV

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**api_key** | **String** |  |  | 

### Return type

[**crate::models::ReleaseDatesList**](ReleaseDatesList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_reviews_paginated

> crate::models::ReviewsPaginated get_movie_reviews_paginated(movie_id, api_key, language, page)
Get Reviews

Get the user reviews for a movie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**api_key** | **String** |  |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::ReviewsPaginated**](ReviewsPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_similar_paginated

> crate::models::MoviePaginated get_movie_similar_paginated(movie_id, api_key, language, page)
Get Similar Movies

Get a list of similar movies. This is **not** the same as the \"Recommendation\" system you see on the website.  These items are assembled by looking at keywords and genres.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**api_key** | **String** |  |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_top_rated_paginated

> crate::models::MoviePaginated get_movie_top_rated_paginated(language, page, region)
Get Top Rated

Get the top rated movies on TMDb.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]
**region** | **String** | Specify a ISO 3166-1 code to filter release dates. |  | 

### Return type

[**crate::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_translations_list

> crate::models::TranslationsList get_movie_translations_list(movie_id, api_key)
Get Translations

Get a list of translations that have been created for a movie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**api_key** | **String** |  |  | 

### Return type

[**crate::models::TranslationsList**](TranslationsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_upcoming_paginated

> crate::models::MoviePaginated get_movie_upcoming_paginated(language, page, region)
Get Upcoming

Get a list of upcoming movies in theatres. This is a release type query that looks for all movies that have a release type of 2 or 3 within the specified date range.  You can optionally specify a `region` prameter which will narrow the search to only look for theatrical release dates within the specified country.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]
**region** | **String** | Specify a ISO 3166-1 code to filter release dates. |  | 

### Return type

[**crate::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_videos_list

> crate::models::VideosList get_movie_videos_list(movie_id, api_key, language)
Get Videos

Get the videos that have been added to a movie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **String** |  | Required | 
**api_key** | **String** |  |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::VideosList**](VideosList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_movie_rating

> crate::models::StatusCodeMessage post_movie_rating(movie_id, content_type, guest_session_id, session_id, body)
Rate Movie

Rate a movie.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** |  | Required | 
**content_type** | **String** |  | Required | [default to application/json;charset=utf-8]
**guest_session_id** | **String** |  |  | 
**session_id** | **String** |  |  | 
**body** | [**ValueBody**](ValueBody.md) |  |  | 

### Return type

[**crate::models::StatusCodeMessage**](StatusCodeMessage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

