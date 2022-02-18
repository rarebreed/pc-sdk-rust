# RuleSet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the RuleSet. | 
**date_created** | Option<**String**> | Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**version** | Option<**i32**> | Required for updates, must match the version number of the most recent update | [optional]
**contact_list** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**queue** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**rules** | [**Vec<crate::models::DialerRule>**](DialerRule.md) | The list of rules. | 
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


