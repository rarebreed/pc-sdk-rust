# DomainNetworkRoute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prefix** | Option<**String**> | The IPv4 or IPv6 route prefix in CIDR notation. | [optional]
**nexthop** | Option<**String**> | The IPv4 or IPv6 nexthop IP address. | [optional]
**persistent** | Option<**bool**> | True if this route will persist on Edge restart.  Routes assigned by DHCP will be returned as false. | [optional]
**metric** | Option<**i32**> | The metric being used for route. Lower values will have a higher priority. | [optional]
**family** | Option<**i32**> | The address family for this route. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


