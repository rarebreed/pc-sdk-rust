# AuthzDivision

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**description** | **String** | A helpful description for the division. | 
**home_division** | Option<**bool**> | A flag indicating whether this division is the \"Home\" (default) division. Cannot be modified and any supplied value will be ignored on create or update. | [optional][readonly]
**object_counts** | Option<**::std::collections::HashMap<String, i64>**> | A count of objects in this division, grouped by type. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


