# \TVApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tv_rating**](TVApi.md#delete_tv_rating) | **delete** /tv/{tv_id}/rating | Delete Rating
[**get_tv_account_states**](TVApi.md#get_tv_account_states) | **get** /tv/{tv_id}/account_states | Get Account States
[**get_tv_airing_today_paginated**](TVApi.md#get_tv_airing_today_paginated) | **get** /tv/airing_today | Get TV Airing Today
[**get_tv_alternative_titles_list**](TVApi.md#get_tv_alternative_titles_list) | **get** /tv/{tv_id}/alternative_titles | Get Alternative Titles
[**get_tv_changes**](TVApi.md#get_tv_changes) | **get** /tv/{tv_id}/changes | Get Changes
[**get_tv_content_ratings_list**](TVApi.md#get_tv_content_ratings_list) | **get** /tv/{tv_id}/content_ratings | Get Content Ratings
[**get_tv_credits**](TVApi.md#get_tv_credits) | **get** /tv/{tv_id}/credits | Get Credits
[**get_tv_details**](TVApi.md#get_tv_details) | **get** /tv/{tv_id} | Get Details
[**get_tv_external_ids**](TVApi.md#get_tv_external_ids) | **get** /tv/{tv_id}/external_ids | Get External IDs
[**get_tv_images**](TVApi.md#get_tv_images) | **get** /tv/{tv_id}/images | Get Images
[**get_tv_keywords_list**](TVApi.md#get_tv_keywords_list) | **get** /tv/{tv_id}/keywords | Get Keywords
[**get_tv_latest_details**](TVApi.md#get_tv_latest_details) | **get** /tv/latest | Get Latest
[**get_tv_on_the_air_paginated**](TVApi.md#get_tv_on_the_air_paginated) | **get** /tv/on_the_air | Get TV On The Air
[**get_tv_popular_paginated**](TVApi.md#get_tv_popular_paginated) | **get** /tv/popular | Get Popular
[**get_tv_recommendations_paginated**](TVApi.md#get_tv_recommendations_paginated) | **get** /tv/{tv_id}/recommendations | Get Recommendations
[**get_tv_similar_paginated**](TVApi.md#get_tv_similar_paginated) | **get** /tv/{tv_id}/similar | Get Similar TV Shows
[**get_tv_top_rated_paginated**](TVApi.md#get_tv_top_rated_paginated) | **get** /tv/top_rated | Get Top Rated
[**get_tv_translations_list**](TVApi.md#get_tv_translations_list) | **get** /tv/{tv_id}/translations | Get Translations
[**get_tv_videos_list**](TVApi.md#get_tv_videos_list) | **get** /tv/{tv_id}/videos | Get Videos
[**post_tv_rating**](TVApi.md#post_tv_rating) | **post** /tv/{tv_id}/rating | Rate TV Show



## delete_tv_rating

> ::models::InlineResponse401 delete_tv_rating(ctx, tv_id, content_type, optional)
Delete Rating

Remove your rating for a TV show.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **guest_session_id** | **String**|  | 
 **session_id** | **String**|  | 

### Return type

[**::models::InlineResponse401**](inline_response_401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_account_states

> ::models::AccountStates get_tv_account_states(ctx, tv_id, optional)
Get Account States

Grab the following account states for a session:  - TV show rating - If it belongs to your watchlist - If it belongs to your favourite list

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **guest_session_id** | **String**|  | 
 **session_id** | **String**|  | 

### Return type

[**::models::AccountStates**](AccountStates.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_airing_today_paginated

> ::models::TvPaginated get_tv_airing_today_paginated(ctx, optional)
Get TV Airing Today

Get a list of TV shows that are airing today. This query is purely day based as we do not currently support airing times.  You can specify a [timezone](endpoint:KQ4CDdEoWKJYLkrhS) to offset the day calculation. Without a specified timezone, this query defaults to EST (Eastern Time UTC-05:00).

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
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_alternative_titles_list

> ::models::AlternativeTitlesList get_tv_alternative_titles_list(ctx, tv_id, optional)
Get Alternative Titles

Returns all of the alternative titles for a TV show.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::AlternativeTitlesList**](AlternativeTitlesList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_changes

> ::models::ChangeDetails get_tv_changes(ctx, tv_id, optional)
Get Changes

Get the changes for a TV show. By default only the last 24 hours are returned.  You can query up to 14 days in a single query by using the `start_date` and `end_date` query parameters.  TV show changes are different than movie changes in that there are some edits on seasons and episodes that will create a change entry at the show level. These can be found under the season and episode keys. These keys will contain a `series_id` and `episode_id`. You can use the [season changes](#endpoint:ZqhtyQbJ4YXz8RSya) and [episode changes](#endpoint:FpqrhBzgf2RJv4bHC) methods to look these up individually.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **start_date** | **String**| Filter the results with a start date. | 
 **end_date** | **String**| Filter the results with a end date. | 
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::ChangeDetails**](ChangeDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_content_ratings_list

> ::models::RatingsList get_tv_content_ratings_list(ctx, tv_id, optional)
Get Content Ratings

Get the list of content ratings (certifications) that have been added to a TV show.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::RatingsList**](RatingsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_credits

> ::models::Credits get_tv_credits(ctx, tv_id, optional)
Get Credits

Get the credits (cast and crew) that have been added to a TV show.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::Credits**](Credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_details

> ::models::TvDetails get_tv_details(ctx, tv_id, optional)
Get Details

Get the primary TV show details by id.  Supports `append_to_response`. Read more about this [here](#docTextSection:JdZq8ctmcxNqyLQjp).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **append_to_response** | **String**| Append requests within the same namespace to the response. | 

### Return type

[**::models::TvDetails**](TvDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_external_ids

> ::models::MovieTvExternalIds get_tv_external_ids(ctx, tv_id, optional)
Get External IDs

Get the external ids for a TV show. We currently support the following external sources.  | **External Sources** | | -------------------- | | IMDB ID              | | Freebase MID         | | Freebase ID          | | TVDB ID              | | TVRage ID            |

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::MovieTvExternalIds**](MovieTvExternalIds.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_images

> ::models::Images get_tv_images(ctx, tv_id, optional)
Get Images

Get the images that belong to a TV show.  Querying images with a `language` parameter will filter the results. If you want to include a fallback language (especially useful for backdrops) you can use the `include_image_language` parameter. This should be a comma seperated value like so: `include_image_language=en,null`.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::Images**](Images.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_keywords_list

> ::models::KeywordsList get_tv_keywords_list(ctx, tv_id)
Get Keywords

Get the keywords that have been added to a TV show.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 

### Return type

[**::models::KeywordsList**](KeywordsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_latest_details

> ::models::TvDetails get_tv_latest_details(ctx, optional)
Get Latest

Get the most newly created TV show. This is a live response and will continuously change.

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

[**::models::TvDetails**](TvDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_on_the_air_paginated

> ::models::TvPaginated get_tv_on_the_air_paginated(ctx, optional)
Get TV On The Air

Get a list of shows that are currently on the air.  This query looks for any TV show that has an episode with an air date in the next 7 days.

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
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_popular_paginated

> ::models::TvPaginated get_tv_popular_paginated(ctx, optional)
Get Popular

Get a list of the current popular TV shows on TMDb. This list updates daily.

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
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_recommendations_paginated

> ::models::TvPaginated get_tv_recommendations_paginated(ctx, tv_id, optional)
Get Recommendations

Get the list of TV show recommendations for this item.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_similar_paginated

> ::models::TvPaginated get_tv_similar_paginated(ctx, tv_id, optional)
Get Similar TV Shows

Get a list of similar TV shows. These items are assembled by looking at keywords and genres.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_top_rated_paginated

> ::models::TvPaginated get_tv_top_rated_paginated(ctx, optional)
Get Top Rated

Get a list of the top rated TV shows on TMDb.

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
 **page** | **i32**| Specify which page to query. | [default to 1]

### Return type

[**::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_translations_list

> ::models::Translations get_tv_translations_list(ctx, tv_id, optional)
Get Translations

Get a list of the translations that exist for a TV show.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::Translations**](Translations.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_videos_list

> ::models::VideosList get_tv_videos_list(ctx, tv_id, optional)
Get Videos

Get the videos that have been added to a TV show.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::VideosList**](VideosList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tv_rating

> ::models::InlineResponse401 post_tv_rating(ctx, tv_id, content_type, optional)
Rate TV Show

Rate a TV show.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **guest_session_id** | **String**|  | 
 **session_id** | **String**|  | 
 **body** | [**InlineObject6**](InlineObject6.md)|  | 

### Return type

[**::models::InlineResponse401**](inline_response_401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
