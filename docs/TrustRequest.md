# TrustRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**created_by** | Option<[**crate::models::OrgUser**](OrgUser.md)> |  | [optional]
**date_created** | Option<**String**> | Date request was created. There is a 48 hour expiration on all requests. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**trustee** | [**crate::models::Organization**](Organization.md) |  | 
**users** | Option<[**Vec<crate::models::OrgUser>**](OrgUser.md)> | The list of trustee users that are requesting access. | [optional][readonly]
**groups** | Option<[**Vec<crate::models::TrustGroup>**](TrustGroup.md)> | The list of trustee groups that are requesting access. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


