# DncList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the DncList. | 
**date_created** | Option<**String**> | Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**version** | Option<**i32**> | Required for updates, must match the version number of the most recent update | [optional]
**import_status** | Option<[**crate::models::ImportStatus**](ImportStatus.md)> |  | [optional]
**size** | Option<**i64**> | The total number of phone numbers in the DncList. | [optional][readonly]
**dnc_source_type** | **String** | The type of the DncList. | [readonly]
**login_id** | Option<**String**> | A dnc.com loginId. Required if the dncSourceType is dnc.com. | [optional]
**dnc_codes** | Option<**Vec<String>**> | The list of dnc.com codes to be treated as DNC. Required if the dncSourceType is dnc.com. | [optional]
**license_id** | Option<**String**> | A gryphon license number. Required if the dncSourceType is gryphon. | [optional]
**division** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


