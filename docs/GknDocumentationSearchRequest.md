# GknDocumentationSearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sort_order** | Option<**String**> | The sort order for results | [optional]
**sort_by** | Option<**String**> | The field in the resource that you want to sort the results by | [optional]
**page_size** | Option<**i32**> | The number of results per page | [optional]
**page_number** | Option<**i32**> | The page of resources you want to retrieve | [optional]
**sort** | Option<[**Vec<crate::models::SearchSort>**](SearchSort.md)> | Multi-value sort order, list of multiple sort values | [optional]
**query** | Option<[**Vec<crate::models::GknDocumentationSearchCriteria>**](GKNDocumentationSearchCriteria.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


