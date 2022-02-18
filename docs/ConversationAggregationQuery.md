# ConversationAggregationQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interval** | **String** | Behaves like one clause in a SQL WHERE. Specifies the date and time range of data being queried. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**granularity** | Option<**String**> | Granularity aggregates metrics into subpartitions within the time interval specified. The default granularity is the same duration as the interval. Periods are represented as an ISO-8601 string. For example: P1D or P1DT12H | [optional]
**time_zone** | Option<**String**> | Time zone context used to calculate response intervals (this allows resolving DST changes). The interval offset is used even when timeZone is specified. Default is UTC. Time zones are represented as a string of the zone name as found in the IANA time zone database. For example: UTC, Etc/UTC, or Europe/London | [optional]
**group_by** | Option<**Vec<String>**> | Behaves like a SQL GROUPBY. Allows for multiple levels of grouping as a list of dimensions. Partitions resulting aggregate computations into distinct named subgroups rather than across the entire result set as if it were one group. | [optional]
**filter** | Option<[**crate::models::ConversationAggregateQueryFilter**](ConversationAggregateQueryFilter.md)> |  | [optional]
**metrics** | **Vec<String>** | Behaves like a SQL SELECT clause. Only named metrics will be retrieved. | 
**flatten_multivalued_dimensions** | Option<**bool**> | Flattens any multivalued dimensions used in response groups (e.g. ['a','b','c']->'a,b,c') | [optional]
**views** | Option<[**Vec<crate::models::ConversationAggregationView>**](ConversationAggregationView.md)> | Custom derived metric views | [optional]
**alternate_time_dimension** | Option<**String**> | Dimension to use as the alternative timestamp for data in the aggregate.  Choosing \"eventTime\" uses the actual time of the data event. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


