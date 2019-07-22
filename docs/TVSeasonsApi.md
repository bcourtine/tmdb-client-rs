# \TVSeasonsApi

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

> ::models::EpisodeRatingList get_tv_season_account_states(ctx, tv_id, season_number, optional)
Get Account States

Returns all of the user ratings for the season's episodes.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **guest_session_id** | **String**|  | 
 **session_id** | **String**|  | 

### Return type

[**::models::EpisodeRatingList**](EpisodeRatingList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_changes

> ::models::ChangeDetails get_tv_season_changes(ctx, season_id, optional)
Get  Changes

Get the changes for a TV season. By default only the last 24 hours are returned.  You can query up to 14 days in a single query by using the `start_date` and `end_date` query parameters.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **season_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **season_id** | **i32**|  | 
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


## get_tv_season_credits

> ::models::Credits get_tv_season_credits(ctx, tv_id, season_number, optional)
Get Credits

Get the credits for TV season.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::Credits**](Credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_details

> ::models::SeasonDetails get_tv_season_details(ctx, tv_id, season_number, optional)
Get Details

Get the TV season details by id.  Supports `append_to_response`. Read more about this [here](#docTextSection:JdZq8ctmcxNqyLQjp).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]
 **append_to_response** | **String**| Append requests within the same namespace to the response. | 

### Return type

[**::models::SeasonDetails**](SeasonDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_external_ids

> ::models::ExternalIds get_tv_season_external_ids(ctx, tv_id, season_number, optional)
Get External IDs

Get the external ids for a TV season. We currently support the following external sources.  | **External Sources** | | -------------------- | | Freebase MID         | | Freebase ID          | | TVDB ID              | | TVRage ID            |

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::ExternalIds**](ExternalIds.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_images

> ::models::Images get_tv_season_images(ctx, tv_id, season_number, optional)
Get Images

Get the images that belong to a TV season.  Querying images with a `language` parameter will filter the results. If you want to include a fallback language (especially useful for backdrops) you can use the `include_image_language` parameter. This should be a comma seperated value like so: `include_image_language=en,null`.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::Images**](Images.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_videos

> ::models::VideosList get_tv_season_videos(ctx, tv_id, season_number, optional)
Get Videos

Get the videos that have been added to a TV season.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tv_id** | **i32**|  | 
  **season_number** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tv_id** | **i32**|  | 
 **season_number** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::VideosList**](VideosList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
