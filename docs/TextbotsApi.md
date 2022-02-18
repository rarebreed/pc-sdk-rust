# \TextbotsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_textbots_bots_search**](TextbotsApi.md#get_textbots_bots_search) | **GET** /api/v2/textbots/bots/search | Find bots using the currently configured friendly name or ID.
[**post_textbots_botflows_session_turns**](TextbotsApi.md#post_textbots_botflows_session_turns) | **POST** /api/v2/textbots/botflows/sessions/{sessionId}/turns | Issue a bot flow turn event
[**post_textbots_botflows_sessions**](TextbotsApi.md#post_textbots_botflows_sessions) | **POST** /api/v2/textbots/botflows/sessions | Create an execution instance of a bot flow definition.
[**post_textbots_bots_execute**](TextbotsApi.md#post_textbots_bots_execute) | **POST** /api/v2/textbots/bots/execute | Send an intent to a bot to start a dialog/interact with it via text



## get_textbots_bots_search

> crate::models::BotSearchResponseEntityListing get_textbots_bots_search(bot_type, bot_name, bot_id, page_size)
Find bots using the currently configured friendly name or ID.

The name does allow case-insensitive partial string matches or by IDs (up to 50), but not both at the same time. Optionally you can limit the scope of the search by providing one or more bot types.  You can specify the maximum results to return, up to a limit of 100

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_type** | Option<[**Vec<String>**](String.md)> | Bot types |  |
**bot_name** | Option<**String**> | Bot name |  |
**bot_id** | Option<[**Vec<String>**](String.md)> | Bot IDs |  |
**page_size** | Option<**i32**> | The maximum results to return |  |[default to 25]

### Return type

[**crate::models::BotSearchResponseEntityListing**](BotSearchResponseEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_textbots_botflows_session_turns

> crate::models::TextBotFlowTurnResponse post_textbots_botflows_session_turns(session_id, turn_request)
Issue a bot flow turn event

Send a turn event to an executing bot flow and produce the next action to take.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The bot flow session ID, typically obtained from 'POST /api/v2/textbots/botflows/sessions' | [required] |
**turn_request** | [**TextBotFlowTurnRequest**](TextBotFlowTurnRequest.md) |  | [required] |

### Return type

[**crate::models::TextBotFlowTurnResponse**](TextBotFlowTurnResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_textbots_botflows_sessions

> crate::models::TextBotFlowLaunchResponse post_textbots_botflows_sessions(launch_request)
Create an execution instance of a bot flow definition.

The launch is asynchronous; use the returned instance ID to post turns to it using 'POST /api/v2/textbots/botflows/sessions/{sessionId}/turns'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**launch_request** | [**TextBotFlowLaunchRequest**](TextBotFlowLaunchRequest.md) |  | [required] |

### Return type

[**crate::models::TextBotFlowLaunchResponse**](TextBotFlowLaunchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_textbots_bots_execute

> crate::models::PostTextResponse post_textbots_bots_execute(post_text_request)
Send an intent to a bot to start a dialog/interact with it via text

This will either start a bot with the given id or relay a communication to an existing bot session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_text_request** | [**PostTextRequest**](PostTextRequest.md) |  | [required] |

### Return type

[**crate::models::PostTextResponse**](PostTextResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

