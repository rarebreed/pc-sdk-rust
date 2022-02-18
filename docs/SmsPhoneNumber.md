# SmsPhoneNumber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**phone_number** | **String** | A phone number provisioned for SMS communications in E.164 format. E.g. +13175555555 or +34234234234 | 
**phone_number_type** | Option<**String**> | Type of the phone number provisioned. | [optional][readonly]
**provisioned_through_pure_cloud** | Option<**bool**> | Is set to false, if the phone number is provisioned through a SMS provider, outside of PureCloud | [optional]
**phone_number_status** | Option<**String**> | Status of the provisioned phone number. | [optional]
**capabilities** | Option<**Vec<String>**> | The capabilities of the phone number available for provisioning. | [optional][readonly]
**country_code** | Option<**String**> | The ISO 3166-1 alpha-2 country code of the country this phone number is associated with. | [optional]
**date_created** | Option<**String**> | Date this phone number was provisioned. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | Date this phone number was modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**created_by** | Option<[**crate::models::User**](User.md)> |  | [optional]
**modified_by** | Option<[**crate::models::User**](User.md)> |  | [optional]
**version** | **i32** | Version number required for updates. | 
**purchase_date** | Option<**String**> | Date this phone number was purchased, if the phoneNumberType is shortcode. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**cancellation_date** | Option<**String**> | Contract end date of this phone number, if the phoneNumberType is shortcode. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**renewal_date** | Option<**String**> | Contract renewal date of this phone number, if the phoneNumberType is shortcode. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**auto_renewable** | Option<**String**> | Renewal time period of this phone number, if the phoneNumberType is shortcode. | [optional]
**address_id** | Option<[**crate::models::SmsAddress**](SmsAddress.md)> |  | [optional]
**short_code_billing_type** | Option<**String**> | BillingType of this phone number, if the phoneNumberType is shortcode. | [optional]
**provisioning_status** | Option<[**crate::models::SmsProvisioningStatus**](SmsProvisioningStatus.md)> |  | [optional]
**country** | Option<**String**> | Localized country name for the country code this phone number belongs too | [optional]
**supports_sms** | Option<**bool**> | Set to true if this phone number has the capability to support SMS | [optional]
**supports_mms** | Option<**bool**> | Set to true if this phone number has the capability to support MMS | [optional]
**supports_voice** | Option<**bool**> | Set to true if this phone number has the capability to support voice | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


