# TVEpisodesApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tv_season_episode_rating**](TVEpisodesApi.md#delete_tv_season_episode_rating) | **delete** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/rating | Delete Rating
[**get_tv_episode_changes**](TVEpisodesApi.md#get_tv_episode_changes) | **get** /tv/episode/{episode_id}/changes | Get Changes
[**get_tv_season_episode_account_states**](TVEpisodesApi.md#get_tv_season_episode_account_states) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/account_states | Get Account States
[**get_tv_season_episode_credits**](TVEpisodesApi.md#get_tv_season_episode_credits) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/credits | Get Credits
[**get_tv_season_episode_details**](TVEpisodesApi.md#get_tv_season_episode_details) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number} | Get Details
[**get_tv_season_episode_external_ids**](TVEpisodesApi.md#get_tv_season_episode_external_ids) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/external_ids | Get TV Episode External IDs
[**get_tv_season_episode_images**](TVEpisodesApi.md#get_tv_season_episode_images) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/images | Get Images
[**get_tv_season_episode_translations_list**](TVEpisodesApi.md#get_tv_season_episode_translations_list) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/translations | Get Translations
[**get_tv_season_episode_videos_list**](TVEpisodesApi.md#get_tv_season_episode_videos_list) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/videos | Get  Videos
[**post_tv_season_episode_rating**](TVEpisodesApi.md#post_tv_season_episode_rating) | **post** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/rating | Rate TV Episode



## delete_tv_season_episode_rating

> crate::models::StatusCodeMessage delete_tv_season_episode_rating(tv_id, season_number, episode_number, content_type, guest_session_id, session_id)
Delete Rating

Remove your rating for a TV episode.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**episode_number** | **i32** |  | Required | 
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


## get_tv_episode_changes

> crate::models::ChangeDetails get_tv_episode_changes(episode_id, start_date, end_date, page)
Get Changes

Get the changes for a TV episode. By default only the last 24 hours are returned.  You can query up to 14 days in a single query by using the `start_date` and `end_date` query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_id** | **i32** |  | Required | 
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


## get_tv_season_episode_account_states

> crate::models::EpisodeRatingList get_tv_season_episode_account_states(tv_id, season_number, episode_number, guest_session_id, session_id)
Get Account States

Get your rating for a episode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**episode_number** | **i32** |  | Required | 
**guest_session_id** | **String** |  |  | 
**session_id** | **String** |  |  | 

### Return type

[**crate::models::EpisodeRatingList**](EpisodeRatingList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_credits

> crate::models::Credits get_tv_season_episode_credits(tv_id, season_number, episode_number)
Get Credits

Get the credits (cast, crew and guest stars) for a TV episode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**episode_number** | **i32** |  | Required | 

### Return type

[**crate::models::Credits**](Credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_details

> crate::models::EpisodeDetails get_tv_season_episode_details(tv_id, season_number, episode_number, include_image_language, language, append_to_response)
Get Details

Get the TV episode details by id.  Supports `append_to_response`. Read more about this [here](#docTextSection:JdZq8ctmcxNqyLQjp).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**episode_number** | **i32** |  | Required | 
**include_image_language** | **String** | Pass a ISO 639-1 value to get additional images (cf. https://developers.themoviedb.org/3/getting-started/image-languages). |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**append_to_response** | **String** | Append requests within the same namespace to the response. |  | 

### Return type

[**crate::models::EpisodeDetails**](EpisodeDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_external_ids

> crate::models::ExternalIds get_tv_season_episode_external_ids(tv_id, season_number, episode_number)
Get TV Episode External IDs

Get the external ids for a TV episode. We currently support the following external sources.  | **External Sources** | | -------------------- | | IMDB ID              | | Freebase MID         | | Freebase ID          | | TVDB ID              | | TVRage ID            |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**episode_number** | **i32** |  | Required | 

### Return type

[**crate::models::ExternalIds**](ExternalIds.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_images

> crate::models::Images get_tv_season_episode_images(tv_id, season_number, episode_number, include_image_language, language)
Get Images

Get the images that belong to a TV episode.  Querying images with a `language` parameter will filter the results. If you want to include a fallback language (especially useful for backdrops) you can use the `include_image_language` parameter. This should be a comma seperated value like so: `include_image_language=en,null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**episode_number** | **i32** |  | Required | 
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


## get_tv_season_episode_translations_list

> crate::models::TranslationsList get_tv_season_episode_translations_list(tv_id, season_number, episode_number)
Get Translations

Get the translation data for an episode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**episode_number** | **i32** |  | Required | 

### Return type

[**crate::models::TranslationsList**](TranslationsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_episode_videos_list

> crate::models::VideosList get_tv_season_episode_videos_list(tv_id, season_number, episode_number, language)
Get  Videos

Get the videos that have been added to a TV episode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**episode_number** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::VideosList**](VideosList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tv_season_episode_rating

> crate::models::StatusCodeMessage post_tv_season_episode_rating(tv_id, season_number, episode_number, content_type, guest_session_id, session_id, body)
Rate TV Episode

Rate a TV episode.  A valid session or guest session ID is required. You can read more about how this works [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**episode_number** | **i32** |  | Required | 
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

