# TVEpisodeGroupsApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_episode_group_details**](TVEpisodeGroupsApi.md#get_episode_group_details) | **get** /tv/episode_group/{episode_group_id} | Get Details



## get_episode_group_details

> crate::models::EpisodeGroupDetails get_episode_group_details(episode_group_id, language)
Get Details

Get the details of a TV episode group. Groups support 7 different types which are enumerated as the following: 1. Original air date 2. Absolute 3. DVD 4. Digital 5. Story arc 6. Production 7. TV.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_group_id** | **String** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::EpisodeGroupDetails**](EpisodeGroupDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

