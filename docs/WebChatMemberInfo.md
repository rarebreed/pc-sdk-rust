# WebChatMemberInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The communicationId of this member. | [optional]
**display_name** | Option<**String**> | The display name of the member. | [optional]
**first_name** | Option<**String**> | The first name of the member. | [optional]
**last_name** | Option<**String**> | The last name of the member. | [optional]
**email** | Option<**String**> | The email address of the member. | [optional]
**phone_number** | Option<**String**> | The phone number of the member. | [optional]
**avatar_image_url** | Option<**String**> | The url to the avatar image of the member. | [optional]
**role** | **String** | The role of the member, one of [agent, customer, acd, workflow] | 
**join_date** | Option<**String**> | The time the member joined the conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**leave_date** | Option<**String**> | The time the member left the conversation, or null if the member is still active in the conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**authenticated_guest** | Option<**bool**> | If true, the guest member is an authenticated guest. | [optional]
**custom_fields** | Option<**::std::collections::HashMap<String, String>**> | Any custom fields of information pertaining to this member. | [optional]
**state** | Option<**String**> | The connection state of this member. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


