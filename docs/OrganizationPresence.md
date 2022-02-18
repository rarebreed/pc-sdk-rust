# OrganizationPresence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**language_labels** | **::std::collections::HashMap<String, String>** | The label used for the system presence in each specified language | 
**system_presence** | Option<**String**> |  | [optional]
**deactivated** | Option<**bool**> |  | [optional]
**primary** | Option<**bool**> |  | [optional]
**created_by** | Option<[**crate::models::User**](User.md)> |  | [optional]
**created_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**modified_by** | Option<[**crate::models::User**](User.md)> |  | [optional]
**modified_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


