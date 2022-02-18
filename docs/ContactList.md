# ContactList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**date_created** | Option<**String**> | Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**version** | Option<**i32**> | Required for updates, must match the version number of the most recent update | [optional]
**division** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**column_names** | **Vec<String>** | The names of the contact data columns. | 
**phone_columns** | Option<[**Vec<crate::models::ContactPhoneNumberColumn>**](ContactPhoneNumberColumn.md)> | Indicates which columns are phone numbers. | [optional]
**import_status** | Option<[**crate::models::ImportStatus**](ImportStatus.md)> |  | [optional]
**preview_mode_column_name** | Option<**String**> | A column to check if a contact should always be dialed in preview mode. | [optional]
**preview_mode_accepted_values** | Option<**Vec<String>**> | The values in the previewModeColumnName column that indicate a contact should always be dialed in preview mode. | [optional]
**size** | Option<**i64**> | The number of contacts in the ContactList. | [optional][readonly]
**attempt_limits** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**automatic_time_zone_mapping** | Option<**bool**> | Indicates if automatic time zone mapping is to be used for this ContactList. | [optional]
**zip_code_column_name** | Option<**String**> | The name of contact list column containing the zip code for use with automatic time zone mapping. Only allowed if 'automaticTimeZoneMapping' is set to true. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


