# UserDetailsQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interval** | **String** | Specifies the date and time range of data being queried. Conversations MUST have started within this time range to potentially be included within the result set. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**user_filters** | Option<[**Vec<crate::models::UserDetailQueryFilter>**](UserDetailQueryFilter.md)> | Filters that target the users to retrieve data for | [optional]
**presence_filters** | Option<[**Vec<crate::models::PresenceDetailQueryFilter>**](PresenceDetailQueryFilter.md)> | Filters that target system and organization presence-level data | [optional]
**routing_status_filters** | Option<[**Vec<crate::models::RoutingStatusDetailQueryFilter>**](RoutingStatusDetailQueryFilter.md)> | Filters that target agent routing status-level data | [optional]
**order** | Option<**String**> | Sort the result set in ascending/descending order. Default is ascending | [optional]
**presence_aggregations** | Option<[**Vec<crate::models::AnalyticsQueryAggregation>**](AnalyticsQueryAggregation.md)> | Include faceted search and aggregate roll-ups of presence data in your search results. This does not function as a filter, but rather, summary data about the presence results matching your filters | [optional]
**routing_status_aggregations** | Option<[**Vec<crate::models::AnalyticsQueryAggregation>**](AnalyticsQueryAggregation.md)> | Include faceted search and aggregate roll-ups of agent routing status data in your search results. This does not function as a filter, but rather, summary data about the agent routing status results matching your filters | [optional]
**paging** | Option<[**crate::models::PagingSpec**](PagingSpec.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


