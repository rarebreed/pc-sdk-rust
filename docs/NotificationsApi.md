# \NotificationsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_notifications_channel_subscriptions**](NotificationsApi.md#delete_notifications_channel_subscriptions) | **DELETE** /api/v2/notifications/channels/{channelId}/subscriptions | Remove all subscriptions
[**get_notifications_availabletopics**](NotificationsApi.md#get_notifications_availabletopics) | **GET** /api/v2/notifications/availabletopics | Get available notification topics.
[**get_notifications_channel_subscriptions**](NotificationsApi.md#get_notifications_channel_subscriptions) | **GET** /api/v2/notifications/channels/{channelId}/subscriptions | The list of all subscriptions for this channel
[**get_notifications_channels**](NotificationsApi.md#get_notifications_channels) | **GET** /api/v2/notifications/channels | The list of existing channels
[**head_notifications_channel**](NotificationsApi.md#head_notifications_channel) | **HEAD** /api/v2/notifications/channels/{channelId} | Verify a channel still exists and is valid
[**post_notifications_channel_subscriptions**](NotificationsApi.md#post_notifications_channel_subscriptions) | **POST** /api/v2/notifications/channels/{channelId}/subscriptions | Add a list of subscriptions to the existing list of subscriptions
[**post_notifications_channels**](NotificationsApi.md#post_notifications_channels) | **POST** /api/v2/notifications/channels | Create a new channel
[**put_notifications_channel_subscriptions**](NotificationsApi.md#put_notifications_channel_subscriptions) | **PUT** /api/v2/notifications/channels/{channelId}/subscriptions | Replace the current list of subscriptions with a new list.



## delete_notifications_channel_subscriptions

> delete_notifications_channel_subscriptions(channel_id)
Remove all subscriptions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_availabletopics

> crate::models::AvailableTopicEntityListing get_notifications_availabletopics(expand, include_preview)
Get available notification topics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |
**include_preview** | Option<**bool**> | Whether or not to include Preview topics |  |[default to true]

### Return type

[**crate::models::AvailableTopicEntityListing**](AvailableTopicEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_channel_subscriptions

> crate::models::ChannelTopicEntityListing get_notifications_channel_subscriptions(channel_id)
The list of all subscriptions for this channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel ID | [required] |

### Return type

[**crate::models::ChannelTopicEntityListing**](ChannelTopicEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_channels

> crate::models::ChannelEntityListing get_notifications_channels(includechannels)
The list of existing channels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**includechannels** | Option<**String**> | Show user's channels for this specific token or across all tokens for this user and app.  Channel Ids for other access tokens will not be shown, but will be presented to show their existence. |  |[default to token]

### Return type

[**crate::models::ChannelEntityListing**](ChannelEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_notifications_channel

> head_notifications_channel(channel_id)
Verify a channel still exists and is valid

Returns a 200 OK if channel exists, and a 404 Not Found if it doesn't

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_notifications_channel_subscriptions

> crate::models::ChannelTopicEntityListing post_notifications_channel_subscriptions(channel_id, body)
Add a list of subscriptions to the existing list of subscriptions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel ID | [required] |
**body** | [**Vec<crate::models::ChannelTopic>**](ChannelTopic.md) | Body | [required] |

### Return type

[**crate::models::ChannelTopicEntityListing**](ChannelTopicEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_notifications_channels

> crate::models::Channel post_notifications_channels()
Create a new channel

There is a limit of 20 channels per user/app combination. Creating a 21st channel will remove the channel with oldest last used date. Channels without an active connection will be removed first.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_notifications_channel_subscriptions

> crate::models::ChannelTopicEntityListing put_notifications_channel_subscriptions(channel_id, body)
Replace the current list of subscriptions with a new list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel ID | [required] |
**body** | [**Vec<crate::models::ChannelTopic>**](ChannelTopic.md) | Body | [required] |

### Return type

[**crate::models::ChannelTopicEntityListing**](ChannelTopicEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

