# PatchOutcome

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**is_active** | Option<**bool**> | Whether or not the outcome is active. | [optional]
**display_name** | **String** | The display name of the outcome. | 
**version** | Option<**i32**> | The version of the outcome. | [optional]
**description** | Option<**String**> | A description of the outcome. | [optional]
**is_positive** | Option<**bool**> | Whether or not the outcome is positive. | [optional]
**context** | Option<[**crate::models::Context**](Context.md)> |  | [optional]
**journey** | Option<[**crate::models::Journey**](Journey.md)> |  | [optional]
**associated_value_field** | Option<[**crate::models::AssociatedValueField**](AssociatedValueField.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**created_date** | Option<**String**> | Timestamp indicating when the outcome was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**modified_date** | Option<**String**> | Timestamp indicating when the outcome was last updated. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


