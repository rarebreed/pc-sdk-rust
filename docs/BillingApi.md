# \BillingApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_billing_reports_billableusage**](BillingApi.md#get_billing_reports_billableusage) | **GET** /api/v2/billing/reports/billableusage | Get a report of the billable license usages
[**get_billing_trusteebillingoverview_trustor_org_id**](BillingApi.md#get_billing_trusteebillingoverview_trustor_org_id) | **GET** /api/v2/billing/trusteebillingoverview/{trustorOrgId} | Get the billing overview for an organization that is managed by a partner.



## get_billing_reports_billableusage

> crate::models::BillingUsageReport get_billing_reports_billableusage(start_date, end_date)
Get a report of the billable license usages

Report is of the billable usages (e.g. licenses and devices utilized) for a given period. If response's status is InProgress, wait a few seconds, then try the same request again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** | The period start date. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [required] |
**end_date** | **String** | The period end date. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [required] |

### Return type

[**crate::models::BillingUsageReport**](BillingUsageReport.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_billing_trusteebillingoverview_trustor_org_id

> crate::models::TrusteeBillingOverview get_billing_trusteebillingoverview_trustor_org_id(trustor_org_id, billing_period_index)
Get the billing overview for an organization that is managed by a partner.

Tax Disclaimer: Prices returned by this API do not include applicable taxes. It is the responsibility of the customer to pay all taxes that are appropriate in their jurisdiction. See the PureCloud API Documentation in the Developer Center for more information about this API: https://developer.mypurecloud.com/api/rest/v2/

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | The organization ID of the trustor (customer) organization. | [required] |
**billing_period_index** | Option<**i32**> | 0 for active period (overview data may change until period closes). 1 for prior completed billing period. 2 for two billing cycles prior, and so on. |  |[default to 0]

### Return type

[**crate::models::TrusteeBillingOverview**](TrusteeBillingOverview.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

