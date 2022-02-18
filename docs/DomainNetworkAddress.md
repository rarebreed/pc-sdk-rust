# DomainNetworkAddress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<**String**> | The type of address. | [optional]
**address** | Option<**String**> | An IPv4 or IPv6 IP address. When specifying an address of type \"ip\", use CIDR format for the subnet mask. | [optional]
**persistent** | Option<**bool**> | True if this address will persist on Edge restart.  Addresses assigned by DHCP will be returned as false. | [optional]
**family** | Option<**i32**> | The address family for this address. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


