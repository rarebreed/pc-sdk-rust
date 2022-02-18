# ReportingExportMetadataJobResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**view_type** | Option<**String**> | The view type of the export metadata | [optional]
**date_limitations** | Option<**String**> | The date limitations of the export metadata | [optional]
**required_filters** | Option<**Vec<String>**> | The list of required filters for the export metadata | [optional]
**supported_filters** | Option<**Vec<String>**> | The list of supported filters for the export metadata | [optional]
**required_column_ids** | Option<**Vec<String>**> | The list of required column ids for the export metadata | [optional]
**dependent_column_ids** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | The list of dependent column ids for the export metadata | [optional]
**available_column_ids** | Option<**Vec<String>**> | The list of available column ids for the export metadata | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


