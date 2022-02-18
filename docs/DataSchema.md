# DataSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the schema.  Only required if a schema is used for custom fields during external entity creation or updates. | [optional]
**name** | Option<**String**> |  | [optional]
**version** | **i32** | The schema's version, a positive integer. Required for updates. | 
**applies_to** | Option<**Vec<String>**> | One of \"CONTACT\" or \"EXTERNAL_ORGANIZATION\".  Indicates the built-in entity type to which this schema applies. | [optional][readonly]
**enabled** | Option<**bool**> | The schema's enabled/disabled status. A disabled schema cannot be assigned to any other entities, but the data on those entities from the schema still exists. | [optional]
**created_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**date_created** | Option<**String**> | The date and time this schema was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**json_schema** | [**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md) |  | 
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


