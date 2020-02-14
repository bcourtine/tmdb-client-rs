# TVApi

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
[**get_tv_episode_groups**](TVApi.md#get_tv_episode_groups) | **get** /tv/{tv_id}/episode_groups | Get Episode Groups
[**get_tv_external_ids**](TVApi.md#get_tv_external_ids) | **get** /tv/{tv_id}/external_ids | Get External IDs
[**get_tv_images**](TVApi.md#get_tv_images) | **get** /tv/{tv_id}/images | Get Images
[**get_tv_keywords_list**](TVApi.md#get_tv_keywords_list) | **get** /tv/{tv_id}/keywords | Get Keywords
[**get_tv_latest_details**](TVApi.md#get_tv_latest_details) | **get** /tv/latest | Get Latest
[**get_tv_on_the_air_paginated**](TVApi.md#get_tv_on_the_air_paginated) | **get** /tv/on_the_air | Get TV On The Air
[**get_tv_popular_paginated**](TVApi.md#get_tv_popular_paginated) | **get** /tv/popular | Get Popular
[**get_tv_recommendations_paginated**](TVApi.md#get_tv_recommendations_paginated) | **get** /tv/{tv_id}/recommendations | Get Recommendations
[**get_tv_screened_theatrically**](TVApi.md#get_tv_screened_theatrically) | **get** /tv/{tv_id}/screened_theatrically | Get Screened Theatrically
[**get_tv_similar_paginated**](TVApi.md#get_tv_similar_paginated) | **get** /tv/{tv_id}/similar | Get Similar TV Shows
[**get_tv_top_rated_paginated**](TVApi.md#get_tv_top_rated_paginated) | **get** /tv/top_rated | Get Top Rated
[**get_tv_translations_list**](TVApi.md#get_tv_translations_list) | **get** /tv/{tv_id}/translations | Get Translations
[**get_tv_videos_list**](TVApi.md#get_tv_videos_list) | **get** /tv/{tv_id}/videos | Get Videos
[**post_tv_rating**](TVApi.md#post_tv_rating) | **post** /tv/{tv_id}/rating | Rate TV Show



## delete_tv_rating

> crate::models::StatusCodeMessage delete_tv_rating(tv_id, content_type, guest_session_id, session_id)
Delete Rating

Remove your rating for a TV show.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
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


## get_tv_account_states

> crate::models::AccountStates get_tv_account_states(tv_id, language, guest_session_id, session_id)
Get Account States

Grab the following account states for a session:  - TV show rating - If it belongs to your watchlist - If it belongs to your favourite list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**guest_session_id** | **String** |  |  | 
**session_id** | **String** |  |  | 

### Return type

[**crate::models::AccountStates**](AccountStates.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_airing_today_paginated

> crate::models::TvPaginated get_tv_airing_today_paginated(language, page)
Get TV Airing Today

Get a list of TV shows that are airing today. This query is purely day based as we do not currently support airing times.  You can specify a [timezone](endpoint:KQ4CDdEoWKJYLkrhS) to offset the day calculation. Without a specified timezone, this query defaults to EST (Eastern Time UTC-05:00).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_alternative_titles_list

> crate::models::TvAlternativeTitlesList get_tv_alternative_titles_list(tv_id, language)
Get Alternative Titles

Returns all of the alternative titles for a TV show.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::TvAlternativeTitlesList**](TvAlternativeTitlesList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_changes

> crate::models::ChangeDetails get_tv_changes(tv_id, start_date, end_date, page)
Get Changes

Get the changes for a TV show. By default only the last 24 hours are returned.  You can query up to 14 days in a single query by using the `start_date` and `end_date` query parameters.  TV show changes are different than movie changes in that there are some edits on seasons and episodes that will create a change entry at the show level. These can be found under the season and episode keys. These keys will contain a `series_id` and `episode_id`. You can use the [season changes](#endpoint:ZqhtyQbJ4YXz8RSya) and [episode changes](#endpoint:FpqrhBzgf2RJv4bHC) methods to look these up individually.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
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


## get_tv_content_ratings_list

> crate::models::RatingsList get_tv_content_ratings_list(tv_id, language)
Get Content Ratings

Get the list of content ratings (certifications) that have been added to a TV show.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::RatingsList**](RatingsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_credits

> crate::models::Credits get_tv_credits(tv_id, language)
Get Credits

Get the credits (cast and crew) that have been added to a TV show.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::Credits**](Credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_details

> crate::models::TvDetails get_tv_details(tv_id, include_image_language, language, append_to_response)
Get Details

Get the primary TV show details by id.  Supports `append_to_response`. Read more about this [here](#docTextSection:JdZq8ctmcxNqyLQjp).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**include_image_language** | **String** | Pass a ISO 639-1 value to get additional images (cf. https://developers.themoviedb.org/3/getting-started/image-languages). |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**append_to_response** | **String** | Append requests within the same namespace to the response. |  | 

### Return type

[**crate::models::TvDetails**](TvDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_episode_groups

> crate::models::EpisodeGroupList get_tv_episode_groups(tv_id, language)
Get Episode Groups

Get all of the episode groups that have been created for a TV show.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::EpisodeGroupList**](EpisodeGroupList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_external_ids

> crate::models::ExternalIds get_tv_external_ids(tv_id, language)
Get External IDs

Get the external ids for a TV show. We currently support the following external sources.  | **External Sources** | | -------------------- | | IMDB ID              | | Freebase MID         | | Freebase ID          | | TVDB ID              | | TVRage ID            |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::ExternalIds**](ExternalIds.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_images

> crate::models::Images get_tv_images(tv_id, include_image_language, language)
Get Images

Get the images that belong to a TV show.  Querying images with a `language` parameter will filter the results. If you want to include a fallback language (especially useful for backdrops) you can use the `include_image_language` parameter. This should be a comma seperated value like so: `include_image_language=en,null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
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


## get_tv_keywords_list

> crate::models::KeywordsList get_tv_keywords_list(tv_id)
Get Keywords

Get the keywords that have been added to a TV show.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 

### Return type

[**crate::models::KeywordsList**](KeywordsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_latest_details

> crate::models::TvDetails get_tv_latest_details(language)
Get Latest

Get the most newly created TV show. This is a live response and will continuously change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::TvDetails**](TvDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_on_the_air_paginated

> crate::models::TvPaginated get_tv_on_the_air_paginated(language, page)
Get TV On The Air

Get a list of shows that are currently on the air.  This query looks for any TV show that has an episode with an air date in the next 7 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_popular_paginated

> crate::models::TvPaginated get_tv_popular_paginated(language, page)
Get Popular

Get a list of the current popular TV shows on TMDb. This list updates daily.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_recommendations_paginated

> crate::models::TvPaginated get_tv_recommendations_paginated(tv_id, language, page)
Get Recommendations

Get the list of TV show recommendations for this item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_screened_theatrically

> crate::models::SeasonEpisodeList get_tv_screened_theatrically(tv_id)
Get Screened Theatrically

Get a list of seasons or episodes that have been screened in a film festival or theatre.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 

### Return type

[**crate::models::SeasonEpisodeList**](SeasonEpisodeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_similar_paginated

> crate::models::TvPaginated get_tv_similar_paginated(tv_id, language, page)
Get Similar TV Shows

Get a list of similar TV shows. These items are assembled by looking at keywords and genres.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_top_rated_paginated

> crate::models::TvPaginated get_tv_top_rated_paginated(language, page)
Get Top Rated

Get a list of the top rated TV shows on TMDb.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::TvPaginated**](TvPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_translations_list

> crate::models::TranslationsList get_tv_translations_list(tv_id, language)
Get Translations

Get a list of the translations that exist for a TV show.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::TranslationsList**](TranslationsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_videos_list

> crate::models::VideosList get_tv_videos_list(tv_id, language)
Get Videos

Get the videos that have been added to a TV show.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::VideosList**](VideosList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tv_rating

> crate::models::StatusCodeMessage post_tv_rating(tv_id, content_type, guest_session_id, session_id, body)
Rate TV Show

Rate a TV show.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
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

