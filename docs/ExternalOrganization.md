# ExternalOrganization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional]
**name** | **String** | The name of the company. | 
**company_type** | Option<**String**> |  | [optional]
**industry** | Option<**String**> |  | [optional]
**primary_contact_id** | Option<**String**> |  | [optional]
**address** | Option<[**crate::models::ContactAddress**](ContactAddress.md)> |  | [optional]
**phone_number** | Option<[**crate::models::PhoneNumber**](PhoneNumber.md)> |  | [optional]
**fax_number** | Option<[**crate::models::PhoneNumber**](PhoneNumber.md)> |  | [optional]
**employee_count** | Option<**i64**> |  | [optional]
**revenue** | Option<**i64**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**websites** | Option<**Vec<String>**> |  | [optional]
**tickers** | Option<[**Vec<crate::models::Ticker>**](Ticker.md)> |  | [optional]
**twitter_id** | Option<[**crate::models::TwitterId**](TwitterId.md)> |  | [optional]
**external_system_url** | Option<**String**> | A string that identifies an external system-of-record resource that may have more detailed information on the organization. It should be a valid URL (including the http/https protocol, port, and path [if any]). The value is automatically trimmed of any leading and trailing whitespace. | [optional]
**modify_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**create_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**trustor** | Option<[**crate::models::Trustor**](Trustor.md)> |  | [optional]
**schema** | Option<[**crate::models::DataSchema**](DataSchema.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Custom fields defined in the schema referenced by schemaId and schemaVersion. | [optional]
**external_data_sources** | Option<[**Vec<crate::models::ExternalDataSource>**](ExternalDataSource.md)> | Links to the sources of data (e.g. one source might be a CRM) that contributed data to this record.  Read-only, and only populated when requested via expand param. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


