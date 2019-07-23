# \CreditsApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_credit_details**](CreditsApi.md#get_credit_details) | **get** /credit/{credit_id} | Get Details



## get_credit_details

> crate::models::Credit get_credit_details(credit_id)
Get Details

Get a movie or TV credit details by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credit_id** | **String** |  | Required | 

### Return type

[**crate::models::Credit**](Credit.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

