# AttemptLimits

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**date_created** | Option<**String**> | Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**version** | Option<**i32**> | Required for updates, must match the version number of the most recent update | [optional]
**max_attempts_per_contact** | Option<**i32**> | The maximum number of times a contact can be called within the resetPeriod. Required if maxAttemptsPerNumber is not defined. | [optional]
**max_attempts_per_number** | Option<**i32**> | The maximum number of times a phone number can be called within the resetPeriod. Required if maxAttemptsPerContact is not defined. | [optional]
**time_zone_id** | Option<**String**> | If the resetPeriod is TODAY, this specifies the timezone in which TODAY occurs. Required if the resetPeriod is TODAY. | [optional]
**reset_period** | Option<**String**> | After how long the number of attempts will be set back to 0. Defaults to NEVER. | [optional]
**recall_entries** | Option<[**::std::collections::HashMap<String, crate::models::RecallEntry>**](RecallEntry.md)> | Configuration for recall attempts. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


