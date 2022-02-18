# KnowledgeSearchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**search_id** | Option<**String**> | Search Id | [optional][readonly]
**total** | Option<**i32**> | Total number of records returned | [optional][readonly]
**page_count** | Option<**i32**> | Number of pages returned in the result calculated according to the pageSize and the total | [optional][readonly]
**page_size** | Option<**i32**> | Number of records according to the page size | [optional][readonly]
**page_number** | Option<**i32**> | Current page number for the returned records | [optional][readonly]
**results** | Option<[**Vec<crate::models::KnowledgeSearchDocument>**](KnowledgeSearchDocument.md)> | Results associated to the search response | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


