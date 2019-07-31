# ListsApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_list**](ListsApi.md#delete_list) | **delete** /list/{list_id} | Delete List
[**get_list_details**](ListsApi.md#get_list_details) | **get** /list/{list_id} | Get Details
[**get_list_item_status**](ListsApi.md#get_list_item_status) | **get** /list/{list_id}/item_status | Check Item Status
[**post_list**](ListsApi.md#post_list) | **post** /list | Create List
[**post_list_add_item**](ListsApi.md#post_list_add_item) | **post** /list/{list_id}/add_item | Add Movie
[**post_list_clear**](ListsApi.md#post_list_clear) | **post** /list/{list_id}/clear | Clear List
[**post_list_remove_item**](ListsApi.md#post_list_remove_item) | **post** /list/{list_id}/remove_item | Remove Movie



## delete_list

> crate::models::StatusCodeMessage delete_list(list_id, session_id)
Delete List

Delete a list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | Required | 
**session_id** | **String** |  | Required | 

### Return type

[**crate::models::StatusCodeMessage**](StatusCodeMessage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_details

> crate::models::ListDetails get_list_details(list_id, language)
Get Details

Get the details of a list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::ListDetails**](ListDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_item_status

> crate::models::ItemStatus get_list_item_status(list_id, movie_id)
Check Item Status

You can use this method to check if a movie has already been added to the list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | Required | 
**movie_id** | **i32** |  | Required | 

### Return type

[**crate::models::ItemStatus**](ItemStatus.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list

> crate::models::ListStatusResponse post_list(content_type, session_id, body)
Create List

Create a list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | **String** |  | Required | [default to application/json;charset=utf-8]
**session_id** | **String** |  | Required | 
**body** | [**ListBody**](ListBody.md) |  |  | 

### Return type

[**crate::models::ListStatusResponse**](ListStatusResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list_add_item

> crate::models::StatusCodeMessage post_list_add_item(list_id, content_type, session_id, body)
Add Movie

Add a movie to a list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | Required | 
**content_type** | **String** |  | Required | [default to application/json;charset=utf-8]
**session_id** | **String** |  | Required | 
**body** | [**MediaIdBody**](MediaIdBody.md) |  |  | 

### Return type

[**crate::models::StatusCodeMessage**](StatusCodeMessage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list_clear

> crate::models::StatusCodeMessage post_list_clear(list_id, confirm, session_id)
Clear List

Clear all of the items from a list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | Required | 
**confirm** | **bool** |  | Required | 
**session_id** | **String** |  | Required | 

### Return type

[**crate::models::StatusCodeMessage**](StatusCodeMessage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list_remove_item

> crate::models::StatusCodeMessage post_list_remove_item(list_id, content_type, session_id, body)
Remove Movie

Remove a movie from a list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | Required | 
**content_type** | **String** |  | Required | [default to application/json;charset=utf-8]
**session_id** | **String** |  | Required | 
**body** | [**MediaIdBody**](MediaIdBody.md) |  |  | 

### Return type

[**crate::models::StatusCodeMessage**](StatusCodeMessage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

