# TVSeasonsApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tv_season_account_states**](TVSeasonsApi.md#get_tv_season_account_states) | **get** /tv/{tv_id}/season/{season_number}/account_states | Get Account States
[**get_tv_season_changes**](TVSeasonsApi.md#get_tv_season_changes) | **get** /tv/season/{season_id}/changes | Get  Changes
[**get_tv_season_credits**](TVSeasonsApi.md#get_tv_season_credits) | **get** /tv/{tv_id}/season/{season_number}/credits | Get Credits
[**get_tv_season_details**](TVSeasonsApi.md#get_tv_season_details) | **get** /tv/{tv_id}/season/{season_number} | Get Details
[**get_tv_season_external_ids**](TVSeasonsApi.md#get_tv_season_external_ids) | **get** /tv/{tv_id}/season/{season_number}/external_ids | Get External IDs
[**get_tv_season_images**](TVSeasonsApi.md#get_tv_season_images) | **get** /tv/{tv_id}/season/{season_number}/images | Get Images
[**get_tv_season_videos**](TVSeasonsApi.md#get_tv_season_videos) | **get** /tv/{tv_id}/season/{season_number}/videos | Get Videos



## get_tv_season_account_states

> crate::models::EpisodeRatingList get_tv_season_account_states(tv_id, season_number, language, guest_session_id, session_id)
Get Account States

Returns all of the user ratings for the season's episodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
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


## get_tv_season_changes

> crate::models::ChangeDetails get_tv_season_changes(season_id, start_date, end_date, page)
Get  Changes

Get the changes for a TV season. By default only the last 24 hours are returned.  You can query up to 14 days in a single query by using the `start_date` and `end_date` query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**season_id** | **i32** |  | Required | 
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


## get_tv_season_credits

> crate::models::Credits get_tv_season_credits(tv_id, season_number, language)
Get Credits

Get the credits for TV season.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::Credits**](Credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_details

> crate::models::SeasonDetails get_tv_season_details(tv_id, season_number, include_image_language, language, append_to_response)
Get Details

Get the TV season details by id.  Supports `append_to_response`. Read more about this [here](#docTextSection:JdZq8ctmcxNqyLQjp).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**include_image_language** | **String** | Pass a ISO 639-1 value to get additional images (cf. https://developers.themoviedb.org/3/getting-started/image-languages). |  | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**append_to_response** | **String** | Append requests within the same namespace to the response. |  | 

### Return type

[**crate::models::SeasonDetails**](SeasonDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_external_ids

> crate::models::ExternalIds get_tv_season_external_ids(tv_id, season_number, language)
Get External IDs

Get the external ids for a TV season. We currently support the following external sources.  | **External Sources** | | -------------------- | | Freebase MID         | | Freebase ID          | | TVDB ID              | | TVRage ID            |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::ExternalIds**](ExternalIds.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_images

> crate::models::Images get_tv_season_images(tv_id, season_number, include_image_language, language)
Get Images

Get the images that belong to a TV season.  Querying images with a `language` parameter will filter the results. If you want to include a fallback language (especially useful for backdrops) you can use the `include_image_language` parameter. This should be a comma seperated value like so: `include_image_language=en,null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
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


## get_tv_season_videos

> crate::models::VideosList get_tv_season_videos(tv_id, season_number, language)
Get Videos

Get the videos that have been added to a TV season.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **i32** |  | Required | 
**season_number** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::VideosList**](VideosList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

