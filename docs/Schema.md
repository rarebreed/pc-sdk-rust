# Schema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | A core type's title | [optional][readonly]
**description** | Option<**String**> | A core type's description | [optional][readonly]
**_type** | Option<**Vec<String>**> | An array of fundamental JSON Schema primitive types on which the core type is based | [optional][readonly]
**items** | Option<[**crate::models::Items**](Items.md)> |  | [optional]
**pattern** | Option<**String**> | For the \"date\" and \"datetime\" core types, denotes the regex prescribing the allowable date/datetime format | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


