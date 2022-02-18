# SearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sort_order** | Option<**String**> | The sort order for results | [optional]
**sort_by** | Option<**String**> | The field in the resource that you want to sort the results by | [optional]
**page_size** | Option<**i32**> | The number of results per page | [optional]
**page_number** | Option<**i32**> | The page of resources you want to retrieve | [optional]
**sort** | Option<[**Vec<crate::models::SearchSort>**](SearchSort.md)> | Multi-value sort order, list of multiple sort values | [optional]
**return_fields** | Option<**Vec<String>**> | A List of strings.  Possible values are any field in the resource you are searching on.  The other option is to use ALL_FIELDS, when this is provided all fields in the resource will be returned in the search results. | [optional]
**expand** | Option<**Vec<String>**> | Provides more details about a specified resource | [optional]
**types** | **Vec<String>** | Resource domain type to search | 
**query** | Option<[**Vec<crate::models::SearchCriteria>**](SearchCriteria.md)> | The search criteria | [optional]
**aggregations** | Option<[**Vec<crate::models::SearchAggregation>**](SearchAggregation.md)> | Aggregation criteria | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


