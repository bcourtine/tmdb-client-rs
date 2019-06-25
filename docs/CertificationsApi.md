# \CertificationsApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_movie_certifications_list**](CertificationsApi.md#get_movie_certifications_list) | **get** /certification/movie/list | Get Movie Certifications
[**get_tv_certifications_list**](CertificationsApi.md#get_tv_certifications_list) | **get** /certification/tv/list | Get TV Certifications



## get_movie_certifications_list

> ::models::Certifications get_movie_certifications_list(ctx, )
Get Movie Certifications

Get an up to date list of the officially supported movie certifications on TMDb.

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**::models::Certifications**](certifications.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_certifications_list

> ::models::Certifications get_tv_certifications_list(ctx, )
Get TV Certifications

Get an up to date list of the officially supported TV show certifications on TMDb.

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**::models::Certifications**](certifications.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

