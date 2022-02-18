# JsonSearchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total** | **i64** | The total number of results found | 
**page_count** | **i32** | The total number of pages | 
**page_size** | **i32** | The current page size | 
**page_number** | **i32** | The current page number | 
**types** | **Vec<String>** | Resource types the search was performed against | 
**results** | [**serde_json::Value**](.md) |  | 
**aggregations** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


