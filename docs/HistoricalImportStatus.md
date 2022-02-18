# HistoricalImportStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | Request id of the historical import in the organization | [optional][readonly]
**date_import_ended** | Option<**String**> | The last day of the data you are importing. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_import_started** | Option<**String**> | The first day of the data you are importing. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**status** | Option<**String**> | Status of the historical import in the organization. | [optional][readonly]
**error** | Option<**String**> | Error occured if the status of the import is failed | [optional][readonly]
**date_created** | Option<**String**> | Date in which the historical import is initiated. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Date in which the historical import is modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**active** | Option<**bool**> | Whether this historical import is active or not | [optional][readonly]
**_type** | Option<**String**> | Whether this historical import is of type csv or json | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


