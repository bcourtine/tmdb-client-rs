# \CompaniesApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_company_details**](CompaniesApi.md#get_company_details) | **get** /company/{company_id} | Get Details
[**get_company_movies_paginated**](CompaniesApi.md#get_company_movies_paginated) | **get** /company/{company_id}/movies | Get Movies



## get_company_details

> ::models::CompanyDetails get_company_details(ctx, company_id)
Get Details

Get a companies details by id.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **company_id** | **i32**|  | 

### Return type

[**::models::CompanyDetails**](CompanyDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company_movies_paginated

> ::models::MoviePaginated get_company_movies_paginated(ctx, company_id, optional)
Get Movies

Get the movies of a company by id.  We highly recommend using [movie discover](#endpoint:p5NyoR7dM842S8G9j) instead of this method as it is much more flexible.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **company_id** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **company_id** | **i32**|  | 
 **language** | **String**| Pass a ISO 639-1 value to display translated data for the fields that support it. | [default to <<language>>]

### Return type

[**::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
