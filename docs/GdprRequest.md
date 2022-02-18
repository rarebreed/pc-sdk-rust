# GdprRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**created_by** | [**crate::models::DomainEntityRef**](DomainEntityRef.md) |  | 
**replacement_terms** | Option<[**Vec<crate::models::ReplacementTerm>**](ReplacementTerm.md)> | The replacement terms for the provided search terms, in the case of a GDPR_UPDATE request | [optional]
**request_type** | **String** | The type of GDPR request | 
**created_date** | **String** | When the request was submitted. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [readonly]
**status** | **String** | The status of the request | [readonly]
**subject** | [**crate::models::GdprSubject**](GDPRSubject.md) |  | 
**results_url** | Option<**String**> | The location where the results of the request can be retrieved | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


