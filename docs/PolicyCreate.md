# PolicyCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The policy name. | 
**modified_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**created_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**order** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**media_policies** | Option<[**crate::models::MediaPolicies**](MediaPolicies.md)> |  | [optional]
**conditions** | Option<[**crate::models::PolicyConditions**](PolicyConditions.md)> |  | [optional]
**actions** | Option<[**crate::models::PolicyActions**](PolicyActions.md)> |  | [optional]
**policy_errors** | Option<[**crate::models::PolicyErrors**](PolicyErrors.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


