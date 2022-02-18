# UserStation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A globally unique identifier for this station | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**associated_user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**associated_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**default_user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**provider_info** | Option<**::std::collections::HashMap<String, String>**> | Provider-specific info for this station, e.g. { \"edgeGroupId\": \"ffe7b15c-a9cc-4f4c-88f5-781327819a49\" } | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


