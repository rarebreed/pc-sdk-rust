# CoachingAnnotation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**created_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_created** | Option<**String**> | The date/time the annotation was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**modified_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_modified** | Option<**String**> | The date/time the annotation was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**text** | **String** | The text of the annotation. | 
**is_deleted** | Option<**bool**> | Flag indicating whether the annotation is deleted. | [optional][readonly]
**access_type** | Option<**String**> | Determines the permissions required to view this item. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


