# NluDomainVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**domain** | Option<[**crate::models::NluDomain**](NluDomain.md)> |  | [optional]
**description** | Option<**String**> | The description of the NLU domain version. | [optional]
**language** | **String** | The language that the NLU domain version supports. | 
**published** | Option<**bool**> | Whether this NLU domain version has been published. | [optional][readonly]
**date_created** | Option<**String**> | The date when the NLU domain version was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | The date when the NLU domain version was updated. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_trained** | Option<**String**> | The date when the NLU domain version was trained. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_published** | Option<**String**> | The date when the NLU domain version was published. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**training_status** | Option<**String**> | The training status of the NLU domain version. | [optional][readonly]
**evaluation_status** | Option<**String**> | The evaluation status of the NLU domain version. | [optional][readonly]
**intents** | Option<[**Vec<crate::models::IntentDefinition>**](IntentDefinition.md)> | The intents defined for this NLU domain version. | [optional]
**entity_types** | Option<[**Vec<crate::models::NamedEntityTypeDefinition>**](NamedEntityTypeDefinition.md)> | The entity types defined for this NLU domain version. | [optional]
**entities** | Option<[**Vec<crate::models::NamedEntityDefinition>**](NamedEntityDefinition.md)> | The entities defined for this NLU domain version.This field is mutually exclusive with entityTypeBindings | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


