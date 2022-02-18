# CreateManagementUnitApiRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the management unit | 
**time_zone** | Option<**String**> | The default time zone to use for this management unit.  Moving to Business Unit | [optional]
**start_day_of_week** | Option<**String**> | The configured first day of the week for scheduling and forecasting purposes. Moving to Business Unit | [optional]
**settings** | Option<[**crate::models::CreateManagementUnitSettingsRequest**](CreateManagementUnitSettingsRequest.md)> |  | [optional]
**division_id** | Option<**String**> | The id of the division to which this management unit belongs.  Defaults to home division ID | [optional]
**business_unit_id** | **String** | The id of the business unit to which this management unit belongs | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


