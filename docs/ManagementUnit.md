# ManagementUnit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**business_unit** | Option<[**crate::models::BusinessUnitReference**](BusinessUnitReference.md)> |  | [optional]
**start_day_of_week** | Option<**String**> | Start day of week for scheduling and forecasting purposes. Moving to Business Unit | [optional]
**time_zone** | Option<**String**> | The time zone for the management unit in standard Olson format.  Moving to Business Unit | [optional]
**settings** | Option<[**crate::models::ManagementUnitSettingsResponse**](ManagementUnitSettingsResponse.md)> |  | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]
**division** | Option<[**crate::models::DivisionReference**](DivisionReference.md)> |  | [optional]
**version** | Option<**i32**> | The version of the underlying entity.  Deprecated, use field from settings.metadata instead | [optional][readonly]
**date_modified** | Option<**String**> | The date and time at which this entity was last modified.  Deprecated, use field from settings.metadata instead. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**modified_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


