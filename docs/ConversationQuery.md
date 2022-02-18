# ConversationQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversation_filters** | Option<[**Vec<crate::models::ConversationDetailQueryFilter>**](ConversationDetailQueryFilter.md)> | Filters that target conversation-level data | [optional]
**segment_filters** | Option<[**Vec<crate::models::SegmentDetailQueryFilter>**](SegmentDetailQueryFilter.md)> | Filters that target individual segments within a conversation | [optional]
**evaluation_filters** | Option<[**Vec<crate::models::EvaluationDetailQueryFilter>**](EvaluationDetailQueryFilter.md)> | Filters that target evaluations | [optional]
**survey_filters** | Option<[**Vec<crate::models::SurveyDetailQueryFilter>**](SurveyDetailQueryFilter.md)> | Filters that target surveys | [optional]
**resolution_filters** | Option<[**Vec<crate::models::ResolutionDetailQueryFilter>**](ResolutionDetailQueryFilter.md)> | Filters that target resolutions | [optional]
**order** | Option<**String**> | Sort the result set in ascending/descending order. Default is ascending | [optional]
**order_by** | Option<**String**> | Specify which data element within the result set to use for sorting. The options  to use as a basis for sorting the results: conversationStart, segmentStart, and segmentEnd. If not specified, the default is conversationStart | [optional]
**interval** | **String** | Specifies the date and time range of data being queried. Results will only include conversations that started on a day touched by the interval. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**aggregations** | Option<[**Vec<crate::models::AnalyticsQueryAggregation>**](AnalyticsQueryAggregation.md)> | Include faceted search and aggregate roll-ups describing your search results. This does not function as a filter, but rather, summary data about the data matching your filters | [optional]
**paging** | Option<[**crate::models::PagingSpec**](PagingSpec.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


