# Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**version** | Option<**i32**> | Version number required for updates. | [optional]
**libraries** | [**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md) | One or more libraries response is associated with. | 
**texts** | [**Vec<crate::models::ResponseText>**](ResponseText.md) | One or more texts associated with the response. | 
**created_by** | Option<[**crate::models::User**](User.md)> |  | [optional]
**date_created** | Option<**String**> | The date and time the response was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**interaction_type** | Option<**String**> | The interaction type for this response. | [optional]
**substitutions** | Option<[**Vec<crate::models::ResponseSubstitution>**](ResponseSubstitution.md)> | Details about any text substitutions used in the texts for this response. | [optional]
**substitutions_schema** | Option<[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)> |  | [optional]
**response_type** | Option<**String**> | The response type represented by the response. | [optional]
**messaging_template** | Option<[**crate::models::MessagingTemplate**](MessagingTemplate.md)> |  | [optional]
**assets** | Option<[**Vec<crate::models::AddressableEntityRef>**](AddressableEntityRef.md)> | Assets used in the response | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


