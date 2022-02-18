# TrusteeBillingOverview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**organization** | [**crate::models::NamedEntity**](NamedEntity.md) |  | 
**currency** | **String** | The currency type. | 
**enabled_products** | **Vec<String>** | The charge short names for products enabled during the specified period. | 
**subscription_type** | **String** | The subscription type. | 
**ramp_period_start_date** | Option<**String**> | Date-time the ramp period starts. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**ramp_period_end_date** | Option<**String**> | Date-time the ramp period ends. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**billing_period_start_date** | Option<**String**> | Date-time the billing period started. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**billing_period_end_date** | Option<**String**> | Date-time the billing period ended. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**usages** | [**Vec<crate::models::SubscriptionOverviewUsage>**](SubscriptionOverviewUsage.md) | Usages for the specified period. | 
**contract_amendment_date** | Option<**String**> | Date-time the contract was last amended. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**contract_effective_date** | Option<**String**> | Date-time the contract became effective. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**contract_end_date** | Option<**String**> | Date-time the contract ends. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**minimum_monthly_amount** | Option<**String**> | Minimum amount that will be charged for the month | [optional]
**in_ramp_period** | Option<**bool**> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


