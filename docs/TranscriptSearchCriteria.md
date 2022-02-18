# TranscriptSearchCriteria

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_value** | Option<**String**> | The end value of the range. This field is used for range search types. | [optional]
**values** | Option<**Vec<String>**> | A list of values for the search to match against | [optional]
**start_value** | Option<**String**> | The start value of the range. This field is used for range search types. | [optional]
**fields** | Option<**Vec<String>**> | Field names to search against | [optional]
**value** | Option<**String**> | A value for the search to match against | [optional]
**operator** | Option<**String**> | How to apply this search criteria against other criteria | [optional]
**group** | Option<[**Vec<crate::models::TranscriptSearchCriteria>**](TranscriptSearchCriteria.md)> | Groups multiple conditions | [optional]
**date_format** | Option<**String**> | Set date format for criteria values when using date range search type.  Supports Java date format syntax, example yyyy-MM-dd'T'HH:mm:ss.SSSX. | [optional]
**_type** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


