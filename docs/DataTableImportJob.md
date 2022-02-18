# DataTableImportJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**owner** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**status** | **String** | The status of the import job | 
**date_created** | Option<**String**> | The timestamp of when the import began. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_completed** | Option<**String**> | The timestamp of when the import stopped (either successfully or unsuccessfully). Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**upload_uri** | Option<**String**> | The URL of the location at which the caller can upload the file to be imported | [optional]
**import_mode** | Option<**String**> | The indication of whether the processing should remove rows that don't appear in the import file | [optional]
**error_information** | Option<[**crate::models::ErrorBody**](ErrorBody.md)> |  | [optional]
**count_records_updated** | Option<**i32**> | The current count of the number of records processed | [optional]
**count_records_deleted** | Option<**i32**> | The current count of the number of records deleted | [optional]
**count_records_failed** | Option<**i32**> | The current count of the number of records that failed to import | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


