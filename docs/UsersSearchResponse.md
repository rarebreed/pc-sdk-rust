# UsersSearchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total** | **i64** | The total number of results found | 
**page_count** | **i32** | The total number of pages | 
**page_size** | **i32** | The current page size | 
**page_number** | **i32** | The current page number | 
**previous_page** | Option<**String**> | Q64 value for the previous page of results | [optional]
**current_page** | Option<**String**> | Q64 value for the current page of results | [optional]
**next_page** | Option<**String**> | Q64 value for the next page of results | [optional]
**types** | **Vec<String>** | Resource types the search was performed against | 
**results** | [**Vec<crate::models::User>**](User.md) | Search results | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


