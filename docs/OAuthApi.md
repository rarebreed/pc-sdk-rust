# \OAuthApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_oauth_client**](OAuthApi.md#delete_oauth_client) | **DELETE** /api/v2/oauth/clients/{clientId} | Delete OAuth Client
[**get_oauth_authorization**](OAuthApi.md#get_oauth_authorization) | **GET** /api/v2/oauth/authorizations/{clientId} | Get a client that is authorized by the resource owner
[**get_oauth_authorizations**](OAuthApi.md#get_oauth_authorizations) | **GET** /api/v2/oauth/authorizations | List clients that are authorized by the resource owner
[**get_oauth_client**](OAuthApi.md#get_oauth_client) | **GET** /api/v2/oauth/clients/{clientId} | Get OAuth Client
[**get_oauth_client_usage_query_result**](OAuthApi.md#get_oauth_client_usage_query_result) | **GET** /api/v2/oauth/clients/{clientId}/usage/query/results/{executionId} | Get the results of a usage query
[**get_oauth_client_usage_summary**](OAuthApi.md#get_oauth_client_usage_summary) | **GET** /api/v2/oauth/clients/{clientId}/usage/summary | Get a summary of OAuth client API usage
[**get_oauth_clients**](OAuthApi.md#get_oauth_clients) | **GET** /api/v2/oauth/clients | The list of OAuth clients
[**get_oauth_scope**](OAuthApi.md#get_oauth_scope) | **GET** /api/v2/oauth/scopes/{scopeId} | An OAuth scope
[**get_oauth_scopes**](OAuthApi.md#get_oauth_scopes) | **GET** /api/v2/oauth/scopes | The list of OAuth scopes
[**post_oauth_client_secret**](OAuthApi.md#post_oauth_client_secret) | **POST** /api/v2/oauth/clients/{clientId}/secret | Regenerate Client Secret
[**post_oauth_client_usage_query**](OAuthApi.md#post_oauth_client_usage_query) | **POST** /api/v2/oauth/clients/{clientId}/usage/query | Query for OAuth client API usage
[**post_oauth_clients**](OAuthApi.md#post_oauth_clients) | **POST** /api/v2/oauth/clients | Create OAuth client
[**put_oauth_client**](OAuthApi.md#put_oauth_client) | **PUT** /api/v2/oauth/clients/{clientId} | Update OAuth Client



## delete_oauth_client

> delete_oauth_client(client_id)
Delete OAuth Client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Client ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_authorization

> crate::models::OAuthAuthorization get_oauth_authorization(client_id)
Get a client that is authorized by the resource owner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The ID of client | [required] |

### Return type

[**crate::models::OAuthAuthorization**](OAuthAuthorization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_authorizations

> crate::models::OAuthAuthorizationListing get_oauth_authorizations()
List clients that are authorized by the resource owner

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OAuthAuthorizationListing**](OAuthAuthorizationListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_client

> crate::models::OAuthClient get_oauth_client(client_id)
Get OAuth Client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Client ID | [required] |

### Return type

[**crate::models::OAuthClient**](OAuthClient.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_client_usage_query_result

> crate::models::ApiUsageQueryResult get_oauth_client_usage_query_result(execution_id, client_id)
Get the results of a usage query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execution_id** | **String** | ID of the query execution | [required] |
**client_id** | **String** | Client ID | [required] |

### Return type

[**crate::models::ApiUsageQueryResult**](ApiUsageQueryResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_client_usage_summary

> crate::models::UsageExecutionResult get_oauth_client_usage_summary(client_id, days)
Get a summary of OAuth client API usage

After calling this method, you will then need to poll for the query results based on the returned execution Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Client ID | [required] |
**days** | Option<**String**> | Previous number of days to query |  |[default to 7]

### Return type

[**crate::models::UsageExecutionResult**](UsageExecutionResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_clients

> crate::models::OAuthClientEntityListing get_oauth_clients()
The list of OAuth clients

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OAuthClientEntityListing**](OAuthClientEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_scope

> crate::models::OAuthScope get_oauth_scope(scope_id, accept_language)
An OAuth scope

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope_id** | **String** | Scope ID | [required] |
**accept_language** | Option<**String**> | The language with which to display the scope description. |  |[default to en-us]

### Return type

[**crate::models::OAuthScope**](OAuthScope.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_scopes

> crate::models::OAuthScopeListing get_oauth_scopes(accept_language)
The list of OAuth scopes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | The language with which to display the scope descriptions. |  |[default to en-us]

### Return type

[**crate::models::OAuthScopeListing**](OAuthScopeListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_oauth_client_secret

> crate::models::OAuthClient post_oauth_client_secret(client_id)
Regenerate Client Secret

This operation will set the client secret to a randomly generated cryptographically random value. All clients must be updated with the new secret. This operation should be used with caution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Client ID | [required] |

### Return type

[**crate::models::OAuthClient**](OAuthClient.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_oauth_client_usage_query

> crate::models::UsageExecutionResult post_oauth_client_usage_query(client_id, body)
Query for OAuth client API usage

After calling this method, you will then need to poll for the query results based on the returned execution Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Client ID | [required] |
**body** | [**ApiUsageQuery**](ApiUsageQuery.md) | Query | [required] |

### Return type

[**crate::models::UsageExecutionResult**](UsageExecutionResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_oauth_clients

> crate::models::OAuthClient post_oauth_clients(body)
Create OAuth client

The OAuth Grant/Client is required in order to create an authentication token and gain access to PureCloud.  The preferred authorizedGrantTypes is 'CODE' which requires applications to send a client ID and client secret. This is typically a web server.  If the client is unable to secure the client secret then the 'TOKEN' grant type aka IMPLICIT should be used. This is would be for browser or mobile apps.  If a client is to be used outside of the context of a user then the 'CLIENT-CREDENTIALS' grant may be used. In this case the client must be granted roles  via the 'roleIds' field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OAuthClientRequest**](OAuthClientRequest.md) | Client | [required] |

### Return type

[**crate::models::OAuthClient**](OAuthClient.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_oauth_client

> crate::models::OAuthClient put_oauth_client(client_id, body)
Update OAuth Client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Client ID | [required] |
**body** | [**OAuthClientRequest**](OAuthClientRequest.md) | Client | [required] |

### Return type

[**crate::models::OAuthClient**](OAuthClient.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

