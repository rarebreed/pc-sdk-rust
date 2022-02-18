# DependencyStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**client** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**build_id** | Option<**String**> |  | [optional]
**date_started** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_completed** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**status** | Option<**String**> |  | [optional]
**failed_objects** | Option<[**Vec<crate::models::FailedObject>**](FailedObject.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


