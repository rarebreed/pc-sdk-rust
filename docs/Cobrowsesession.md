# Cobrowsesession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | The connection state of this communication. | [optional]
**id** | Option<**String**> | A globally unique identifier for this communication. | [optional]
**disconnect_type** | Option<**String**> | System defined string indicating what caused the communication to disconnect. Will be null until the communication disconnects. | [optional]
**_self** | Option<[**crate::models::Address**](Address.md)> |  | [optional]
**cobrowse_session_id** | Option<**String**> | The co-browse session ID. | [optional]
**cobrowse_role** | Option<**String**> | This value identifies the role of the co-browse client within the co-browse session (a client is a sharer or a viewer). | [optional]
**controlling** | Option<**Vec<String>**> | ID of co-browse participants for which this client has been granted control (list is empty if this client cannot control any shared pages). | [optional]
**viewer_url** | Option<**String**> | The URL that can be used to open co-browse session in web browser. | [optional]
**provider_event_time** | Option<**String**> | The time when the provider event which triggered this conversation update happened in the corrected provider clock (milliseconds since 1970-01-01 00:00:00 UTC). Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**start_alerting_time** | Option<**String**> | The timestamp the communication has when it is first put into an alerting state. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**connected_time** | Option<**String**> | The timestamp when this communication was connected in the cloud clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**disconnected_time** | Option<**String**> | The timestamp when this communication disconnected from the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**provider** | Option<**String**> | The source provider for the co-browse session. | [optional]
**peer_id** | Option<**String**> | The id of the peer communication corresponding to a matching leg for this communication. | [optional]
**segments** | Option<[**Vec<crate::models::Segment>**](Segment.md)> | The time line of the participant's call, divided into activity segments. | [optional]
**wrapup** | Option<[**crate::models::Wrapup**](Wrapup.md)> |  | [optional]
**after_call_work** | Option<[**crate::models::AfterCallWork**](AfterCallWork.md)> |  | [optional]
**after_call_work_required** | Option<**bool**> | Indicates if after-call work is required for a communication. Only used when the ACW Setting is Agent Requested. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


