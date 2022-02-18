# ResponseQueryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query_phrase** | Option<**String**> | Query phrase to search response text and name. If not set will match all. | [optional]
**page_size** | Option<**i32**> | The maximum number of hits to return. Default: 25, Maximum: 500. | [optional]
**filters** | Option<[**Vec<crate::models::ResponseFilter>**](ResponseFilter.md)> | Filter the query results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


