# MovieDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**adult** | **bool** |  | [optional] 
**backdrop_path** | **String** |  | [optional]
**belongs_to_collection** | **serde_json::Value** |  | [optional]
**budget** | **i32** |  | [optional] 
**genres** | [**Vec<::models::Genre>**](Genre.md) |  | [optional] 
**homepage** | **String** |  | [optional] 
**id** | **i32** |  | [optional] 
**imdb_id** | **String** |  | [optional] 
**original_language** | **String** |  | [optional] 
**original_title** | **String** |  | [optional] 
**overview** | **String** |  | [optional] 
**popularity** | **f32** |  | [optional] 
**poster_path** | **String** |  | [optional]
**production_companies** | [**Vec<::models::CompanyObject>**](CompanyObject.md) |  | [optional]
**production_countries** | [**Vec<::models::Translation>**](Translation.md) |  | [optional]
**release_date** | **String** |  | [optional]
**revenue** | **i32** |  | [optional] 
**runtime** | **i32** |  | [optional] 
**spoken_languages** | [**Vec<::models::Translation>**](Translation.md) |  | [optional]
**status** | **String** |  | [optional] 
**tagline** | **String** |  | [optional] 
**title** | **String** |  | [optional] 
**video** | **bool** |  | [optional] 
**vote_average** | **f32** |  | [optional] 
**vote_count** | **i32** |  | [optional]
**credits** | [**::models::Credits**](Credits.md) |  | [optional]
**videos** | [**::models::VideosList**](VideosList.md) |  | [optional]
**images** | [**::models::Images**](Images.md) |  | [optional]
**release_dates** | [**::models::ReleaseDatesList**](ReleaseDatesList.md) |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

