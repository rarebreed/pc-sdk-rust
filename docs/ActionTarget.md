# ActionTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**user_data** | Option<[**Vec<crate::models::KeyValue>**](KeyValue.md)> | Additional user data associated with the target in key/value format. | [optional]
**supported_media_types** | Option<**Vec<String>**> | Supported media types of the target. | [optional]
**state** | Option<**String**> | Indicates the state of the target. | [optional]
**description** | Option<**String**> | Description of the target. | [optional]
**service_level** | Option<[**crate::models::ServiceLevel**](ServiceLevel.md)> |  | [optional]
**short_abandon_threshold** | Option<**i32**> | Indicates the non-default short abandon threshold | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**created_date** | Option<**String**> | The date the target was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**modified_date** | Option<**String**> | The date the target was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


