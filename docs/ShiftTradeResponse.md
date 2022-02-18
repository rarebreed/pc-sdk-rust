# ShiftTradeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of this shift trade | [optional]
**week_date** | Option<[**String**](string.md)> | The start week date of the associated schedule in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional]
**schedule** | Option<[**crate::models::BuScheduleReferenceForMuRoute**](BuScheduleReferenceForMuRoute.md)> |  | [optional]
**state** | Option<**String**> | The state of this shift trade | [optional]
**initiating_user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**initiating_shift_id** | Option<**String**> | The ID of the shift offered for trade by the initiating user | [optional]
**initiating_shift_start** | Option<**String**> | The start date/time of the shift being offered for trade. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**initiating_shift_end** | Option<**String**> | The end date/time of the shift being offered for trade. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**receiving_user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**receiving_shift_id** | Option<**String**> | The ID of the shift being exchanged for the initiating shift, null if the receiving user is picking up a shift | [optional]
**receiving_shift_start** | Option<**String**> | The start date/time of the receiving shift. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**receiving_shift_end** | Option<**String**> | The end date/time of the receiving shift. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**expiration** | Option<**String**> | When this shift trade offer will expire if not matched or approved. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**one_sided** | Option<**bool**> | Whether this is a one-sided shift trade (e.g. the initiating user is not asking for a shift in return) | [optional]
**acceptable_intervals** | Option<**Vec<String>**> |  | [optional]
**reviewed_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**reviewed_date** | Option<**String**> | The timestamp when this shift trade was reviewed. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


