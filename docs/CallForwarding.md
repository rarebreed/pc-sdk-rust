# CallForwarding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**enabled** | Option<**bool**> | Whether or not CallForwarding is enabled | [optional]
**phone_number** | Option<**String**> | This property is deprecated. Please use the calls property | [optional]
**calls** | Option<[**Vec<crate::models::CallRoute>**](CallRoute.md)> | An ordered list of CallRoutes to be executed when CallForwarding is enabled | [optional]
**voicemail** | Option<**String**> | The type of voicemail to use with the callForwarding configuration | [optional]
**modified_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


