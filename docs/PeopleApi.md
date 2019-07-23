# \PeopleApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_external_ids**](PeopleApi.md#get_external_ids) | **get** /person/{person_id}/external_ids | Get External IDs
[**get_person_changes**](PeopleApi.md#get_person_changes) | **get** /person/{person_id}/changes | Get Changes
[**get_person_combined_credits**](PeopleApi.md#get_person_combined_credits) | **get** /person/{person_id}/combined_credits | Get Combined Credits
[**get_person_details**](PeopleApi.md#get_person_details) | **get** /person/{person_id} | Get Details
[**get_person_images_list**](PeopleApi.md#get_person_images_list) | **get** /person/{person_id}/images | Get Images
[**get_person_latest_details**](PeopleApi.md#get_person_latest_details) | **get** /person/latest | Get Latest
[**get_person_movie_credits**](PeopleApi.md#get_person_movie_credits) | **get** /person/{person_id}/movie_credits | Get Movie Credits
[**get_person_popular_paginated**](PeopleApi.md#get_person_popular_paginated) | **get** /person/popular | Get Popular
[**get_person_tagged_images_paginated**](PeopleApi.md#get_person_tagged_images_paginated) | **get** /person/{person_id}/tagged_images | Get Tagged Images
[**get_person_translations_list**](PeopleApi.md#get_person_translations_list) | **get** /person/{person_id}/translations | Get Translations
[**get_person_tv_credits**](PeopleApi.md#get_person_tv_credits) | **get** /person/{person_id}/tv_credits | Get TV Credits



## get_external_ids

> crate::models::ExternalIds get_external_ids(person_id, language)
Get External IDs

Get the external ids for a person. We currently support the following external sources.  | **External Sources** | | ------------     | | IMDB ID          | | Facebook         | | Freebase MID     | | Freebase ID      | | Instagram        | | TVRage ID        | | Twitter          |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::ExternalIds**](ExternalIds.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_changes

> crate::models::ChangeDetails get_person_changes(person_id, language, start_date, end_date, page)
Get Changes

Get the changes for a person. By default only the last 24 hours are returned.  You can query up to 14 days in a single query by using the `start_date` and `end_date` query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**start_date** | **String** | Filter the results with a start date. |  | 
**end_date** | **String** | Filter the results with a end date. |  | 
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::ChangeDetails**](ChangeDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_combined_credits

> crate::models::PersonCredits get_person_combined_credits(person_id, language)
Get Combined Credits

Get the movie and TV credits together in a single response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::PersonCredits**](person-Credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_details

> crate::models::PersonDetails get_person_details(person_id, language, append_to_response)
Get Details

Get the primary person details by id.  Supports `append_to_response`. Read more about this [here](#docTextSection:JdZq8ctmcxNqyLQjp).  #### New as of November 9, 2016  Biographies are now translatable on TMDb. This means you can query person details with a language parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**append_to_response** | **String** | Append requests within the same namespace to the response. |  | 

### Return type

[**crate::models::PersonDetails**](PersonDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_images_list

> crate::models::Images get_person_images_list(person_id)
Get Images

Get the images for a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** |  | Required | 

### Return type

[**crate::models::Images**](Images.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_latest_details

> crate::models::PersonDetails get_person_latest_details(language)
Get Latest

Get the most newly created person. This is a live response and will continuously change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::PersonDetails**](PersonDetails.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_movie_credits

> crate::models::PersonCredits get_person_movie_credits(person_id, language)
Get Movie Credits

Get the movie credits for a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::PersonCredits**](person-Credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_popular_paginated

> crate::models::PersonPaginated get_person_popular_paginated(language, page)
Get Popular

Get the list of popular people on TMDb. This list updates daily.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::PersonPaginated**](PersonPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_tagged_images_paginated

> crate::models::PersonTaggedImagesPaginated get_person_tagged_images_paginated(person_id, language, page)
Get Tagged Images

Get the images that this person has been tagged in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]
**page** | **i32** | Specify which page to query. |  | [default to 1]

### Return type

[**crate::models::PersonTaggedImagesPaginated**](PersonTaggedImagesPaginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_translations_list

> crate::models::TranslationsList get_person_translations_list(person_id, language)
Get Translations

Get a list of translations that have been created for a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::TranslationsList**](TranslationsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_tv_credits

> crate::models::PersonCredits get_person_tv_credits(person_id, language)
Get TV Credits

Get the TV show credits for a person.  You can query for some extra details about the credit with the [credit method](#endpoint:xPWdEBLkvCNZSicLN).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **i32** |  | Required | 
**language** | **String** | Pass a ISO 639-1 value to display translated data for the fields that support it. |  | [default to <<language>>]

### Return type

[**crate::models::PersonCredits**](person-Credits.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

