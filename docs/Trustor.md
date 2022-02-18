# Trustor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Organization Id for this trust. | [optional][readonly]
**enabled** | **bool** | If disabled no trustee user will have access, even if they were previously added. | 
**date_created** | Option<**String**> | Date Trust was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**created_by** | Option<[**crate::models::OrgUser**](OrgUser.md)> |  | [optional]
**organization** | Option<[**crate::models::Organization**](Organization.md)> |  | [optional]
**authorization** | Option<[**crate::models::TrusteeAuthorization**](TrusteeAuthorization.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


