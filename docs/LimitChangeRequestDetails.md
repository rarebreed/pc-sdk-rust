# LimitChangeRequestDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**key** | **String** | Limit key to be overridden (see https://developer.mypurecloud.com/api/rest/v2/organization/limits.html#available_limits) | 
**namespace** | **String** | Namespace the key belongs to (see https://developer.mypurecloud.com/api/rest/v2/organization/limits.html#available_limits) | 
**requested_value** | **f64** | Requested limit value for a given key | 
**description** | **String** | Description of the need for the limit change request | 
**support_case_url** | **String** | The support case url created by Care | 
**created_by** | Option<**String**> | The user who created the change request | [optional][readonly]
**status** | Option<**String**> | Current status of the limit change request | [optional][readonly]
**current_value** | Option<**f64**> | Current limit value for a given key | [optional][readonly]
**date_created** | Option<**String**> | The date of the limit change request creation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**status_history** | Option<[**Vec<crate::models::StatusChange>**](StatusChange.md)> | List of statuses that a limit change request has gone through | [optional][readonly]
**date_completed** | Option<**String**> | The date of the limit change request completion (ChangeImplemented, Rejected, or RollbackImplemented. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**last_changed_by** | Option<**String**> | The user who last updated the status of the limit change request | [optional][readonly]
**reject_reason** | Option<**String**> | The reason for rejecting the limit override request | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


