# NetworksApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_network_alternative_names_list**](NetworksApi.md#get_network_alternative_names_list) | **get** /network/{network_id}/alternative_names | Get Alternative Names
[**get_network_details**](NetworksApi.md#get_network_details) | **get** /network/{network_id} | Get Details
[**get_network_images**](NetworksApi.md#get_network_images) | **get** /network/{network_id}/images | Get Images



## get_network_alternative_names_list

> crate::models::AlternativeNamesList get_network_alternative_names_list(network_id)
Get Alternative Names

Get the alternative names of a network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **i32** |  | Required | 

### Return type

[**crate::models::AlternativeNamesList**](AlternativeNamesList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_details

> crate::models::Network get_network_details(network_id)
Get Details

Get the details of a network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **i32** |  | Required | 

### Return type

[**crate::models::Network**](Network.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_images

> crate::models::Images get_network_images(network_id)
Get Images

Get the TV network logos by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **i32** |  | Required | 

### Return type

[**crate::models::Images**](Images.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

