# \GreetingsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_greeting**](GreetingsApi.md#delete_greeting) | **DELETE** /api/v2/greetings/{greetingId} | Deletes a Greeting with the given GreetingId
[**get_greeting**](GreetingsApi.md#get_greeting) | **GET** /api/v2/greetings/{greetingId} | Get a Greeting with the given GreetingId
[**get_greeting_media**](GreetingsApi.md#get_greeting_media) | **GET** /api/v2/greetings/{greetingId}/media | Get media playback URI for this greeting
[**get_greetings**](GreetingsApi.md#get_greetings) | **GET** /api/v2/greetings | Gets an Organization's Greetings
[**get_greetings_defaults**](GreetingsApi.md#get_greetings_defaults) | **GET** /api/v2/greetings/defaults | Get an Organization's DefaultGreetingList
[**get_group_greetings**](GreetingsApi.md#get_group_greetings) | **GET** /api/v2/groups/{groupId}/greetings | Get a list of the Group's Greetings
[**get_group_greetings_defaults**](GreetingsApi.md#get_group_greetings_defaults) | **GET** /api/v2/groups/{groupId}/greetings/defaults | Grabs the list of Default Greetings given a Group's ID
[**get_user_greetings**](GreetingsApi.md#get_user_greetings) | **GET** /api/v2/users/{userId}/greetings | Get a list of the User's Greetings
[**get_user_greetings_defaults**](GreetingsApi.md#get_user_greetings_defaults) | **GET** /api/v2/users/{userId}/greetings/defaults | Grabs the list of Default Greetings given a User's ID
[**post_greetings**](GreetingsApi.md#post_greetings) | **POST** /api/v2/greetings | Create a Greeting for an Organization
[**post_group_greetings**](GreetingsApi.md#post_group_greetings) | **POST** /api/v2/groups/{groupId}/greetings | Creates a Greeting for a Group
[**post_user_greetings**](GreetingsApi.md#post_user_greetings) | **POST** /api/v2/users/{userId}/greetings | Creates a Greeting for a User
[**put_greeting**](GreetingsApi.md#put_greeting) | **PUT** /api/v2/greetings/{greetingId} | Updates the Greeting with the given GreetingId
[**put_greetings_defaults**](GreetingsApi.md#put_greetings_defaults) | **PUT** /api/v2/greetings/defaults | Update an Organization's DefaultGreetingList
[**put_group_greetings_defaults**](GreetingsApi.md#put_group_greetings_defaults) | **PUT** /api/v2/groups/{groupId}/greetings/defaults | Updates the DefaultGreetingList of the specified Group
[**put_user_greetings_defaults**](GreetingsApi.md#put_user_greetings_defaults) | **PUT** /api/v2/users/{userId}/greetings/defaults | Updates the DefaultGreetingList of the specified User



## delete_greeting

> delete_greeting(greeting_id)
Deletes a Greeting with the given GreetingId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**greeting_id** | **String** | Greeting ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_greeting

> crate::models::Greeting get_greeting(greeting_id)
Get a Greeting with the given GreetingId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**greeting_id** | **String** | Greeting ID | [required] |

### Return type

[**crate::models::Greeting**](Greeting.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_greeting_media

> crate::models::GreetingMediaInfo get_greeting_media(greeting_id, format_id)
Get media playback URI for this greeting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**greeting_id** | **String** | Greeting ID | [required] |
**format_id** | Option<**String**> | The desired media format. |  |[default to WAV]

### Return type

[**crate::models::GreetingMediaInfo**](GreetingMediaInfo.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_greetings

> crate::models::DomainEntityListing get_greetings(page_size, page_number)
Gets an Organization's Greetings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::DomainEntityListing**](DomainEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_greetings_defaults

> crate::models::DefaultGreetingList get_greetings_defaults()
Get an Organization's DefaultGreetingList

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DefaultGreetingList**](DefaultGreetingList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_greetings

> crate::models::GreetingListing get_group_greetings(group_id, page_size, page_number)
Get a list of the Group's Greetings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::GreetingListing**](GreetingListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_greetings_defaults

> crate::models::DefaultGreetingList get_group_greetings_defaults(group_id)
Grabs the list of Default Greetings given a Group's ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |

### Return type

[**crate::models::DefaultGreetingList**](DefaultGreetingList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_greetings

> crate::models::DomainEntityListing get_user_greetings(user_id, page_size, page_number)
Get a list of the User's Greetings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::DomainEntityListing**](DomainEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_greetings_defaults

> crate::models::DefaultGreetingList get_user_greetings_defaults(user_id)
Grabs the list of Default Greetings given a User's ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::DefaultGreetingList**](DefaultGreetingList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_greetings

> crate::models::Greeting post_greetings(body)
Create a Greeting for an Organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Greeting**](Greeting.md) | The Greeting to create | [required] |

### Return type

[**crate::models::Greeting**](Greeting.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_group_greetings

> crate::models::Greeting post_group_greetings(group_id, body)
Creates a Greeting for a Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**body** | [**Greeting**](Greeting.md) | The Greeting to create | [required] |

### Return type

[**crate::models::Greeting**](Greeting.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_user_greetings

> crate::models::Greeting post_user_greetings(user_id, body)
Creates a Greeting for a User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**Greeting**](Greeting.md) | The Greeting to create | [required] |

### Return type

[**crate::models::Greeting**](Greeting.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_greeting

> crate::models::Greeting put_greeting(greeting_id, body)
Updates the Greeting with the given GreetingId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**greeting_id** | **String** | Greeting ID | [required] |
**body** | [**Greeting**](Greeting.md) | The updated Greeting | [required] |

### Return type

[**crate::models::Greeting**](Greeting.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_greetings_defaults

> crate::models::DefaultGreetingList put_greetings_defaults(body)
Update an Organization's DefaultGreetingList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DefaultGreetingList**](DefaultGreetingList.md) | The updated defaultGreetingList | [required] |

### Return type

[**crate::models::DefaultGreetingList**](DefaultGreetingList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_group_greetings_defaults

> crate::models::DefaultGreetingList put_group_greetings_defaults(group_id, body)
Updates the DefaultGreetingList of the specified Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**body** | [**DefaultGreetingList**](DefaultGreetingList.md) | The updated defaultGreetingList | [required] |

### Return type

[**crate::models::DefaultGreetingList**](DefaultGreetingList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_greetings_defaults

> crate::models::DefaultGreetingList put_user_greetings_defaults(user_id, body)
Updates the DefaultGreetingList of the specified User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**DefaultGreetingList**](DefaultGreetingList.md) | The updated defaultGreetingList | [required] |

### Return type

[**crate::models::DefaultGreetingList**](DefaultGreetingList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

