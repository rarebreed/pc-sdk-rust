# NluDomain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the NLU domain. | 
**language** | Option<**String**> | The language culture of the NLU domain, e.g. `en-us`, `de-de`. | [optional]
**draft_version** | Option<[**crate::models::NluDomainVersion**](NluDomainVersion.md)> |  | [optional]
**last_published_version** | Option<[**crate::models::NluDomainVersion**](NluDomainVersion.md)> |  | [optional]
**date_created** | Option<**String**> | The date when the NLU domain was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | The date when the NLU domain was updated. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**engine_version** | Option<**String**> | The version of the NLU engine to use. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


