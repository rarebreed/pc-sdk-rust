# ScimV2SchemaListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schemas** | Option<**Vec<String>**> | The list of supported schemas. | [optional]
**total_results** | Option<**i64**> | The total number of results. | [optional][readonly]
**start_index** | Option<**i64**> | The 1-based index of the first result returned by this request. Add this to \"itemsPerPage\" when requesting the next page of results. | [optional][readonly]
**items_per_page** | Option<**i64**> | The number of resources returned per page. | [optional][readonly]
**resources** | Option<[**Vec<crate::models::ScimV2SchemaDefinition>**](ScimV2SchemaDefinition.md)> | The list of requested resources. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


