# WebChatGuestMediaRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**types** | **Vec<String>** | The types of media being requested. | 
**state** | **String** | The state of the media request, one of PENDING|ACCEPTED|DECLINED|TIMEDOUT|CANCELLED|ERRORED. | 
**communication_id** | Option<**String**> | The ID of the new media communication, if applicable. | [optional]
**security_key** | Option<**String**> | The security information related to a media request. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


