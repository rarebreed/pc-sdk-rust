# InboundDomain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique Id of the domain such as: example.com | [optional]
**name** | Option<**String**> |  | [optional]
**mx_record_status** | **String** | Mx Record Status | 
**sub_domain** | Option<**bool**> | Indicates if this a PureCloud sub-domain.  If true, then the appropriate DNS records are created for sending/receiving email. | [optional]
**mail_from_settings** | Option<[**crate::models::MailFromResult**](MailFromResult.md)> |  | [optional]
**custom_smtp_server** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


