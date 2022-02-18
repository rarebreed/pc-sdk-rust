# Trunk

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the entity. | 
**division** | Option<[**crate::models::Division**](Division.md)> |  | [optional]
**description** | Option<**String**> | The resource's description. | [optional]
**version** | Option<**i32**> | The current version of the resource. | [optional]
**date_created** | Option<**String**> | The date the resource was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | The date of the last modification to the resource. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**modified_by** | Option<**String**> | The ID of the user that last modified the resource. | [optional]
**created_by** | Option<**String**> | The ID of the user that created the resource. | [optional]
**state** | Option<**String**> | Indicates if the resource is active, inactive, or deleted. | [optional][readonly]
**modified_by_app** | Option<**String**> | The application that last modified the resource. | [optional]
**created_by_app** | Option<**String**> | The application that created the resource. | [optional]
**trunk_type** | Option<**String**> | The type of this trunk. | [optional]
**edge** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**trunk_base** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**trunk_metabase** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**edge_group** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**in_service** | Option<**bool**> | True if this trunk is in-service.  This comes from the trunk_enabled property of the referenced trunk base. | [optional][readonly]
**enabled** | Option<**bool**> | True if the Edge used by this trunk is in-service | [optional]
**logical_interface** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**connected_status** | Option<[**crate::models::TrunkConnectedStatus**](TrunkConnectedStatus.md)> |  | [optional]
**options_status** | Option<[**Vec<crate::models::TrunkMetricsOptions>**](TrunkMetricsOptions.md)> | The trunk optionsStatus | [optional][readonly]
**registers_status** | Option<[**Vec<crate::models::TrunkMetricsRegisters>**](TrunkMetricsRegisters.md)> | The trunk registersStatus | [optional][readonly]
**ip_status** | Option<[**crate::models::TrunkMetricsNetworkTypeIp**](TrunkMetricsNetworkTypeIp.md)> |  | [optional]
**options_enabled_status** | Option<**String**> | Returns Enabled when the trunk base supports the availability interval and it has a value greater than 0. | [optional][readonly]
**registers_enabled_status** | Option<**String**> | Returns Enabled when the trunk base supports the registration interval and it has a value greater than 0. | [optional][readonly]
**family** | Option<**i32**> | The IP Network Family of the trunk | [optional][readonly]
**proxy_address_list** | Option<**Vec<String>**> | The list of proxy addresses (ports if provided) for the trunk | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


