# RoutingRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operator** | Option<**String**> | matching operator.  MEETS_THRESHOLD matches any agent with a score at or above the rule's threshold.  ANY matches all specified agents, regardless of score. | [optional]
**threshold** | Option<**i32**> | threshold required for routing attempt (generally an agent score).  may be null for operator ANY. | [optional]
**wait_seconds** | Option<**f64**> | seconds to wait in this rule before moving to the next | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


