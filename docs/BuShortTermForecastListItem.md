# BuShortTermForecastListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**week_date** | Option<[**String**](string.md)> | The start week date of this forecast in yyyy-MM-dd.  Must fall on the start day of week for the associated business unit. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional]
**week_count** | Option<**i32**> | The number of weeks this forecast covers | [optional]
**creation_method** | Option<**String**> | The method by which this forecast was created | [optional]
**description** | Option<**String**> | The description of this forecast | [optional]
**legacy** | Option<**bool**> | Whether this forecast contains modifications on legacy metrics | [optional][readonly]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]
**can_use_for_scheduling** | Option<**bool**> | Whether this forecast can be used for scheduling | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


