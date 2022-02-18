# DidNumber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**number** | Option<**String**> | The number of the DID formatted as E164. | [optional]
**assigned** | Option<**bool**> | True if this DID is assigned to an entity.  False otherwise. | [optional]
**did_pool** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**owner** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**owner_type** | Option<**String**> | The type of the entity that owns this DID.  If the DID is unassigned, this will be NULL. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


