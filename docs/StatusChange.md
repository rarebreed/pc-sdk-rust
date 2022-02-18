# StatusChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_status_changed** | Option<**String**> | The date of this status change. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**status** | Option<**String**> | The status the change request transitioned to | [optional][readonly]
**previous_status** | Option<**String**> | The status the change request transitioned from | [optional][readonly]
**message** | Option<**String**> | A short message describing the status change | [optional][readonly]
**changed_by** | Option<**String**> | If applicable, the user who updated the change request to this status | [optional][readonly]
**reject_reason** | Option<**String**> | The reason for rejecting the limit override request | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


