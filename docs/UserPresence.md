# UserPresence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**source** | Option<**String**> | Represents the source where the Presence was set. Some examples are: PURECLOUD, LYNC, OUTLOOK, etc. | [optional]
**primary** | Option<**bool**> | A boolean used to tell whether or not to set this presence source as the primary on a PATCH | [optional]
**presence_definition** | Option<[**crate::models::PresenceDefinition**](PresenceDefinition.md)> |  | [optional]
**message** | Option<**String**> |  | [optional]
**modified_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


