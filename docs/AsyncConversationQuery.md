# AsyncConversationQuery

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
**interval** | **String** | Specifies the date and time range of data being queried. Results will include all conversations that had activity during the interval. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**limit** | Option<**i32**> | Specify number of results to be returned | [optional]
**start_of_day_interval_matching** | Option<**bool**> | Add a filter to only include conversations that started after the beginning of the interval start date (UTC) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


