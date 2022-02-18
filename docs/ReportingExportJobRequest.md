# ReportingExportJobRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The user supplied name of the export request | 
**time_zone** | **String** | The requested timezone of the exported data. Time zones are represented as a string of the zone name as found in the IANA time zone database. For example: UTC, Etc/UTC, or Europe/London | 
**export_format** | **String** | The requested format of the exported data | 
**interval** | **String** | The time period used to limit the the exported data. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**period** | **String** | The Period of the request in which to break down the intervals. Periods are represented as an ISO-8601 string. For example: P1D or P1DT12H | 
**view_type** | **String** | The type of view export job to be created | 
**filter** | [**crate::models::ViewFilter**](ViewFilter.md) |  | 
**read** | Option<**bool**> | Indicates if the request has been marked as read | [optional]
**locale** | **String** | The locale use for localization of the exported data, i.e. en-us, es-mx   | 
**has_format_durations** | Option<**bool**> | Indicates if durations are formatted in hh:mm:ss format instead of ms | [optional]
**has_split_filters** | Option<**bool**> | Indicates if filters will be split in aggregate detail exports | [optional]
**exclude_empty_rows** | Option<**bool**> | Excludes empty rows from the exports | [optional]
**has_split_by_media** | Option<**bool**> | Indicates if media type will be split in aggregate detail exports | [optional]
**has_summary_row** | Option<**bool**> | Indicates if summary row needs to be present in exports | [optional]
**csv_delimiter** | Option<**String**> | The user supplied csv delimiter string value either of type 'comma' or 'semicolon' permitted for the export request | [optional]
**selected_columns** | Option<[**Vec<crate::models::SelectedColumns>**](SelectedColumns.md)> | The list of ordered selected columns from the export view by the user | [optional]
**has_custom_participant_attributes** | Option<**bool**> | Indicates if custom participant attributes will be exported | [optional]
**recipient_emails** | Option<**Vec<String>**> | The list of email recipients for the exports | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


