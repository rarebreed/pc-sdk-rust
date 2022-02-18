# Coretype

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**version** | Option<**i32**> | A positive integer denoting the core type's version | [optional]
**date_created** | Option<**String**> | The date the core type was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**schema** | Option<[**crate::models::Schema**](Schema.md)> |  | [optional]
**current** | Option<**bool**> | A boolean indicating if the core type's version is the current one in use by the system | [optional]
**validation_fields** | Option<**Vec<String>**> | An array of strings naming the fields of the core type subject to validation.  Validation constraints are specified by a schema author using the core type. | [optional]
**validation_limits** | Option<[**crate::models::ValidationLimits**](ValidationLimits.md)> |  | [optional]
**item_validation_fields** | Option<**Vec<String>**> | Specific to the \"tag\" core type, this is an array of strings naming the tag item fields of the core type subject to validation | [optional]
**item_validation_limits** | Option<[**crate::models::ItemValidationLimits**](ItemValidationLimits.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


