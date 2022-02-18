# ExternalContact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional]
**first_name** | **String** | The first name of the contact. | 
**middle_name** | Option<**String**> |  | [optional]
**last_name** | **String** | The last name of the contact. | 
**salutation** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**work_phone** | Option<[**crate::models::PhoneNumber**](PhoneNumber.md)> |  | [optional]
**cell_phone** | Option<[**crate::models::PhoneNumber**](PhoneNumber.md)> |  | [optional]
**home_phone** | Option<[**crate::models::PhoneNumber**](PhoneNumber.md)> |  | [optional]
**other_phone** | Option<[**crate::models::PhoneNumber**](PhoneNumber.md)> |  | [optional]
**work_email** | Option<**String**> |  | [optional]
**personal_email** | Option<**String**> |  | [optional]
**other_email** | Option<**String**> |  | [optional]
**address** | Option<[**crate::models::ContactAddress**](ContactAddress.md)> |  | [optional]
**twitter_id** | Option<[**crate::models::TwitterId**](TwitterId.md)> |  | [optional]
**line_id** | Option<[**crate::models::LineId**](LineId.md)> |  | [optional]
**whats_app_id** | Option<[**crate::models::WhatsAppId**](WhatsAppId.md)> |  | [optional]
**facebook_id** | Option<[**crate::models::FacebookId**](FacebookId.md)> |  | [optional]
**modify_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**create_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**external_organization** | Option<[**crate::models::ExternalOrganization**](ExternalOrganization.md)> |  | [optional]
**survey_opt_out** | Option<**bool**> |  | [optional]
**external_system_url** | Option<**String**> | A string that identifies an external system-of-record resource that may have more detailed information on the contact. It should be a valid URL (including the http/https protocol, port, and path [if any]). The value is automatically trimmed of any leading and trailing whitespace. | [optional]
**schema** | Option<[**crate::models::DataSchema**](DataSchema.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Custom fields defined in the schema referenced by schemaId and schemaVersion. | [optional]
**external_data_sources** | Option<[**Vec<crate::models::ExternalDataSource>**](ExternalDataSource.md)> | Links to the sources of data (e.g. one source might be a CRM) that contributed data to this record.  Read-only, and only populated when requested via expand param. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


