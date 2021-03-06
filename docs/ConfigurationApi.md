# ConfigurationApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_configuration**](ConfigurationApi.md#get_configuration) | **get** /configuration | Get API Configuration
[**get_countries_list**](ConfigurationApi.md#get_countries_list) | **get** /configuration/countries | Get Countries
[**get_jobs_list**](ConfigurationApi.md#get_jobs_list) | **get** /configuration/jobs | Get Jobs
[**get_languages_list**](ConfigurationApi.md#get_languages_list) | **get** /configuration/languages | Get Languages
[**get_primary_translations_list**](ConfigurationApi.md#get_primary_translations_list) | **get** /configuration/primary_translations | Get primary translations
[**get_timezones_list**](ConfigurationApi.md#get_timezones_list) | **get** /configuration/timezones | Get List



## get_configuration

> crate::models::Configuration get_configuration(api_key)
Get API Configuration

Get the system wide configuration information. Some elements of the API require some knowledge of this configuration data. The purpose of this is to try and keep the actual API responses as light as possible. It is recommended you cache this data within your application and check for updates every few days.  This method currently holds the data relevant to building image URLs as well as the change key map.  To build an image URL, you will need 3 pieces of data. The `base_url`, `size` and `file_path`. Simply combine them all and you will have a fully qualified URL. Here’s an example URL:      https://image.tmdb.org/t/p/w500/8uO0gUM8aNqYLs1OsTBQiXu0fEv.jpg  The configuration method also contains the list of change keys which can be useful if you are building an app that consumes data from the change feed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** |  |  | 

### Return type

[**crate::models::Configuration**](Configuration.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_countries_list

> Vec<crate::models::Translation> get_countries_list()
Get Countries

Get the list of countries (ISO 3166-1 tags) used throughout TMDb.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Translation>**](Translation.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jobs_list

> Vec<crate::models::Jobs> get_jobs_list()
Get Jobs

The the list of official jobs that are used on TMDb.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Jobs>**](Jobs.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languages_list

> Vec<crate::models::Translation> get_languages_list()
Get Languages

Get the list of languages (ISO 639-1 tags) used throughout TMDb.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Translation>**](Translation.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_primary_translations_list

> Vec<String> get_primary_translations_list()
Get primary translations

Get a list of the officially supported translations on TMDb.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_timezones_list

> Vec<crate::models::Timezones> get_timezones_list()
Get List

Get the list of supported timezones on TMDb.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Timezones>**](Timezones.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

