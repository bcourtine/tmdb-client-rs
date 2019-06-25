# \AuthenticationApi

All URIs are relative to *https://api.themoviedb.org/3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_new_authentication_guest_session**](AuthenticationApi.md#get_new_authentication_guest_session) | **get** /authentication/guest_session/new | Create Guest Session
[**get_new_authentication_session**](AuthenticationApi.md#get_new_authentication_session) | **get** /authentication/session/new | Create Session
[**get_new_authentication_token**](AuthenticationApi.md#get_new_authentication_token) | **get** /authentication/token/new | Create Request Token
[**get_validate_authentication_token_with_login**](AuthenticationApi.md#get_validate_authentication_token_with_login) | **get** /authentication/token/validate_with_login | Validate Request Token



## get_new_authentication_guest_session

> ::models::GuestSessionResponse get_new_authentication_guest_session(ctx, )
Create Guest Session

This method will let you create a new guest session. Guest sessions are a type of session that will let a user rate movies and TV shows but not require them to have a TMDb user account. More information about user authentication can be found [here](#docTextSection:NSZtgz7zptsiLYxXZ).  Please note, you should only generate a single guest session per user (or device) as you will be able to attach the ratings to a TMDb user account in the future. There is also IP limits in place so you should always make sure it's the end user doing the guest session actions.  If a guest session is not used for the first time within 24 hours, it will be automatically deleted.

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**::models::GuestSessionResponse**](guest-session-response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_new_authentication_session

> ::models::SessionResponse get_new_authentication_session(ctx, request_token)
Create Session

You can use this method to create a fully valid session ID once a user has validated the request token. More information about how this works can be found [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request_token** | **String**|  | 

### Return type

[**::models::SessionResponse**](session-response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_new_authentication_token

> ::models::TokenResponseWithExpiration get_new_authentication_token(ctx, )
Create Request Token

Create a temporary request token that can be used to validate a TMDb user login. More details about how this works can be found [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**::models::TokenResponseWithExpiration**](token-response-with-expiration.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_validate_authentication_token_with_login

> ::models::TokenResponse get_validate_authentication_token_with_login(ctx, username, password, request_token)
Validate Request Token

This method allows an application to validate a request token by entering a username and password.  #### Caution Please note, using this method is **strongly discouraged**. The preferred method of validating a request token is to have a user authenticate the request via the TMDb website. You can read about that method [here](#docTextSection:NSZtgz7zptsiLYxXZ).

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **username** | **String**|  | 
  **password** | **String**|  | 
  **request_token** | **String**|  | 

### Return type

[**::models::TokenResponse**](token-response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
