# CreateCallbackCommand

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**script_id** | Option<**String**> | The identifier of the script to be used for the callback | [optional]
**queue_id** | Option<**String**> | The identifier of the queue to be used for the callback. Either queueId or routingData is required. | [optional]
**routing_data** | Option<[**crate::models::RoutingData**](RoutingData.md)> |  | [optional]
**callback_user_name** | Option<**String**> | The name of the party to be called back. | [optional]
**callback_numbers** | **Vec<String>** | A list of phone numbers for the callback. | 
**callback_scheduled_time** | Option<**String**> | The scheduled date-time for the callback as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss.SSSZ | [optional]
**country_code** | Option<**String**> | The country code to be associated with the callback numbers. | [optional]
**validate_callback_numbers** | Option<**bool**> | Whether or not to validate the callback numbers for phone number format. | [optional]
**data** | Option<**::std::collections::HashMap<String, String>**> | A map of key-value pairs containing additional data that can be associated to the callback. These values will appear in the attributes property on the conversation participant. Example: { \"notes\": \"ready to close the deal!\", \"customerPreferredName\": \"Doc\" } | [optional]
**caller_id** | Option<**String**> | The phone number displayed to recipients when a phone call is placed as part of the callback. Must conform to the E.164 format. May be overridden by other settings in the system such as external trunk settings. Telco support for \"callerId\" varies. | [optional]
**caller_id_name** | Option<**String**> | The name displayed to recipients when a phone call is placed as part of the callback. May be overridden by other settings in the system such as external trunk settings. Telco support for \"callerIdName\" varies. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


