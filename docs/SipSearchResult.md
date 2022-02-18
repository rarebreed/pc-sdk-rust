# SipSearchResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**status** | Option<**i32**> | Status of the search request | [optional]
**sid** | Option<**String**> | Session id associated to the search request | [optional]
**auth** | Option<**String**> | Auth token used for this search request | [optional]
**message** | Option<**String**> | Any messages returned from homer as part of the response | [optional]
**data** | Option<[**Vec<crate::models::HomerRecord>**](HomerRecord.md)> | Homer search data that is returned | [optional]
**count** | Option<**i32**> | Number of records returned | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


