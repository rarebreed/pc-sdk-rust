# \WebChatApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_webchat_deployment**](WebChatApi.md#delete_webchat_deployment) | **DELETE** /api/v2/webchat/deployments/{deploymentId} | Delete a WebChat deployment
[**delete_webchat_guest_conversation_member**](WebChatApi.md#delete_webchat_guest_conversation_member) | **DELETE** /api/v2/webchat/guest/conversations/{conversationId}/members/{memberId} | Remove a member from a chat conversation
[**delete_webchat_settings**](WebChatApi.md#delete_webchat_settings) | **DELETE** /api/v2/webchat/settings | Remove WebChat deployment settings
[**get_webchat_deployment**](WebChatApi.md#get_webchat_deployment) | **GET** /api/v2/webchat/deployments/{deploymentId} | Get a WebChat deployment
[**get_webchat_deployments**](WebChatApi.md#get_webchat_deployments) | **GET** /api/v2/webchat/deployments | List WebChat deployments
[**get_webchat_guest_conversation_mediarequest**](WebChatApi.md#get_webchat_guest_conversation_mediarequest) | **GET** /api/v2/webchat/guest/conversations/{conversationId}/mediarequests/{mediaRequestId} | Get a media request in the conversation
[**get_webchat_guest_conversation_mediarequests**](WebChatApi.md#get_webchat_guest_conversation_mediarequests) | **GET** /api/v2/webchat/guest/conversations/{conversationId}/mediarequests | Get all media requests to the guest in the conversation
[**get_webchat_guest_conversation_member**](WebChatApi.md#get_webchat_guest_conversation_member) | **GET** /api/v2/webchat/guest/conversations/{conversationId}/members/{memberId} | Get a web chat conversation member
[**get_webchat_guest_conversation_members**](WebChatApi.md#get_webchat_guest_conversation_members) | **GET** /api/v2/webchat/guest/conversations/{conversationId}/members | Get the members of a chat conversation.
[**get_webchat_guest_conversation_message**](WebChatApi.md#get_webchat_guest_conversation_message) | **GET** /api/v2/webchat/guest/conversations/{conversationId}/messages/{messageId} | Get a web chat conversation message
[**get_webchat_guest_conversation_messages**](WebChatApi.md#get_webchat_guest_conversation_messages) | **GET** /api/v2/webchat/guest/conversations/{conversationId}/messages | Get the messages of a chat conversation.
[**get_webchat_settings**](WebChatApi.md#get_webchat_settings) | **GET** /api/v2/webchat/settings | Get WebChat deployment settings
[**patch_webchat_guest_conversation_mediarequest**](WebChatApi.md#patch_webchat_guest_conversation_mediarequest) | **PATCH** /api/v2/webchat/guest/conversations/{conversationId}/mediarequests/{mediaRequestId} | Update a media request in the conversation, setting the state to ACCEPTED/DECLINED/ERRORED
[**post_webchat_deployments**](WebChatApi.md#post_webchat_deployments) | **POST** /api/v2/webchat/deployments | Create WebChat deployment
[**post_webchat_guest_conversation_member_messages**](WebChatApi.md#post_webchat_guest_conversation_member_messages) | **POST** /api/v2/webchat/guest/conversations/{conversationId}/members/{memberId}/messages | Send a message in a chat conversation.
[**post_webchat_guest_conversation_member_typing**](WebChatApi.md#post_webchat_guest_conversation_member_typing) | **POST** /api/v2/webchat/guest/conversations/{conversationId}/members/{memberId}/typing | Send a typing-indicator in a chat conversation.
[**post_webchat_guest_conversations**](WebChatApi.md#post_webchat_guest_conversations) | **POST** /api/v2/webchat/guest/conversations | Create an ACD chat conversation from an external customer.
[**put_webchat_deployment**](WebChatApi.md#put_webchat_deployment) | **PUT** /api/v2/webchat/deployments/{deploymentId} | Update a WebChat deployment
[**put_webchat_settings**](WebChatApi.md#put_webchat_settings) | **PUT** /api/v2/webchat/settings | Update WebChat deployment settings



## delete_webchat_deployment

> delete_webchat_deployment(deployment_id)
Delete a WebChat deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Deployment Id | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webchat_guest_conversation_member

> delete_webchat_guest_conversation_member(conversation_id, member_id)
Remove a member from a chat conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**member_id** | **String** | memberId | [required] |

### Return type

 (empty response body)

### Authorization

[Guest Chat JWT](../README.md#Guest Chat JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webchat_settings

> delete_webchat_settings()
Remove WebChat deployment settings

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webchat_deployment

> crate::models::WebChatDeployment get_webchat_deployment(deployment_id)
Get a WebChat deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Deployment Id | [required] |

### Return type

[**crate::models::WebChatDeployment**](WebChatDeployment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webchat_deployments

> crate::models::WebChatDeploymentEntityListing get_webchat_deployments()
List WebChat deployments

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebChatDeploymentEntityListing**](WebChatDeploymentEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webchat_guest_conversation_mediarequest

> crate::models::WebChatGuestMediaRequest get_webchat_guest_conversation_mediarequest(conversation_id, media_request_id)
Get a media request in the conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**media_request_id** | **String** | mediaRequestId | [required] |

### Return type

[**crate::models::WebChatGuestMediaRequest**](WebChatGuestMediaRequest.md)

### Authorization

[Guest Chat JWT](../README.md#Guest Chat JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webchat_guest_conversation_mediarequests

> crate::models::WebChatGuestMediaRequestEntityList get_webchat_guest_conversation_mediarequests(conversation_id)
Get all media requests to the guest in the conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::WebChatGuestMediaRequestEntityList**](WebChatGuestMediaRequestEntityList.md)

### Authorization

[Guest Chat JWT](../README.md#Guest Chat JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webchat_guest_conversation_member

> crate::models::WebChatMemberInfo get_webchat_guest_conversation_member(conversation_id, member_id)
Get a web chat conversation member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**member_id** | **String** | memberId | [required] |

### Return type

[**crate::models::WebChatMemberInfo**](WebChatMemberInfo.md)

### Authorization

[Guest Chat JWT](../README.md#Guest Chat JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webchat_guest_conversation_members

> crate::models::WebChatMemberInfoEntityList get_webchat_guest_conversation_members(conversation_id, page_size, page_number, exclude_disconnected_members)
Get the members of a chat conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**page_size** | Option<**i32**> | The number of entries to return per page, or omitted for the default. |  |[default to 25]
**page_number** | Option<**i32**> | The page number to return, or omitted for the first page. |  |[default to 1]
**exclude_disconnected_members** | Option<**bool**> | If true, the results will not contain members who have a DISCONNECTED state. |  |[default to false]

### Return type

[**crate::models::WebChatMemberInfoEntityList**](WebChatMemberInfoEntityList.md)

### Authorization

[Guest Chat JWT](../README.md#Guest Chat JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webchat_guest_conversation_message

> crate::models::WebChatMessage get_webchat_guest_conversation_message(conversation_id, message_id)
Get a web chat conversation message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**message_id** | **String** | messageId | [required] |

### Return type

[**crate::models::WebChatMessage**](WebChatMessage.md)

### Authorization

[Guest Chat JWT](../README.md#Guest Chat JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webchat_guest_conversation_messages

> crate::models::WebChatMessageEntityList get_webchat_guest_conversation_messages(conversation_id, after, before, sort_order, max_results)
Get the messages of a chat conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**after** | Option<**String**> | If available, get the messages chronologically after the id of this message |  |
**before** | Option<**String**> | If available, get the messages chronologically before the id of this message |  |
**sort_order** | Option<**String**> | Sort order |  |[default to ascending]
**max_results** | Option<**i32**> | Limit the returned number of messages, up to a maximum of 100 |  |[default to 100]

### Return type

[**crate::models::WebChatMessageEntityList**](WebChatMessageEntityList.md)

### Authorization

[Guest Chat JWT](../README.md#Guest Chat JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webchat_settings

> crate::models::WebChatSettings get_webchat_settings()
Get WebChat deployment settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebChatSettings**](WebChatSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_webchat_guest_conversation_mediarequest

> crate::models::WebChatGuestMediaRequest patch_webchat_guest_conversation_mediarequest(conversation_id, media_request_id, body)
Update a media request in the conversation, setting the state to ACCEPTED/DECLINED/ERRORED

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**media_request_id** | **String** | mediaRequestId | [required] |
**body** | [**WebChatGuestMediaRequest**](WebChatGuestMediaRequest.md) | Request | [required] |

### Return type

[**crate::models::WebChatGuestMediaRequest**](WebChatGuestMediaRequest.md)

### Authorization

[Guest Chat JWT](../README.md#Guest Chat JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_webchat_deployments

> crate::models::WebChatDeployment post_webchat_deployments(body)
Create WebChat deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WebChatDeployment**](WebChatDeployment.md) | Deployment | [required] |

### Return type

[**crate::models::WebChatDeployment**](WebChatDeployment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_webchat_guest_conversation_member_messages

> crate::models::WebChatMessage post_webchat_guest_conversation_member_messages(conversation_id, member_id, body)
Send a message in a chat conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**member_id** | **String** | memberId | [required] |
**body** | [**CreateWebChatMessageRequest**](CreateWebChatMessageRequest.md) | Message | [required] |

### Return type

[**crate::models::WebChatMessage**](WebChatMessage.md)

### Authorization

[Guest Chat JWT](../README.md#Guest Chat JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_webchat_guest_conversation_member_typing

> crate::models::WebChatTyping post_webchat_guest_conversation_member_typing(conversation_id, member_id)
Send a typing-indicator in a chat conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**member_id** | **String** | memberId | [required] |

### Return type

[**crate::models::WebChatTyping**](WebChatTyping.md)

### Authorization

[Guest Chat JWT](../README.md#Guest Chat JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_webchat_guest_conversations

> crate::models::CreateWebChatConversationResponse post_webchat_guest_conversations(body)
Create an ACD chat conversation from an external customer.

This endpoint will create a new ACD Chat conversation under the specified Chat Deployment.  The conversation will begin with a guest member in it (with a role=CUSTOMER) according to the customer information that is supplied. If the guest member is authenticated, the 'memberAuthToken' field should include his JWT as generated by the 'POST /api/v2/signeddata' resource; if the guest member is anonymous (and the Deployment permits it) this field can be omitted.  The returned data includes the IDs of the conversation created, along with a newly-create JWT token that you can supply to all future endpoints as authentication to perform operations against that conversation. After successfully creating a conversation, you should connect a websocket to the event stream named in the 'eventStreamUri' field of the response; the conversation is not routed until the event stream is attached.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateWebChatConversationRequest**](CreateWebChatConversationRequest.md) | CreateConversationRequest | [required] |

### Return type

[**crate::models::CreateWebChatConversationResponse**](CreateWebChatConversationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_webchat_deployment

> crate::models::WebChatDeployment put_webchat_deployment(deployment_id, body)
Update a WebChat deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Deployment Id | [required] |
**body** | [**WebChatDeployment**](WebChatDeployment.md) | Deployment | [required] |

### Return type

[**crate::models::WebChatDeployment**](WebChatDeployment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_webchat_settings

> crate::models::WebChatSettings put_webchat_settings(body)
Update WebChat deployment settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WebChatSettings**](WebChatSettings.md) | webChatSettings | [required] |

### Return type

[**crate::models::WebChatSettings**](WebChatSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

