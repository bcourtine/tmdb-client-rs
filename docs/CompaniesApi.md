# \CompaniesApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_company_details**](CompaniesApi.md#get_company_details) | **get** /company/{company_id} | Get Details
[**get_company_images**](CompaniesApi.md#get_company_images) | **get** /company/{company_id}/images | Get Images
[**get_company_movies_paginated**](CompaniesApi.md#get_company_movies_paginated) | **get** /company/{company_id}/movies | Get Movies



## get_company_details

> crate::models::CompanyDetails get_company_details(company_id)
Get Details

Get a companies details by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** |  | Required | 

### Return type

[**crate::models::CompanyDetails**](CompanyDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company_images

> crate::models::Images get_company_images(company_id)
Get Images

Get company logos by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** |  | Required | 

### Return type

[**crate::models::Images**](Images.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company_movies_paginated

> crate::models::MoviePaginated get_company_movies_paginated(company_id, language)
Get Movies

Get the movies of a company by id.  We highly recommend using [movie discover](#endpoint:p5NyoR7dM842S8G9j) instead of this method as it is much more flexible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::MoviePaginated**](MoviePaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

