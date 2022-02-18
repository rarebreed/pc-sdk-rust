# AsyncUserDetailsQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interval** | **String** | Specifies the date and time range of data being queried. Conversations MUST have started within this time range to potentially be included within the result set. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**user_filters** | Option<[**Vec<crate::models::UserDetailQueryFilter>**](UserDetailQueryFilter.md)> | Filters that target the users to retrieve data for | [optional]
**presence_filters** | Option<[**Vec<crate::models::PresenceDetailQueryFilter>**](PresenceDetailQueryFilter.md)> | Filters that target system and organization presence-level data | [optional]
**routing_status_filters** | Option<[**Vec<crate::models::RoutingStatusDetailQueryFilter>**](RoutingStatusDetailQueryFilter.md)> | Filters that target agent routing status-level data | [optional]
**order** | Option<**String**> | Sort the result set in ascending/descending order. Default is ascending | [optional]
**limit** | Option<**i32**> | Specify number of results to be returned | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


