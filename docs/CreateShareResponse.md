# CreateShareResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**shared_entity_type** | Option<**String**> |  | [optional]
**shared_entity** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**member_type** | Option<**String**> |  | [optional]
**member** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**shared_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**workspace** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**succeeded** | Option<[**Vec<crate::models::Share>**](Share.md)> |  | [optional]
**failed** | Option<[**Vec<crate::models::Share>**](Share.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


