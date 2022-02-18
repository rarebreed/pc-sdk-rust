# Geolocation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**_type** | Option<**String**> | A string used to describe the type of client the geolocation is being updated from e.g. ios, android, web, etc. | [optional]
**primary** | Option<**bool**> | A boolean used to tell whether or not to set this geolocation client as the primary on a PATCH | [optional]
**latitude** | Option<**f64**> |  | [optional]
**longitude** | Option<**f64**> |  | [optional]
**country** | Option<**String**> |  | [optional]
**region** | Option<**String**> |  | [optional]
**city** | Option<**String**> |  | [optional]
**locations** | Option<[**Vec<crate::models::LocationDefinition>**](LocationDefinition.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


