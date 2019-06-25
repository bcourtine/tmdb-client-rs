# \CreditsApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_credit_details**](CreditsApi.md#get_credit_details) | **get** /credit/{credit_id} | Get Details



## get_credit_details

> ::models::Credit get_credit_details(ctx, credit_id)
Get Details

Get a movie or TV credit details by id.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **credit_id** | **String**|  | 

### Return type

[**::models::Credit**](credit.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

