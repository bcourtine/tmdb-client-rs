# \CollectionsApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_collection_details**](CollectionsApi.md#get_collection_details) | **get** /collection/{collection_id} | Get Details
[**get_collection_images_list**](CollectionsApi.md#get_collection_images_list) | **get** /collection/{collection_id}/images | Get Images
[**get_collection_translations_list**](CollectionsApi.md#get_collection_translations_list) | **get** /collection/{collection_id}/translations | Get Translations



## get_collection_details

> crate::models::CollectionObject get_collection_details(collection_id, language)
Get Details

Get collection details by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::CollectionObject**](collectionObject.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_images_list

> crate::models::Images get_collection_images_list(collection_id, language)
Get Images

Get the images for a collection by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::Images**](Images.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_translations_list

> crate::models::TranslationsList get_collection_translations_list(collection_id, language)
Get Translations

Get the list translations for a collection by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::TranslationsList**](TranslationsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

