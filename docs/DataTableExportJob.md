# DataTableExportJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**owner** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**status** | **String** | The status of the export job | 
**date_created** | Option<**String**> | The timestamp of when the export began. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_completed** | Option<**String**> | The timestamp of when the export stopped (either successfully or unsuccessfully). Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**download_uri** | Option<**String**> | The URL of the location at which the caller can download the export file, when available | [optional]
**error_information** | Option<[**crate::models::ErrorBody**](ErrorBody.md)> |  | [optional]
**count_records_processed** | Option<**i32**> | The current count of the number of records processed | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


