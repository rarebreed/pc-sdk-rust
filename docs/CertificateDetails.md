# CertificateDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issuer** | Option<**String**> | Information about the issuer of the certificate.  The value of this property is a comma separated key=value format.  Each key is one of the attribute names supported by X.500. | [optional]
**subject** | Option<**String**> | Information about the subject of the certificate.  The value of this property is a comma separated key=value format.  Each key is one of the attribute names supported by X.500. | [optional]
**expiration_date** | Option<**String**> | The expiration date of the certificate. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**issue_date** | Option<**String**> | The issue date of the certificate. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**expired** | Option<**bool**> | True if the certificate is expired, false otherwise. | [optional]
**signature_valid** | Option<**bool**> |  | [optional]
**valid** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


