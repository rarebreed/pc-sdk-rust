# CreateWebChatConversationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **String** | The organization identifier. | 
**deployment_id** | **String** | The web chat Deployment ID which contains the appropriate settings for this chat conversation. | 
**routing_target** | [**crate::models::WebChatRoutingTarget**](WebChatRoutingTarget.md) |  | 
**member_info** | [**crate::models::GuestMemberInfo**](GuestMemberInfo.md) |  | 
**member_auth_token** | Option<**String**> | If the guest member is an authenticated member (ie, not anonymous) his JWT is provided here. The token will have been previously generated with the \"POST /api/v2/signeddata\" resource. | [optional]
**journey_context** | Option<[**crate::models::JourneyContext**](JourneyContext.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


