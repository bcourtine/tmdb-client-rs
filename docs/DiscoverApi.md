# \DiscoverApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_discover_movie_paginated**](DiscoverApi.md#get_discover_movie_paginated) | **get** /discover/movie | Movie Discover
[**get_discover_tv_paginated**](DiscoverApi.md#get_discover_tv_paginated) | **get** /discover/tv | TV Discover



## get_discover_movie_paginated

> ::models::MoviePaginated get_discover_movie_paginated(ctx, optional)
Movie Discover

Discover movies by different types of data like average rating, number of votes, genres and certifications. You can get a valid list of certifications from the /certifications method.  Discover also supports a nice list of sort options. See below for all of the available options.  Please note, when using `certification` \\ `certification.lte` you must also specify `certification_country`. These two parameters work together in order to filter the results. You can only filter results with the countries we have added to our [certifications list](#endpoint:faFKjuKG2HnwexAWM).  If you specify the `region` parameter, the regional release date will be used instead of the primary release date. The date returned will be the first date based on your query (ie. if a `with_release_type` is specified). It's important to note the order of the release types that are used. Specifying \"2|3\" would return the limited theatrical release date as opposed to \"3|2\" which would return the theatrical date.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort_by** | **String**| Choose from one of the many available sort options. | [default to popularity.desc]
 **certification_country** | **String**| Used in conjunction with the certification filter, use this to specify a country with a valid certification. | 
 **certification** | **String**| Filter results with a valid certification from the 'certification_country' field. | 
 **certification_lte** | **String**| Filter and only include movies that have a certification that is less than or equal to the specified value. | 
 **include_adult** | **bool**| A filter and include or exclude adult movies. | [default to false]
 **include_video** | **bool**| A filter to include or exclude videos. | [default to false]
 **language** | **String**| Specify a language to query translatable fields with. | [default to en-US]
 **page** | **i32**| Specify the page of results to query. | [default to 1]
 **primary_release_year** | **i32**| A filter to limit the results to a specific primary release year. | 
 **primary_release_date_gte** | **String**| Filter and only include movies that have a primary release date that is greater or equal to the specified value. | 
 **primary_release_date_lte** | **String**| Filter and only include movies that have a primary release date that is less than or equal to the specified value. | 
 **release_date_gte** | **String**| Filter and only include movies that have a release date (looking at all release dates) that is greater or equal to the specified value. | 
 **release_date_lte** | **String**| Filter and only include movies that have a release date (looking at all release dates) that is less than or equal to the specified value. | 
 **vote_count_gte** | **i32**| Filter and only include movies that have a vote count that is greater or equal to the specified value. | 
 **vote_count_lte** | **i32**| Filter and only include movies that have a vote count that is less than or equal to the specified value. | 
 **vote_average_gte** | **f32**| Filter and only include movies that have a rating that is greater or equal to the specified value. | 
 **vote_average_lte** | **f32**| Filter and only include movies that have a rating that is less than or equal to the specified value. | 
 **with_cast** | **String**| A comma separated list of person ID's. Only include movies that have one of the ID's added as an actor. | 
 **with_crew** | **String**| A comma separated list of person ID's. Only include movies that have one of the ID's added as a crew member. | 
 **with_companies** | **String**| A comma separated list of production company ID's. Only include movies that have one of the ID's added as a production company. | 
 **with_genres** | **String**| Comma separated value of genre ids that you want to include in the results. | 
 **with_keywords** | **String**| A comma separated list of keyword ID's. Only include movies that have one of the ID's added as a keyword. | 
 **with_people** | **String**| A comma separated list of person ID's. Only include movies that have one of the ID's added as a either a actor or a crew member. | 
 **year** | **i32**| A filter to limit the results to a specific year (looking at all release dates). | 
 **without_genres** | **String**| Comma separated value of genre ids that you want to exclude from the results. | 
 **with_runtime_gte** | **i32**| Filter and only inlcude movies that have a runtime that is greater or equal to a value. | 
 **with_runtime_lte** | **i32**| Filter and only inlcude movies that have a runtime that is less than or equal to a value. | 
 **with_release_type** | **i32**| Specify a comma (AND) or pipe (OR) separated value to filter release types by. These release types map to the same values found on the movie release date method. | 
 **with_original_language** | **String**| Specify an ISO 639-1 string to filter results by their original language value. | 
 **without_keywords** | **String**| Exclude items with certain keywords. You can comma and pipe seperate these values to create an 'AND' or 'OR' logic. | 
 **region** | **String**| Specify a ISO 3166-1 code to filter release dates. | 

### Return type

[**::models::MoviePaginated**](movie-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_tv_paginated

> ::models::TvPaginated get_discover_tv_paginated(ctx, optional)
TV Discover

Discover TV shows by different types of data like average rating, number of votes, genres, the network they aired on and air dates.  Discover also supports a nice list of sort options. See below for all of the available options.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort_by** | **String**| Choose from one of the many available sort options. | [default to popularity.desc]
 **air_date_gte** | **String**| Filter and only include TV shows that have a air date (by looking at all episodes) that is greater or equal to the specified value. | 
 **air_date_lte** | **String**| Filter and only include TV shows that have a air date (by looking at all episodes) that is less than or equal to the specified value. | 
 **first_air_date_gte** | **String**| Filter and only include TV shows that have a original air date that is greater or equal to the specified value. Can be used in conjunction with the \"include_null_first_air_dates\" filter if you want to include items with no air date. | 
 **first_air_date_lte** | **String**| Filter and only include TV shows that have a original air date that is less than or equal to the specified value. Can be used in conjunction with the \"include_null_first_air_dates\" filter if you want to include items with no air date. | 
 **first_air_date_year** | **i32**| Filter and only include TV shows that have a original air date year that equal to the specified value. Can be used in conjunction with the \"include_null_first_air_dates\" filter if you want to include items with no air date. | 
 **language** | **String**| Specify a language to query translatable fields with. | [default to en-US]
 **page** | **i32**| Specify the page of results to query. | [default to 1]
 **timezone** | **String**| Used in conjunction with the air_date.gte/lte filter to calculate the proper UTC offset. | [default to America/New_York]
 **vote_average_gte** | **f32**| Filter and only include movies that have a rating that is greater or equal to the specified value. | 
 **vote_count_gte** | **i32**| Filter and only include movies that have a rating that is less than or equal to the specified value. | 
 **with_genres** | **String**| Comma separated value of genre ids that you want to include in the results. | 
 **with_networks** | **String**| Comma separated value of network ids that you want to include in the results. | 
 **without_genres** | **String**| Comma separated value of genre ids that you want to exclude from the results. | 
 **with_runtime_gte** | **i32**| Filter and only inlcude movies that have a runtime that is greater or equal to a value. | 
 **with_runtime_lte** | **i32**| Filter and only inlcude movies that have a runtime that is less than or equal to a value. | 
 **include_null_first_air_dates** | **bool**| Use this filter to include TV shows that don't have an air date while using any of the \"first_air_date\" filters. | [default to false]
 **with_original_language** | **String**| Specify an ISO 639-1 string to filter results by their original language value. | 
 **without_keywords** | **String**| Exclude items with certain keywords. You can comma and pipe seperate these values to create an 'AND' or 'OR' logic. | 

### Return type

[**::models::TvPaginated**](tv-paginated.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

