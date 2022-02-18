# UserSearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sort_order** | Option<**String**> | The sort order for results | [optional]
**sort_by** | Option<**String**> | The field in the resource that you want to sort the results by | [optional]
**page_size** | Option<**i32**> | The number of results per page | [optional]
**page_number** | Option<**i32**> | The page of resources you want to retrieve | [optional]
**sort** | Option<[**Vec<crate::models::SearchSort>**](SearchSort.md)> | Multi-value sort order, list of multiple sort values | [optional]
**expand** | Option<**Vec<String>**> | Provides more details about a specified resource | [optional]
**query** | Option<[**Vec<crate::models::UserSearchCriteria>**](UserSearchCriteria.md)> |  | [optional]
**integration_presence_source** | Option<**String**> | Gets an integration presence for users instead of their defaults. This parameter will only be used when presence is provided as an \"expand\". When using this parameter the maximum number of users that can be returned is 100. | [optional]
**enforce_permissions** | Option<**bool**> | This property only applies to api/v2/user/search; when set to true add additional search criteria to filter users by: directory:user:view | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


