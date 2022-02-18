# DialerContact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**contact_list_id** | **String** | The identifier of the contact list containing this contact. | 
**data** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | An ordered map of the contact's columns and corresponding values. | 
**call_records** | Option<[**::std::collections::HashMap<String, crate::models::CallRecord>**](CallRecord.md)> | A map of call records for the contact phone columns. | [optional][readonly]
**latest_sms_evaluations** | Option<[**::std::collections::HashMap<String, crate::models::MessageEvaluation>**](MessageEvaluation.md)> | A map of SMS records for the contact phone columns. | [optional][readonly]
**callable** | Option<**bool**> | Indicates whether or not the contact can be called. | [optional]
**phone_number_status** | Option<[**::std::collections::HashMap<String, crate::models::PhoneNumberStatus>**](PhoneNumberStatus.md)> | A map of phone number columns to PhoneNumberStatuses, which indicate if the phone number is callable or not. | [optional]
**contact_column_time_zones** | Option<[**::std::collections::HashMap<String, crate::models::ContactColumnTimeZone>**](ContactColumnTimeZone.md)> | Map containing data about the timezone the contact is mapped to. This will only be populated if the contact list has automatic timezone mapping turned on. The key is the column name. The value is the timezone it mapped to and the type of column: Phone or Zip | [optional][readonly]
**configuration_overrides** | Option<[**crate::models::ConfigurationOverrides**](ConfigurationOverrides.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


