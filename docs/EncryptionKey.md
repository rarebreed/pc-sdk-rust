# EncryptionKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**create_date** | Option<**String**> | create date of the key pair. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**keydata_summary** | Option<**String**> | key data summary (base 64 encoded public key) | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**local_encryption_configuration** | Option<[**crate::models::LocalEncryptionConfiguration**](LocalEncryptionConfiguration.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


