# Note

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**entity_id** | Option<**String**> | The id of the contact or organization to which this note refers. This only needs to be set for input when using the Bulk APIs. | [optional]
**entity_type** | Option<**String**> | This is only need to be set when using Bulk API. Using any other value than contact or organization will result in null being used. | [optional]
**note_text** | Option<**String**> |  | [optional]
**modify_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**create_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**created_by** | [**crate::models::User**](User.md) |  | 
**external_data_sources** | Option<[**Vec<crate::models::ExternalDataSource>**](ExternalDataSource.md)> | Links to the sources of data (e.g. one source might be a CRM) that contributed data to this record.  Read-only, and only populated when requested via expand param. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


