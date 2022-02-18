# OutboundSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**date_created** | Option<**String**> | Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**version** | Option<**i32**> | Required for updates, must match the version number of the most recent update | [optional]
**max_calls_per_agent** | Option<**i32**> | The maximum number of calls that can be placed per agent on any campaign | [optional]
**max_configurable_calls_per_agent** | Option<**i32**> | The maximum number of calls that can be configured to be placed per agent on any campaign | [optional][readonly]
**max_line_utilization** | Option<**f64**> | The maximum percentage of lines that should be used for Outbound, expressed as a decimal in the range [0.0, 1.0] | [optional]
**abandon_seconds** | Option<**f64**> | The number of seconds used to determine if a call is abandoned | [optional]
**compliance_abandon_rate_denominator** | Option<**String**> | The denominator to be used in determining the compliance abandon rate | [optional]
**automatic_time_zone_mapping** | Option<[**crate::models::AutomaticTimeZoneMappingSettings**](AutomaticTimeZoneMappingSettings.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


