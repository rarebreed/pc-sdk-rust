# Station

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**user_id** | Option<**String**> | The Id of the user currently logged in and associated with the station. | [optional]
**web_rtc_user_id** | Option<**String**> | The Id of the user configured for the station if it is of type inin_webrtc_softphone. Empty if station type is not inin_webrtc_softphone. | [optional]
**primary_edge** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**secondary_edge** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**line_appearance_id** | Option<**String**> |  | [optional]
**web_rtc_media_dscp** | Option<**i32**> | The default or configured value of media dscp for the station. Empty if station type is not inin_webrtc_softphone. | [optional][readonly]
**web_rtc_persistent_enabled** | Option<**bool**> | The default or configured value of persistent connection setting for the station. Empty if station type is not inin_webrtc_softphone. | [optional][readonly]
**web_rtc_force_turn** | Option<**bool**> | Whether the station is configured to require TURN for routing WebRTC calls. Empty if station type is not inin_webrtc_softphone. | [optional][readonly]
**web_rtc_call_appearances** | Option<**i32**> | The number of call appearances on the station. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


