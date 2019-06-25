# \FindApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_find_external_id**](FindApi.md#get_find_external_id) | **get** /find/{external_id} | Find by ID



## get_find_external_id

> ::models::FindByExternalIdResults get_find_external_id(ctx, external_id, external_source, optional)
Find by ID

The find method makes it easy to search for objects in our database by an external id. For instance, an IMDB ID.  This method will search all objects (movies, TV shows and people) and return the results in a single response.  The supported external sources for each object are as follows.  |              | Movies | TV Shows | TV Seasons | TV Episodes | People | | ------------ | ------ | -------- | ---------- | ----------- | ------ | | IMDB ID      | ✓      | ✓        | ✗          | ✓           | ✓ | Freebase MID | ✗      | ✓        | ✓          | ✓           | ✓ | Freebase ID  | ✗      | ✓        | ✓          | ✓           | ✓ | TVDB ID      | ✗      | ✓        | ✓          | ✓           | ✗ | TVRage ID    | ✗      | ✓        | ✓          | ✓           | ✓ 

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **external_id** | **String**|  | 
  **external_source** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **external_id** | **String**|  | 
 **external_source** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::FindByExternalIdResults**](find-by-external-id-results.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

