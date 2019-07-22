# \ListsApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_list_details**](ListsApi.md#get_list_details) | **get** /list/{list_id} | Get Details
[**get_list_item_status**](ListsApi.md#get_list_item_status) | **get** /list/{list_id}/item_status | Check Item Status
[**post_list**](ListsApi.md#post_list) | **post** /list | Create List
[**post_list_add_item**](ListsApi.md#post_list_add_item) | **post** /list/{list_id}/add_item | Add Movie
[**post_list_clear**](ListsApi.md#post_list_clear) | **post** /list/{list_id}/clear | Clear List
[**post_list_remove_item**](ListsApi.md#post_list_remove_item) | **post** /list/{list_id}/remove_item | Remove Movie
[**delete_list**](ListsApi.md#delete_list) | **delete** /list/{list_id} | Delete List



## get_list_details

> ::models::ListDetails get_list_details(ctx, list_id, optional)
Get Details

Get the details of a list.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **list_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **list_id** | **String**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::ListDetails**](ListDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_item_status

> ::models::ItemStatus get_list_item_status(ctx, list_id, movie_id)
Check Item Status

You can use this method to check if a movie has already been added to the list.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **list_id** | **String**|  | 
  **movie_id** | **i32**|  | 

### Return type

[**::models::ItemStatus**](ItemStatus.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list

> ::models::ListStatusResponse post_list(ctx, content_type, session_id, optional)
Create List

Create a list.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **session_id** | **String**|  | 
 **body** | [**ListBody**](ListBody.md)|  | 

### Return type

[**::models::ListStatusResponse**](ListStatusResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list_add_item

> ::models::InlineResponse401 post_list_add_item(ctx, list_id, content_type, session_id, optional)
Add Movie

Add a movie to a list.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **list_id** | **String**|  | 
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **list_id** | **String**|  | 
 **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **session_id** | **String**|  | 
 **body** | [**MediaIdBody**](MediaIdBody.md)|  | 

### Return type

[**::models::InlineResponse401**](inline_response_401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list_clear

> ::models::InlineResponse401 post_list_clear(ctx, list_id, confirm, session_id)
Clear List

Clear all of the items from a list.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **list_id** | **String**|  | 
  **confirm** | **bool**|  | 
  **session_id** | **String**|  | 

### Return type

[**::models::InlineResponse401**](inline_response_401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list_remove_item

> ::models::InlineResponse401 post_list_remove_item(ctx, list_id, content_type, session_id, optional)
Remove Movie

Remove a movie from a list.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **list_id** | **String**|  | 
  **content_type** | **String**|  | [default to application/json;charset=utf-8]
  **session_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **list_id** | **String**|  | 
 **content_type** | **String**|  | [default to application/json;charset=utf-8]
 **session_id** | **String**|  | 
 **body** | [**MediaIdBody**](MediaIdBody.md)|  | 

### Return type

[**::models::InlineResponse401**](inline_response_401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_list

> ::models::InlineResponse401 delete_list(list_id, session_id)
Delete List

Delete a list.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **list_id** | **String**|  | 
 **session_id** | **String**|  | 

### Return type

[**::models::InlineResponse401**](InlineResponse401.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
