# FindApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_find_external_id**](FindApi.md#get_find_external_id) | **get** /find/{external_id} | Find by ID



## get_find_external_id

> crate::models::FindByExternalIdResults get_find_external_id(external_id, external_source, language)
Find by ID

The find method makes it easy to search for objects in our database by an external id. For instance, an IMDB ID.  This method will search all objects (movies, TV shows and people) and return the results in a single response.  The supported external sources for each object are as follows.  |              | Movies | TV Shows | TV Seasons | TV Episodes | People | | ------------ | ------ | -------- | ---------- | ----------- | ------ | | IMDB ID      | ✓      | ✓        | ✗          | ✓           | ✓ | Freebase MID | ✗      | ✓        | ✓          | ✓           | ✓ | Freebase ID  | ✗      | ✓        | ✓          | ✓           | ✓ | TVDB ID      | ✗      | ✓        | ✓          | ✓           | ✗ | TVRage ID    | ✗      | ✓        | ✓          | ✓           | ✓ 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** |  | Required | 
**external_source** | **String** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::FindByExternalIdResults**](FindByExternalIdResults.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

