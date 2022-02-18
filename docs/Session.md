# Session

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**customer_id** | Option<**String**> | Primary identifier of the customer in the source where the events for the session originate from. | [optional]
**customer_id_type** | Option<**String**> | Type of source customer identifier (e.g. cookie, email, phone). | [optional]
**_type** | Option<**String**> | Session types indicate the type or category of sessions (e.g. web, ticket, delivery, atm). | [optional]
**external_id** | Option<**String**> | Unique identifier in the external system where the events for the session originate from. | [optional]
**external_url** | Option<**String**> | A URL that identifies an external system-of-record resource that may have more detailed information on the session. | [optional]
**short_id** | Option<**String**> | Shortened numeric identifier of 4-6 digits. | [optional]
**outcome_achievements** | Option<[**Vec<crate::models::OutcomeAchievement>**](OutcomeAchievement.md)> | List of the outcome achievements by the customer in this session. | [optional]
**segment_assignments** | Option<[**Vec<crate::models::SessionSegmentAssignment>**](SessionSegmentAssignment.md)> | List of the segment assignments to the customer in this session. | [optional]
**attributes** | Option<[**::std::collections::HashMap<String, crate::models::CustomEventAttribute>**](CustomEventAttribute.md)> | Attributes projected from the session's event stream. | [optional]
**attribute_lists** | Option<[**::std::collections::HashMap<String, crate::models::CustomEventAttributeList>**](CustomEventAttributeList.md)> | List-type attributes projected from the session's event stream. | [optional]
**browser** | Option<[**crate::models::Browser**](Browser.md)> |  | [optional]
**device** | Option<[**crate::models::Device**](Device.md)> |  | [optional]
**geolocation** | Option<[**crate::models::JourneyGeolocation**](JourneyGeolocation.md)> |  | [optional]
**ip_address** | Option<**String**> | Customer's IP address. | [optional]
**ip_organization** | Option<**String**> | Customer's IP-based organization or ISP name. | [optional]
**last_page** | Option<[**crate::models::JourneyPage**](JourneyPage.md)> |  | [optional]
**mkt_campaign** | Option<[**crate::models::JourneyCampaign**](JourneyCampaign.md)> |  | [optional]
**referrer** | Option<[**crate::models::Referrer**](Referrer.md)> |  | [optional]
**search_terms** | Option<**Vec<String>**> | Search terms associated with the session. | [optional]
**user_agent_string** | Option<**String**> | String identifying the user agent. | [optional]
**duration_in_seconds** | Option<**i32**> | Indicates how long the session has been active (valid for an individual device). | [optional]
**event_count** | Option<**i32**> | The count of all events performed during the session. | [optional]
**pageview_count** | Option<**i32**> | The count of all pageviews performed during the session. | [optional]
**screenview_count** | Option<**i32**> | The count of all screenviews performed during the session. | [optional]
**last_event** | Option<[**crate::models::SessionLastEvent**](SessionLastEvent.md)> |  | [optional]
**last_connected_queue** | Option<[**crate::models::ConnectedQueue**](ConnectedQueue.md)> |  | [optional]
**last_connected_user** | Option<[**crate::models::ConnectedUser**](ConnectedUser.md)> |  | [optional]
**last_user_disposition** | Option<[**crate::models::ConversationUserDisposition**](ConversationUserDisposition.md)> |  | [optional]
**conversation_channels** | Option<[**Vec<crate::models::ConversationChannel>**](ConversationChannel.md)> | Represents the channels used for this conversation. | [optional]
**originating_direction** | Option<**String**> | The original direction of the conversation. | [optional]
**conversation_subject** | Option<**String**> | The subject for the conversation, for example an email subject. | [optional]
**last_user_disconnect_type** | Option<**String**> | Disconnect reason for the last user connected to the conversation. | [optional]
**last_acd_outcome** | Option<**String**> | Last ACD outcome for the conversation. | [optional]
**authenticated** | Option<**bool**> | Indicates whether or not the session is authenticated. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**created_date** | Option<**String**> | Timestamp indicating when the session was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**ended_date** | Option<**String**> | Timestamp indicating when the session was ended. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**external_contact** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**away_date** | Option<**String**> | Timestamp indicating when the visitor should be considered as away. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**idle_date** | Option<**String**> | Timestamp indicating when the visitor should be considered as idle. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**conversation** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


