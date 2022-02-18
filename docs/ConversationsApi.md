# \ConversationsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_analytics_conversations_details_job**](ConversationsApi.md#delete_analytics_conversations_details_job) | **DELETE** /api/v2/analytics/conversations/details/jobs/{jobId} | Delete/cancel an async request
[**delete_conversation_participant_code**](ConversationsApi.md#delete_conversation_participant_code) | **DELETE** /api/v2/conversations/{conversationId}/participants/{participantId}/codes/{addCommunicationCode} | Delete a code used to add a communication to this participant
[**delete_conversation_participant_flaggedreason**](ConversationsApi.md#delete_conversation_participant_flaggedreason) | **DELETE** /api/v2/conversations/{conversationId}/participants/{participantId}/flaggedreason | Remove flagged reason from conversation participant.
[**delete_conversations_call_participant_consult**](ConversationsApi.md#delete_conversations_call_participant_consult) | **DELETE** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/consult | Cancel the transfer
[**delete_conversations_email_messages_draft_attachment**](ConversationsApi.md#delete_conversations_email_messages_draft_attachment) | **DELETE** /api/v2/conversations/emails/{conversationId}/messages/draft/attachments/{attachmentId} | Delete attachment from draft
[**delete_conversations_messaging_integrations_facebook_integration_id**](ConversationsApi.md#delete_conversations_messaging_integrations_facebook_integration_id) | **DELETE** /api/v2/conversations/messaging/integrations/facebook/{integrationId} | Delete a Facebook messaging integration
[**delete_conversations_messaging_integrations_line_integration_id**](ConversationsApi.md#delete_conversations_messaging_integrations_line_integration_id) | **DELETE** /api/v2/conversations/messaging/integrations/line/{integrationId} | Delete a LINE messenger integration
[**delete_conversations_messaging_integrations_open_integration_id**](ConversationsApi.md#delete_conversations_messaging_integrations_open_integration_id) | **DELETE** /api/v2/conversations/messaging/integrations/open/{integrationId} | Delete an Open messaging integration
[**delete_conversations_messaging_integrations_twitter_integration_id**](ConversationsApi.md#delete_conversations_messaging_integrations_twitter_integration_id) | **DELETE** /api/v2/conversations/messaging/integrations/twitter/{integrationId} | Delete a Twitter messaging integration
[**delete_conversations_messaging_integrations_whatsapp_integration_id**](ConversationsApi.md#delete_conversations_messaging_integrations_whatsapp_integration_id) | **DELETE** /api/v2/conversations/messaging/integrations/whatsapp/{integrationId} | Delete a WhatsApp messaging integration
[**delete_conversations_messaging_supportedcontent_supported_content_id**](ConversationsApi.md#delete_conversations_messaging_supportedcontent_supported_content_id) | **DELETE** /api/v2/conversations/messaging/supportedcontent/{supportedContentId} | Delete a supported content profile
[**get_analytics_conversation_details**](ConversationsApi.md#get_analytics_conversation_details) | **GET** /api/v2/analytics/conversations/{conversationId}/details | Get a conversation by id
[**get_analytics_conversations_details**](ConversationsApi.md#get_analytics_conversations_details) | **GET** /api/v2/analytics/conversations/details | Gets multiple conversations by id
[**get_analytics_conversations_details_job**](ConversationsApi.md#get_analytics_conversations_details_job) | **GET** /api/v2/analytics/conversations/details/jobs/{jobId} | Get status for async query for conversation details
[**get_analytics_conversations_details_job_results**](ConversationsApi.md#get_analytics_conversations_details_job_results) | **GET** /api/v2/analytics/conversations/details/jobs/{jobId}/results | Fetch a page of results for an async query
[**get_analytics_conversations_details_jobs_availability**](ConversationsApi.md#get_analytics_conversations_details_jobs_availability) | **GET** /api/v2/analytics/conversations/details/jobs/availability | Lookup the datalake availability date and time
[**get_conversation**](ConversationsApi.md#get_conversation) | **GET** /api/v2/conversations/{conversationId} | Get conversation
[**get_conversation_participant_secureivrsession**](ConversationsApi.md#get_conversation_participant_secureivrsession) | **GET** /api/v2/conversations/{conversationId}/participants/{participantId}/secureivrsessions/{secureSessionId} | Fetch info on a secure session
[**get_conversation_participant_secureivrsessions**](ConversationsApi.md#get_conversation_participant_secureivrsessions) | **GET** /api/v2/conversations/{conversationId}/participants/{participantId}/secureivrsessions | Get a list of secure sessions for this participant.
[**get_conversation_participant_wrapup**](ConversationsApi.md#get_conversation_participant_wrapup) | **GET** /api/v2/conversations/{conversationId}/participants/{participantId}/wrapup | Get the wrap-up for this conversation participant. 
[**get_conversation_participant_wrapupcodes**](ConversationsApi.md#get_conversation_participant_wrapupcodes) | **GET** /api/v2/conversations/{conversationId}/participants/{participantId}/wrapupcodes | Get list of wrapup codes for this conversation participant
[**get_conversations**](ConversationsApi.md#get_conversations) | **GET** /api/v2/conversations | Get active conversations for the logged in user
[**get_conversations_call**](ConversationsApi.md#get_conversations_call) | **GET** /api/v2/conversations/calls/{conversationId} | Get call conversation
[**get_conversations_call_participant_wrapup**](ConversationsApi.md#get_conversations_call_participant_wrapup) | **GET** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/wrapup | Get the wrap-up for this conversation participant. 
[**get_conversations_call_participant_wrapupcodes**](ConversationsApi.md#get_conversations_call_participant_wrapupcodes) | **GET** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/wrapupcodes | Get list of wrapup codes for this conversation participant
[**get_conversations_callback**](ConversationsApi.md#get_conversations_callback) | **GET** /api/v2/conversations/callbacks/{conversationId} | Get callback conversation
[**get_conversations_callback_participant_wrapup**](ConversationsApi.md#get_conversations_callback_participant_wrapup) | **GET** /api/v2/conversations/callbacks/{conversationId}/participants/{participantId}/wrapup | Get the wrap-up for this conversation participant. 
[**get_conversations_callback_participant_wrapupcodes**](ConversationsApi.md#get_conversations_callback_participant_wrapupcodes) | **GET** /api/v2/conversations/callbacks/{conversationId}/participants/{participantId}/wrapupcodes | Get list of wrapup codes for this conversation participant
[**get_conversations_callbacks**](ConversationsApi.md#get_conversations_callbacks) | **GET** /api/v2/conversations/callbacks | Get active callback conversations for the logged in user
[**get_conversations_calls**](ConversationsApi.md#get_conversations_calls) | **GET** /api/v2/conversations/calls | Get active call conversations for the logged in user
[**get_conversations_calls_history**](ConversationsApi.md#get_conversations_calls_history) | **GET** /api/v2/conversations/calls/history | Get call history
[**get_conversations_calls_maximumconferenceparties**](ConversationsApi.md#get_conversations_calls_maximumconferenceparties) | **GET** /api/v2/conversations/calls/maximumconferenceparties | Get the maximum number of participants that this user can have on a conference
[**get_conversations_chat**](ConversationsApi.md#get_conversations_chat) | **GET** /api/v2/conversations/chats/{conversationId} | Get chat conversation
[**get_conversations_chat_message**](ConversationsApi.md#get_conversations_chat_message) | **GET** /api/v2/conversations/chats/{conversationId}/messages/{messageId} | Get a web chat conversation message
[**get_conversations_chat_messages**](ConversationsApi.md#get_conversations_chat_messages) | **GET** /api/v2/conversations/chats/{conversationId}/messages | Get the messages of a chat conversation.
[**get_conversations_chat_participant_wrapup**](ConversationsApi.md#get_conversations_chat_participant_wrapup) | **GET** /api/v2/conversations/chats/{conversationId}/participants/{participantId}/wrapup | Get the wrap-up for this conversation participant. 
[**get_conversations_chat_participant_wrapupcodes**](ConversationsApi.md#get_conversations_chat_participant_wrapupcodes) | **GET** /api/v2/conversations/chats/{conversationId}/participants/{participantId}/wrapupcodes | Get list of wrapup codes for this conversation participant
[**get_conversations_chats**](ConversationsApi.md#get_conversations_chats) | **GET** /api/v2/conversations/chats | Get active chat conversations for the logged in user
[**get_conversations_cobrowsesession**](ConversationsApi.md#get_conversations_cobrowsesession) | **GET** /api/v2/conversations/cobrowsesessions/{conversationId} | Get cobrowse conversation
[**get_conversations_cobrowsesession_participant_wrapup**](ConversationsApi.md#get_conversations_cobrowsesession_participant_wrapup) | **GET** /api/v2/conversations/cobrowsesessions/{conversationId}/participants/{participantId}/wrapup | Get the wrap-up for this conversation participant. 
[**get_conversations_cobrowsesession_participant_wrapupcodes**](ConversationsApi.md#get_conversations_cobrowsesession_participant_wrapupcodes) | **GET** /api/v2/conversations/cobrowsesessions/{conversationId}/participants/{participantId}/wrapupcodes | Get list of wrapup codes for this conversation participant
[**get_conversations_cobrowsesessions**](ConversationsApi.md#get_conversations_cobrowsesessions) | **GET** /api/v2/conversations/cobrowsesessions | Get active cobrowse conversations for the logged in user
[**get_conversations_email**](ConversationsApi.md#get_conversations_email) | **GET** /api/v2/conversations/emails/{conversationId} | Get email conversation
[**get_conversations_email_message**](ConversationsApi.md#get_conversations_email_message) | **GET** /api/v2/conversations/emails/{conversationId}/messages/{messageId} | Get conversation message
[**get_conversations_email_messages**](ConversationsApi.md#get_conversations_email_messages) | **GET** /api/v2/conversations/emails/{conversationId}/messages | Get conversation messages
[**get_conversations_email_messages_draft**](ConversationsApi.md#get_conversations_email_messages_draft) | **GET** /api/v2/conversations/emails/{conversationId}/messages/draft | Get conversation draft reply
[**get_conversations_email_participant_wrapup**](ConversationsApi.md#get_conversations_email_participant_wrapup) | **GET** /api/v2/conversations/emails/{conversationId}/participants/{participantId}/wrapup | Get the wrap-up for this conversation participant. 
[**get_conversations_email_participant_wrapupcodes**](ConversationsApi.md#get_conversations_email_participant_wrapupcodes) | **GET** /api/v2/conversations/emails/{conversationId}/participants/{participantId}/wrapupcodes | Get list of wrapup codes for this conversation participant
[**get_conversations_emails**](ConversationsApi.md#get_conversations_emails) | **GET** /api/v2/conversations/emails | Get active email conversations for the logged in user
[**get_conversations_message**](ConversationsApi.md#get_conversations_message) | **GET** /api/v2/conversations/messages/{conversationId} | Get message conversation
[**get_conversations_message_communication_messages_media_media_id**](ConversationsApi.md#get_conversations_message_communication_messages_media_media_id) | **GET** /api/v2/conversations/messages/{conversationId}/communications/{communicationId}/messages/media/{mediaId} | Get media
[**get_conversations_message_details**](ConversationsApi.md#get_conversations_message_details) | **GET** /api/v2/conversations/messages/{messageId}/details | Get message
[**get_conversations_message_message**](ConversationsApi.md#get_conversations_message_message) | **GET** /api/v2/conversations/messages/{conversationId}/messages/{messageId} | Get conversation message
[**get_conversations_message_participant_wrapup**](ConversationsApi.md#get_conversations_message_participant_wrapup) | **GET** /api/v2/conversations/messages/{conversationId}/participants/{participantId}/wrapup | Get the wrap-up for this conversation participant. 
[**get_conversations_message_participant_wrapupcodes**](ConversationsApi.md#get_conversations_message_participant_wrapupcodes) | **GET** /api/v2/conversations/messages/{conversationId}/participants/{participantId}/wrapupcodes | Get list of wrapup codes for this conversation participant
[**get_conversations_messages**](ConversationsApi.md#get_conversations_messages) | **GET** /api/v2/conversations/messages | Get active message conversations for the logged in user
[**get_conversations_messaging_facebook_app**](ConversationsApi.md#get_conversations_messaging_facebook_app) | **GET** /api/v2/conversations/messaging/facebook/app | Get Genesys Facebook App Id
[**get_conversations_messaging_integrations**](ConversationsApi.md#get_conversations_messaging_integrations) | **GET** /api/v2/conversations/messaging/integrations | Get a list of Integrations
[**get_conversations_messaging_integrations_facebook**](ConversationsApi.md#get_conversations_messaging_integrations_facebook) | **GET** /api/v2/conversations/messaging/integrations/facebook | Get a list of Facebook Integrations
[**get_conversations_messaging_integrations_facebook_integration_id**](ConversationsApi.md#get_conversations_messaging_integrations_facebook_integration_id) | **GET** /api/v2/conversations/messaging/integrations/facebook/{integrationId} | Get a Facebook messaging integration
[**get_conversations_messaging_integrations_line**](ConversationsApi.md#get_conversations_messaging_integrations_line) | **GET** /api/v2/conversations/messaging/integrations/line | Get a list of LINE messenger Integrations
[**get_conversations_messaging_integrations_line_integration_id**](ConversationsApi.md#get_conversations_messaging_integrations_line_integration_id) | **GET** /api/v2/conversations/messaging/integrations/line/{integrationId} | Get a LINE messenger integration
[**get_conversations_messaging_integrations_open**](ConversationsApi.md#get_conversations_messaging_integrations_open) | **GET** /api/v2/conversations/messaging/integrations/open | Get a list of Open messaging integrations
[**get_conversations_messaging_integrations_open_integration_id**](ConversationsApi.md#get_conversations_messaging_integrations_open_integration_id) | **GET** /api/v2/conversations/messaging/integrations/open/{integrationId} | Get an Open messaging integration
[**get_conversations_messaging_integrations_twitter**](ConversationsApi.md#get_conversations_messaging_integrations_twitter) | **GET** /api/v2/conversations/messaging/integrations/twitter | Get a list of Twitter Integrations
[**get_conversations_messaging_integrations_twitter_integration_id**](ConversationsApi.md#get_conversations_messaging_integrations_twitter_integration_id) | **GET** /api/v2/conversations/messaging/integrations/twitter/{integrationId} | Get a Twitter messaging integration
[**get_conversations_messaging_integrations_whatsapp**](ConversationsApi.md#get_conversations_messaging_integrations_whatsapp) | **GET** /api/v2/conversations/messaging/integrations/whatsapp | Get a list of WhatsApp Integrations
[**get_conversations_messaging_integrations_whatsapp_integration_id**](ConversationsApi.md#get_conversations_messaging_integrations_whatsapp_integration_id) | **GET** /api/v2/conversations/messaging/integrations/whatsapp/{integrationId} | Get a WhatsApp messaging integration
[**get_conversations_messaging_sticker**](ConversationsApi.md#get_conversations_messaging_sticker) | **GET** /api/v2/conversations/messaging/stickers/{messengerType} | Get a list of Messaging Stickers
[**get_conversations_messaging_supportedcontent**](ConversationsApi.md#get_conversations_messaging_supportedcontent) | **GET** /api/v2/conversations/messaging/supportedcontent | Get a list of Supported Content profiles
[**get_conversations_messaging_supportedcontent_default**](ConversationsApi.md#get_conversations_messaging_supportedcontent_default) | **GET** /api/v2/conversations/messaging/supportedcontent/default | Get the organization's default supported content profile that will be used as the default when creating an integration.
[**get_conversations_messaging_supportedcontent_supported_content_id**](ConversationsApi.md#get_conversations_messaging_supportedcontent_supported_content_id) | **GET** /api/v2/conversations/messaging/supportedcontent/{supportedContentId} | Get a supported content profile
[**get_conversations_messaging_threadingtimeline**](ConversationsApi.md#get_conversations_messaging_threadingtimeline) | **GET** /api/v2/conversations/messaging/threadingtimeline | Get conversation threading window timeline for each messaging type
[**patch_conversation_participant**](ConversationsApi.md#patch_conversation_participant) | **PATCH** /api/v2/conversations/{conversationId}/participants/{participantId} | Update a participant.
[**patch_conversation_participant_attributes**](ConversationsApi.md#patch_conversation_participant_attributes) | **PATCH** /api/v2/conversations/{conversationId}/participants/{participantId}/attributes | Update the attributes on a conversation participant.
[**patch_conversations_call**](ConversationsApi.md#patch_conversations_call) | **PATCH** /api/v2/conversations/calls/{conversationId} | Update a conversation by setting it's recording state, merging in other conversations to create a conference, or disconnecting all of the participants
[**patch_conversations_call_participant**](ConversationsApi.md#patch_conversations_call_participant) | **PATCH** /api/v2/conversations/calls/{conversationId}/participants/{participantId} | Update conversation participant
[**patch_conversations_call_participant_attributes**](ConversationsApi.md#patch_conversations_call_participant_attributes) | **PATCH** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/attributes | Update the attributes on a conversation participant.
[**patch_conversations_call_participant_communication**](ConversationsApi.md#patch_conversations_call_participant_communication) | **PATCH** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/communications/{communicationId} | Update conversation participant's communication by disconnecting it.
[**patch_conversations_call_participant_consult**](ConversationsApi.md#patch_conversations_call_participant_consult) | **PATCH** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/consult | Change who can speak
[**patch_conversations_callback**](ConversationsApi.md#patch_conversations_callback) | **PATCH** /api/v2/conversations/callbacks/{conversationId} | Update a conversation by disconnecting all of the participants
[**patch_conversations_callback_participant**](ConversationsApi.md#patch_conversations_callback_participant) | **PATCH** /api/v2/conversations/callbacks/{conversationId}/participants/{participantId} | Update conversation participant
[**patch_conversations_callback_participant_attributes**](ConversationsApi.md#patch_conversations_callback_participant_attributes) | **PATCH** /api/v2/conversations/callbacks/{conversationId}/participants/{participantId}/attributes | Update the attributes on a conversation participant.
[**patch_conversations_callback_participant_communication**](ConversationsApi.md#patch_conversations_callback_participant_communication) | **PATCH** /api/v2/conversations/callbacks/{conversationId}/participants/{participantId}/communications/{communicationId} | Update conversation participant's communication by disconnecting it.
[**patch_conversations_chat**](ConversationsApi.md#patch_conversations_chat) | **PATCH** /api/v2/conversations/chats/{conversationId} | Update a conversation by disconnecting all of the participants
[**patch_conversations_chat_participant**](ConversationsApi.md#patch_conversations_chat_participant) | **PATCH** /api/v2/conversations/chats/{conversationId}/participants/{participantId} | Update conversation participant
[**patch_conversations_chat_participant_attributes**](ConversationsApi.md#patch_conversations_chat_participant_attributes) | **PATCH** /api/v2/conversations/chats/{conversationId}/participants/{participantId}/attributes | Update the attributes on a conversation participant.
[**patch_conversations_chat_participant_communication**](ConversationsApi.md#patch_conversations_chat_participant_communication) | **PATCH** /api/v2/conversations/chats/{conversationId}/participants/{participantId}/communications/{communicationId} | Update conversation participant's communication by disconnecting it.
[**patch_conversations_cobrowsesession**](ConversationsApi.md#patch_conversations_cobrowsesession) | **PATCH** /api/v2/conversations/cobrowsesessions/{conversationId} | Update a conversation by disconnecting all of the participants
[**patch_conversations_cobrowsesession_participant**](ConversationsApi.md#patch_conversations_cobrowsesession_participant) | **PATCH** /api/v2/conversations/cobrowsesessions/{conversationId}/participants/{participantId} | Update conversation participant
[**patch_conversations_cobrowsesession_participant_attributes**](ConversationsApi.md#patch_conversations_cobrowsesession_participant_attributes) | **PATCH** /api/v2/conversations/cobrowsesessions/{conversationId}/participants/{participantId}/attributes | Update the attributes on a conversation participant.
[**patch_conversations_cobrowsesession_participant_communication**](ConversationsApi.md#patch_conversations_cobrowsesession_participant_communication) | **PATCH** /api/v2/conversations/cobrowsesessions/{conversationId}/participants/{participantId}/communications/{communicationId} | Update conversation participant's communication by disconnecting it.
[**patch_conversations_email**](ConversationsApi.md#patch_conversations_email) | **PATCH** /api/v2/conversations/emails/{conversationId} | Update a conversation by disconnecting all of the participants
[**patch_conversations_email_participant**](ConversationsApi.md#patch_conversations_email_participant) | **PATCH** /api/v2/conversations/emails/{conversationId}/participants/{participantId} | Update conversation participant
[**patch_conversations_email_participant_attributes**](ConversationsApi.md#patch_conversations_email_participant_attributes) | **PATCH** /api/v2/conversations/emails/{conversationId}/participants/{participantId}/attributes | Update the attributes on a conversation participant.
[**patch_conversations_email_participant_communication**](ConversationsApi.md#patch_conversations_email_participant_communication) | **PATCH** /api/v2/conversations/emails/{conversationId}/participants/{participantId}/communications/{communicationId} | Update conversation participant's communication by disconnecting it.
[**patch_conversations_message**](ConversationsApi.md#patch_conversations_message) | **PATCH** /api/v2/conversations/messages/{conversationId} | Update a conversation by disconnecting all of the participants
[**patch_conversations_message_participant**](ConversationsApi.md#patch_conversations_message_participant) | **PATCH** /api/v2/conversations/messages/{conversationId}/participants/{participantId} | Update conversation participant
[**patch_conversations_message_participant_attributes**](ConversationsApi.md#patch_conversations_message_participant_attributes) | **PATCH** /api/v2/conversations/messages/{conversationId}/participants/{participantId}/attributes | Update the attributes on a conversation participant.
[**patch_conversations_message_participant_communication**](ConversationsApi.md#patch_conversations_message_participant_communication) | **PATCH** /api/v2/conversations/messages/{conversationId}/participants/{participantId}/communications/{communicationId} | Update conversation participant's communication by disconnecting it.
[**patch_conversations_messaging_integrations_facebook_integration_id**](ConversationsApi.md#patch_conversations_messaging_integrations_facebook_integration_id) | **PATCH** /api/v2/conversations/messaging/integrations/facebook/{integrationId} | Update Facebook messaging integration
[**patch_conversations_messaging_integrations_open_integration_id**](ConversationsApi.md#patch_conversations_messaging_integrations_open_integration_id) | **PATCH** /api/v2/conversations/messaging/integrations/open/{integrationId} | Update an Open messaging integration
[**patch_conversations_messaging_integrations_twitter_integration_id**](ConversationsApi.md#patch_conversations_messaging_integrations_twitter_integration_id) | **PATCH** /api/v2/conversations/messaging/integrations/twitter/{integrationId} | Update Twitter messaging integration
[**patch_conversations_messaging_integrations_whatsapp_integration_id**](ConversationsApi.md#patch_conversations_messaging_integrations_whatsapp_integration_id) | **PATCH** /api/v2/conversations/messaging/integrations/whatsapp/{integrationId} | Update or activate a WhatsApp messaging integration.
[**patch_conversations_messaging_supportedcontent_supported_content_id**](ConversationsApi.md#patch_conversations_messaging_supportedcontent_supported_content_id) | **PATCH** /api/v2/conversations/messaging/supportedcontent/{supportedContentId} | Update a supported content profile
[**post_analytics_conversation_details_properties**](ConversationsApi.md#post_analytics_conversation_details_properties) | **POST** /api/v2/analytics/conversations/{conversationId}/details/properties | Index conversation properties
[**post_analytics_conversations_aggregates_query**](ConversationsApi.md#post_analytics_conversations_aggregates_query) | **POST** /api/v2/analytics/conversations/aggregates/query | Query for conversation aggregates
[**post_analytics_conversations_details_jobs**](ConversationsApi.md#post_analytics_conversations_details_jobs) | **POST** /api/v2/analytics/conversations/details/jobs | Query for conversation details asynchronously
[**post_analytics_conversations_details_query**](ConversationsApi.md#post_analytics_conversations_details_query) | **POST** /api/v2/analytics/conversations/details/query | Query for conversation details
[**post_conversation_assign**](ConversationsApi.md#post_conversation_assign) | **POST** /api/v2/conversations/{conversationId}/assign | Attempts to manually assign a specified conversation to a specified user.  Ignores bullseye ring, PAR score, skills, and languages.
[**post_conversation_disconnect**](ConversationsApi.md#post_conversation_disconnect) | **POST** /api/v2/conversations/{conversationId}/disconnect | Performs a full conversation teardown. Issues disconnect requests for any connected media. Applies a system wrap-up code to any participants that are pending wrap-up. This is not intended to be the normal way of ending interactions but is available in the event of problems with the application to allow a resynchronization of state across all components. It is recommended that users submit a support case if they are relying on this endpoint systematically as there is likely something that needs investigation.
[**post_conversation_participant_callbacks**](ConversationsApi.md#post_conversation_participant_callbacks) | **POST** /api/v2/conversations/{conversationId}/participants/{participantId}/callbacks | Create a new callback for the specified participant on the conversation.
[**post_conversation_participant_digits**](ConversationsApi.md#post_conversation_participant_digits) | **POST** /api/v2/conversations/{conversationId}/participants/{participantId}/digits | Sends DTMF to the participant
[**post_conversation_participant_replace**](ConversationsApi.md#post_conversation_participant_replace) | **POST** /api/v2/conversations/{conversationId}/participants/{participantId}/replace | Replace this participant with the specified user and/or address
[**post_conversation_participant_secureivrsessions**](ConversationsApi.md#post_conversation_participant_secureivrsessions) | **POST** /api/v2/conversations/{conversationId}/participants/{participantId}/secureivrsessions | Create secure IVR session. Only a participant in the conversation can invoke a secure IVR.
[**post_conversations_call**](ConversationsApi.md#post_conversations_call) | **POST** /api/v2/conversations/calls/{conversationId} | Place a new call as part of a callback conversation.
[**post_conversations_call_participant_coach**](ConversationsApi.md#post_conversations_call_participant_coach) | **POST** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/coach | Listen in on the conversation from the point of view of a given participant while speaking to just the given participant.
[**post_conversations_call_participant_consult**](ConversationsApi.md#post_conversations_call_participant_consult) | **POST** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/consult | Initiate and update consult transfer
[**post_conversations_call_participant_monitor**](ConversationsApi.md#post_conversations_call_participant_monitor) | **POST** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/monitor | Listen in on the conversation from the point of view of a given participant.
[**post_conversations_call_participant_replace**](ConversationsApi.md#post_conversations_call_participant_replace) | **POST** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/replace | Replace this participant with the specified user and/or address
[**post_conversations_call_participants**](ConversationsApi.md#post_conversations_call_participants) | **POST** /api/v2/conversations/calls/{conversationId}/participants | Add participants to a conversation
[**post_conversations_callback_participant_replace**](ConversationsApi.md#post_conversations_callback_participant_replace) | **POST** /api/v2/conversations/callbacks/{conversationId}/participants/{participantId}/replace | Replace this participant with the specified user and/or address
[**post_conversations_callbacks**](ConversationsApi.md#post_conversations_callbacks) | **POST** /api/v2/conversations/callbacks | Create a Callback
[**post_conversations_calls**](ConversationsApi.md#post_conversations_calls) | **POST** /api/v2/conversations/calls | Create a call conversation
[**post_conversations_chat_communication_messages**](ConversationsApi.md#post_conversations_chat_communication_messages) | **POST** /api/v2/conversations/chats/{conversationId}/communications/{communicationId}/messages | Send a message on behalf of a communication in a chat conversation.
[**post_conversations_chat_communication_typing**](ConversationsApi.md#post_conversations_chat_communication_typing) | **POST** /api/v2/conversations/chats/{conversationId}/communications/{communicationId}/typing | Send a typing-indicator on behalf of a communication in a chat conversation.
[**post_conversations_chat_participant_replace**](ConversationsApi.md#post_conversations_chat_participant_replace) | **POST** /api/v2/conversations/chats/{conversationId}/participants/{participantId}/replace | Replace this participant with the specified user and/or address
[**post_conversations_chats**](ConversationsApi.md#post_conversations_chats) | **POST** /api/v2/conversations/chats | Create a web chat conversation
[**post_conversations_cobrowsesession_participant_replace**](ConversationsApi.md#post_conversations_cobrowsesession_participant_replace) | **POST** /api/v2/conversations/cobrowsesessions/{conversationId}/participants/{participantId}/replace | Replace this participant with the specified user and/or address
[**post_conversations_email_inboundmessages**](ConversationsApi.md#post_conversations_email_inboundmessages) | **POST** /api/v2/conversations/emails/{conversationId}/inboundmessages | Send an email to an external conversation. An external conversation is one where the provider is not PureCloud based. This endpoint allows the sender of the external email to reply or send a new message to the existing conversation. The new message will be treated as part of the existing conversation and chained to it.
[**post_conversations_email_messages**](ConversationsApi.md#post_conversations_email_messages) | **POST** /api/v2/conversations/emails/{conversationId}/messages | Send an email reply
[**post_conversations_email_messages_draft_attachments_copy**](ConversationsApi.md#post_conversations_email_messages_draft_attachments_copy) | **POST** /api/v2/conversations/emails/{conversationId}/messages/draft/attachments/copy | Copy attachments from an email message to the current draft.
[**post_conversations_email_participant_replace**](ConversationsApi.md#post_conversations_email_participant_replace) | **POST** /api/v2/conversations/emails/{conversationId}/participants/{participantId}/replace | Replace this participant with the specified user and/or address
[**post_conversations_emails**](ConversationsApi.md#post_conversations_emails) | **POST** /api/v2/conversations/emails | Create an email conversation
[**post_conversations_faxes**](ConversationsApi.md#post_conversations_faxes) | **POST** /api/v2/conversations/faxes | Create Fax Conversation
[**post_conversations_message_communication_messages**](ConversationsApi.md#post_conversations_message_communication_messages) | **POST** /api/v2/conversations/messages/{conversationId}/communications/{communicationId}/messages | Send message
[**post_conversations_message_communication_messages_media**](ConversationsApi.md#post_conversations_message_communication_messages_media) | **POST** /api/v2/conversations/messages/{conversationId}/communications/{communicationId}/messages/media | Create media
[**post_conversations_message_messages_bulk**](ConversationsApi.md#post_conversations_message_messages_bulk) | **POST** /api/v2/conversations/messages/{conversationId}/messages/bulk | Get messages in batch
[**post_conversations_message_participant_replace**](ConversationsApi.md#post_conversations_message_participant_replace) | **POST** /api/v2/conversations/messages/{conversationId}/participants/{participantId}/replace | Replace this participant with the specified user and/or address
[**post_conversations_messages**](ConversationsApi.md#post_conversations_messages) | **POST** /api/v2/conversations/messages | Create an outbound messaging conversation.
[**post_conversations_messages_agentless**](ConversationsApi.md#post_conversations_messages_agentless) | **POST** /api/v2/conversations/messages/agentless | Send an agentless outbound message
[**post_conversations_messages_inbound_open**](ConversationsApi.md#post_conversations_messages_inbound_open) | **POST** /api/v2/conversations/messages/inbound/open | Send an inbound Open Message
[**post_conversations_messaging_integrations_facebook**](ConversationsApi.md#post_conversations_messaging_integrations_facebook) | **POST** /api/v2/conversations/messaging/integrations/facebook | Create a Facebook Integration
[**post_conversations_messaging_integrations_line**](ConversationsApi.md#post_conversations_messaging_integrations_line) | **POST** /api/v2/conversations/messaging/integrations/line | Create a LINE messenger Integration
[**post_conversations_messaging_integrations_open**](ConversationsApi.md#post_conversations_messaging_integrations_open) | **POST** /api/v2/conversations/messaging/integrations/open | Create an Open messaging integration
[**post_conversations_messaging_integrations_twitter**](ConversationsApi.md#post_conversations_messaging_integrations_twitter) | **POST** /api/v2/conversations/messaging/integrations/twitter | Create a Twitter Integration
[**post_conversations_messaging_integrations_whatsapp**](ConversationsApi.md#post_conversations_messaging_integrations_whatsapp) | **POST** /api/v2/conversations/messaging/integrations/whatsapp | Create a WhatsApp Integration
[**post_conversations_messaging_supportedcontent**](ConversationsApi.md#post_conversations_messaging_supportedcontent) | **POST** /api/v2/conversations/messaging/supportedcontent | Create a Supported Content profile
[**put_conversation_participant_flaggedreason**](ConversationsApi.md#put_conversation_participant_flaggedreason) | **PUT** /api/v2/conversations/{conversationId}/participants/{participantId}/flaggedreason | Set flagged reason on conversation participant to indicate bad conversation quality.
[**put_conversation_tags**](ConversationsApi.md#put_conversation_tags) | **PUT** /api/v2/conversations/{conversationId}/tags | Update the tags on a conversation.
[**put_conversations_call_participant_communication_uuidata**](ConversationsApi.md#put_conversations_call_participant_communication_uuidata) | **PUT** /api/v2/conversations/calls/{conversationId}/participants/{participantId}/communications/{communicationId}/uuidata | Set uuiData to be sent on future commands.
[**put_conversations_email_messages_draft**](ConversationsApi.md#put_conversations_email_messages_draft) | **PUT** /api/v2/conversations/emails/{conversationId}/messages/draft | Update conversation draft reply
[**put_conversations_messaging_integrations_line_integration_id**](ConversationsApi.md#put_conversations_messaging_integrations_line_integration_id) | **PUT** /api/v2/conversations/messaging/integrations/line/{integrationId} | Update a LINE messenger integration
[**put_conversations_messaging_supportedcontent_default**](ConversationsApi.md#put_conversations_messaging_supportedcontent_default) | **PUT** /api/v2/conversations/messaging/supportedcontent/default | Set the organization's default supported content profile that may be assigned to an integration when it is created.
[**put_conversations_messaging_threadingtimeline**](ConversationsApi.md#put_conversations_messaging_threadingtimeline) | **PUT** /api/v2/conversations/messaging/threadingtimeline | Update conversation threading window timeline for each messaging type



## delete_analytics_conversations_details_job

> delete_analytics_conversations_details_job(job_id)
Delete/cancel an async request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversation_participant_code

> delete_conversation_participant_code(conversation_id, participant_id, add_communication_code)
Delete a code used to add a communication to this participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |
**add_communication_code** | **String** | addCommunicationCode | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversation_participant_flaggedreason

> delete_conversation_participant_flaggedreason(conversation_id, participant_id)
Remove flagged reason from conversation participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversations_call_participant_consult

> delete_conversations_call_participant_consult(conversation_id, participant_id)
Cancel the transfer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversations_email_messages_draft_attachment

> delete_conversations_email_messages_draft_attachment(conversation_id, attachment_id)
Delete attachment from draft

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**attachment_id** | **String** | attachmentId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversations_messaging_integrations_facebook_integration_id

> delete_conversations_messaging_integrations_facebook_integration_id(integration_id)
Delete a Facebook messaging integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversations_messaging_integrations_line_integration_id

> delete_conversations_messaging_integrations_line_integration_id(integration_id)
Delete a LINE messenger integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversations_messaging_integrations_open_integration_id

> delete_conversations_messaging_integrations_open_integration_id(integration_id)
Delete an Open messaging integration

See https://developer.genesys.cloud/api/digital/openmessaging/ for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversations_messaging_integrations_twitter_integration_id

> delete_conversations_messaging_integrations_twitter_integration_id(integration_id)
Delete a Twitter messaging integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversations_messaging_integrations_whatsapp_integration_id

> crate::models::WhatsAppIntegration delete_conversations_messaging_integrations_whatsapp_integration_id(integration_id)
Delete a WhatsApp messaging integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |

### Return type

[**crate::models::WhatsAppIntegration**](WhatsAppIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversations_messaging_supportedcontent_supported_content_id

> delete_conversations_messaging_supportedcontent_supported_content_id(supported_content_id)
Delete a supported content profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supported_content_id** | **String** | Supported Content ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_conversation_details

> crate::models::AnalyticsConversationWithoutAttributes get_analytics_conversation_details(conversation_id)
Get a conversation by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::AnalyticsConversationWithoutAttributes**](AnalyticsConversationWithoutAttributes.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_conversations_details

> crate::models::AnalyticsConversationWithoutAttributesMultiGetResponse get_analytics_conversations_details(id)
Gets multiple conversations by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**Vec<String>**](String.md)> | Comma-separated conversation ids |  |

### Return type

[**crate::models::AnalyticsConversationWithoutAttributesMultiGetResponse**](AnalyticsConversationWithoutAttributesMultiGetResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_conversations_details_job

> crate::models::AsyncQueryStatus get_analytics_conversations_details_job(job_id)
Get status for async query for conversation details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

### Return type

[**crate::models::AsyncQueryStatus**](AsyncQueryStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_conversations_details_job_results

> crate::models::AnalyticsConversationAsyncQueryResponse get_analytics_conversations_details_job_results(job_id, cursor, page_size)
Fetch a page of results for an async query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page) |  |
**page_size** | Option<**i32**> | The desired maximum number of results |  |

### Return type

[**crate::models::AnalyticsConversationAsyncQueryResponse**](AnalyticsConversationAsyncQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_conversations_details_jobs_availability

> crate::models::DataAvailabilityResponse get_analytics_conversations_details_jobs_availability()
Lookup the datalake availability date and time

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DataAvailabilityResponse**](DataAvailabilityResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation

> crate::models::Conversation get_conversation(conversation_id)
Get conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |

### Return type

[**crate::models::Conversation**](Conversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_participant_secureivrsession

> crate::models::SecureSession get_conversation_participant_secureivrsession(conversation_id, participant_id, secure_session_id)
Fetch info on a secure session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |
**secure_session_id** | **String** | secure IVR session ID | [required] |

### Return type

[**crate::models::SecureSession**](SecureSession.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_participant_secureivrsessions

> crate::models::SecureSessionEntityListing get_conversation_participant_secureivrsessions(conversation_id, participant_id)
Get a list of secure sessions for this participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |

### Return type

[**crate::models::SecureSessionEntityListing**](SecureSessionEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_participant_wrapup

> crate::models::AssignedWrapupCode get_conversation_participant_wrapup(conversation_id, participant_id, provisional)
Get the wrap-up for this conversation participant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |
**provisional** | Option<**bool**> | Indicates if the wrap-up code is provisional. |  |[default to false]

### Return type

[**crate::models::AssignedWrapupCode**](AssignedWrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_participant_wrapupcodes

> Vec<crate::models::WrapupCode> get_conversation_participant_wrapupcodes(conversation_id, participant_id)
Get list of wrapup codes for this conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |

### Return type

[**Vec<crate::models::WrapupCode>**](WrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations

> crate::models::ConversationEntityListing get_conversations(communication_type)
Get active conversations for the logged in user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**communication_type** | Option<**String**> | Call or Chat communication filtering |  |

### Return type

[**crate::models::ConversationEntityListing**](ConversationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_call

> crate::models::CallConversation get_conversations_call(conversation_id)
Get call conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::CallConversation**](CallConversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_call_participant_wrapup

> crate::models::AssignedWrapupCode get_conversations_call_participant_wrapup(conversation_id, participant_id, provisional)
Get the wrap-up for this conversation participant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**provisional** | Option<**bool**> | Indicates if the wrap-up code is provisional. |  |[default to false]

### Return type

[**crate::models::AssignedWrapupCode**](AssignedWrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_call_participant_wrapupcodes

> Vec<crate::models::WrapupCode> get_conversations_call_participant_wrapupcodes(conversation_id, participant_id)
Get list of wrapup codes for this conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |

### Return type

[**Vec<crate::models::WrapupCode>**](WrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_callback

> crate::models::CallbackConversation get_conversations_callback(conversation_id)
Get callback conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::CallbackConversation**](CallbackConversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_callback_participant_wrapup

> crate::models::AssignedWrapupCode get_conversations_callback_participant_wrapup(conversation_id, participant_id, provisional)
Get the wrap-up for this conversation participant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**provisional** | Option<**bool**> | Indicates if the wrap-up code is provisional. |  |[default to false]

### Return type

[**crate::models::AssignedWrapupCode**](AssignedWrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_callback_participant_wrapupcodes

> Vec<crate::models::WrapupCode> get_conversations_callback_participant_wrapupcodes(conversation_id, participant_id)
Get list of wrapup codes for this conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |

### Return type

[**Vec<crate::models::WrapupCode>**](WrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_callbacks

> crate::models::CallbackConversationEntityListing get_conversations_callbacks()
Get active callback conversations for the logged in user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CallbackConversationEntityListing**](CallbackConversationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_calls

> crate::models::CallConversationEntityListing get_conversations_calls()
Get active call conversations for the logged in user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CallConversationEntityListing**](CallConversationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_calls_history

> crate::models::CallHistoryConversationEntityListing get_conversations_calls_history(page_size, page_number, interval, expand)
Get call history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size, maximum 50 |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**interval** | Option<**String**> | Interval string; format is ISO-8601. Separate start and end times with forward slash '/' |  |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::CallHistoryConversationEntityListing**](CallHistoryConversationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_calls_maximumconferenceparties

> crate::models::MaxParticipants get_conversations_calls_maximumconferenceparties()
Get the maximum number of participants that this user can have on a conference

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MaxParticipants**](MaxParticipants.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_chat

> crate::models::ChatConversation get_conversations_chat(conversation_id)
Get chat conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::ChatConversation**](ChatConversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_chat_message

> crate::models::WebChatMessage get_conversations_chat_message(conversation_id, message_id)
Get a web chat conversation message

The current user must be involved with the conversation to get its messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**message_id** | **String** | messageId | [required] |

### Return type

[**crate::models::WebChatMessage**](WebChatMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_chat_messages

> crate::models::WebChatMessageEntityList get_conversations_chat_messages(conversation_id, after, before, sort_order, max_results)
Get the messages of a chat conversation.

The current user must be involved with the conversation to get its messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**after** | Option<**String**> | If specified, get the messages chronologically after the id of this message |  |
**before** | Option<**String**> | If specified, get the messages chronologically before the id of this message |  |
**sort_order** | Option<**String**> | Sort order |  |[default to ascending]
**max_results** | Option<**i32**> | Limit the returned number of messages, up to a maximum of 100 |  |[default to 100]

### Return type

[**crate::models::WebChatMessageEntityList**](WebChatMessageEntityList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_chat_participant_wrapup

> crate::models::AssignedWrapupCode get_conversations_chat_participant_wrapup(conversation_id, participant_id, provisional)
Get the wrap-up for this conversation participant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**provisional** | Option<**bool**> | Indicates if the wrap-up code is provisional. |  |[default to false]

### Return type

[**crate::models::AssignedWrapupCode**](AssignedWrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_chat_participant_wrapupcodes

> Vec<crate::models::WrapupCode> get_conversations_chat_participant_wrapupcodes(conversation_id, participant_id)
Get list of wrapup codes for this conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |

### Return type

[**Vec<crate::models::WrapupCode>**](WrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_chats

> crate::models::ChatConversationEntityListing get_conversations_chats()
Get active chat conversations for the logged in user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ChatConversationEntityListing**](ChatConversationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_cobrowsesession

> crate::models::CobrowseConversation get_conversations_cobrowsesession(conversation_id)
Get cobrowse conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::CobrowseConversation**](CobrowseConversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_cobrowsesession_participant_wrapup

> crate::models::AssignedWrapupCode get_conversations_cobrowsesession_participant_wrapup(conversation_id, participant_id, provisional)
Get the wrap-up for this conversation participant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**provisional** | Option<**bool**> | Indicates if the wrap-up code is provisional. |  |[default to false]

### Return type

[**crate::models::AssignedWrapupCode**](AssignedWrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_cobrowsesession_participant_wrapupcodes

> Vec<crate::models::WrapupCode> get_conversations_cobrowsesession_participant_wrapupcodes(conversation_id, participant_id)
Get list of wrapup codes for this conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |

### Return type

[**Vec<crate::models::WrapupCode>**](WrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_cobrowsesessions

> crate::models::CobrowseConversationEntityListing get_conversations_cobrowsesessions()
Get active cobrowse conversations for the logged in user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CobrowseConversationEntityListing**](CobrowseConversationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_email

> crate::models::EmailConversation get_conversations_email(conversation_id)
Get email conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::EmailConversation**](EmailConversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_email_message

> crate::models::EmailMessage get_conversations_email_message(conversation_id, message_id)
Get conversation message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**message_id** | **String** | messageId | [required] |

### Return type

[**crate::models::EmailMessage**](EmailMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_email_messages

> crate::models::EmailMessageListing get_conversations_email_messages(conversation_id)
Get conversation messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::EmailMessageListing**](EmailMessageListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_email_messages_draft

> crate::models::EmailMessage get_conversations_email_messages_draft(conversation_id)
Get conversation draft reply

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::EmailMessage**](EmailMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_email_participant_wrapup

> crate::models::AssignedWrapupCode get_conversations_email_participant_wrapup(conversation_id, participant_id, provisional)
Get the wrap-up for this conversation participant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**provisional** | Option<**bool**> | Indicates if the wrap-up code is provisional. |  |[default to false]

### Return type

[**crate::models::AssignedWrapupCode**](AssignedWrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_email_participant_wrapupcodes

> Vec<crate::models::WrapupCode> get_conversations_email_participant_wrapupcodes(conversation_id, participant_id)
Get list of wrapup codes for this conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |

### Return type

[**Vec<crate::models::WrapupCode>**](WrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_emails

> crate::models::EmailConversationEntityListing get_conversations_emails()
Get active email conversations for the logged in user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EmailConversationEntityListing**](EmailConversationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_message

> crate::models::MessageConversation get_conversations_message(conversation_id)
Get message conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::MessageConversation**](MessageConversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_message_communication_messages_media_media_id

> crate::models::MessageMediaData get_conversations_message_communication_messages_media_media_id(conversation_id, communication_id, media_id)
Get media

See https://developer.genesys.cloud/api/rest/v2/conversations/messaging-media-upload for example usage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**communication_id** | **String** | communicationId | [required] |
**media_id** | **String** | mediaId | [required] |

### Return type

[**crate::models::MessageMediaData**](MessageMediaData.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_message_details

> crate::models::MessageData get_conversations_message_details(message_id)
Get message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | messageId | [required] |

### Return type

[**crate::models::MessageData**](MessageData.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_message_message

> crate::models::MessageData get_conversations_message_message(conversation_id, message_id)
Get conversation message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**message_id** | **String** | messageId | [required] |

### Return type

[**crate::models::MessageData**](MessageData.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_message_participant_wrapup

> crate::models::AssignedWrapupCode get_conversations_message_participant_wrapup(conversation_id, participant_id, provisional)
Get the wrap-up for this conversation participant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**provisional** | Option<**bool**> | Indicates if the wrap-up code is provisional. |  |[default to false]

### Return type

[**crate::models::AssignedWrapupCode**](AssignedWrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_message_participant_wrapupcodes

> Vec<crate::models::WrapupCode> get_conversations_message_participant_wrapupcodes(conversation_id, participant_id)
Get list of wrapup codes for this conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** |  conversationId | [required] |
**participant_id** | **String** | participantId | [required] |

### Return type

[**Vec<crate::models::WrapupCode>**](WrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messages

> crate::models::MessageConversationEntityListing get_conversations_messages()
Get active message conversations for the logged in user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MessageConversationEntityListing**](MessageConversationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_facebook_app

> crate::models::FacebookAppCredentials get_conversations_messaging_facebook_app()
Get Genesys Facebook App Id

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FacebookAppCredentials**](FacebookAppCredentials.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations

> crate::models::MessagingIntegrationEntityListing get_conversations_messaging_integrations(page_size, page_number, expand, supported_content_id)
Get a list of Integrations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<**String**> | Expand instructions for the return value. |  |
**supported_content_id** | Option<**String**> | Filter integrations returned based on the supported content ID |  |

### Return type

[**crate::models::MessagingIntegrationEntityListing**](MessagingIntegrationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations_facebook

> crate::models::FacebookIntegrationEntityListing get_conversations_messaging_integrations_facebook(page_size, page_number, expand, supported_content_id)
Get a list of Facebook Integrations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<**String**> | Expand instructions for the return value. |  |
**supported_content_id** | Option<**String**> | Filter integrations returned based on the supported content ID |  |

### Return type

[**crate::models::FacebookIntegrationEntityListing**](FacebookIntegrationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations_facebook_integration_id

> crate::models::FacebookIntegration get_conversations_messaging_integrations_facebook_integration_id(integration_id, expand)
Get a Facebook messaging integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |
**expand** | Option<**String**> | Expand instructions for the return value. |  |

### Return type

[**crate::models::FacebookIntegration**](FacebookIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations_line

> crate::models::LineIntegrationEntityListing get_conversations_messaging_integrations_line(page_size, page_number, expand, supported_content_id)
Get a list of LINE messenger Integrations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<**String**> | Expand instructions for the return value. |  |
**supported_content_id** | Option<**String**> | Filter integrations returned based on the supported content ID |  |

### Return type

[**crate::models::LineIntegrationEntityListing**](LineIntegrationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations_line_integration_id

> crate::models::LineIntegration get_conversations_messaging_integrations_line_integration_id(integration_id, expand)
Get a LINE messenger integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |
**expand** | Option<**String**> | Expand instructions for the return value. |  |

### Return type

[**crate::models::LineIntegration**](LineIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations_open

> crate::models::OpenIntegrationEntityListing get_conversations_messaging_integrations_open(page_size, page_number, expand, supported_content_id)
Get a list of Open messaging integrations

See https://developer.genesys.cloud/api/digital/openmessaging/ for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<**String**> | Expand instructions for the return value. |  |
**supported_content_id** | Option<**String**> | Filter integrations returned based on the supported content ID |  |

### Return type

[**crate::models::OpenIntegrationEntityListing**](OpenIntegrationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations_open_integration_id

> crate::models::OpenIntegration get_conversations_messaging_integrations_open_integration_id(integration_id, expand)
Get an Open messaging integration

See https://developer.genesys.cloud/api/digital/openmessaging/ for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |
**expand** | Option<**String**> | Expand instructions for the return value. |  |

### Return type

[**crate::models::OpenIntegration**](OpenIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations_twitter

> crate::models::TwitterIntegrationEntityListing get_conversations_messaging_integrations_twitter(page_size, page_number, expand, supported_content_id)
Get a list of Twitter Integrations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<**String**> | Expand instructions for the return value. |  |
**supported_content_id** | Option<**String**> | Filter integrations returned based on the supported content ID |  |

### Return type

[**crate::models::TwitterIntegrationEntityListing**](TwitterIntegrationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations_twitter_integration_id

> crate::models::TwitterIntegration get_conversations_messaging_integrations_twitter_integration_id(integration_id, expand)
Get a Twitter messaging integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |
**expand** | Option<**String**> | Expand instructions for the return value. |  |

### Return type

[**crate::models::TwitterIntegration**](TwitterIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations_whatsapp

> crate::models::WhatsAppIntegrationEntityListing get_conversations_messaging_integrations_whatsapp(page_size, page_number, expand, supported_content_id)
Get a list of WhatsApp Integrations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<**String**> | Expand instructions for the return value. |  |
**supported_content_id** | Option<**String**> | Filter integrations returned based on the supported content ID |  |

### Return type

[**crate::models::WhatsAppIntegrationEntityListing**](WhatsAppIntegrationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_integrations_whatsapp_integration_id

> crate::models::WhatsAppIntegration get_conversations_messaging_integrations_whatsapp_integration_id(integration_id, expand)
Get a WhatsApp messaging integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |
**expand** | Option<**String**> | Expand instructions for the return value. |  |

### Return type

[**crate::models::WhatsAppIntegration**](WhatsAppIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_sticker

> crate::models::MessagingStickerEntityListing get_conversations_messaging_sticker(messenger_type, page_size, page_number)
Get a list of Messaging Stickers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messenger_type** | **String** | Messenger Type | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::MessagingStickerEntityListing**](MessagingStickerEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_supportedcontent

> crate::models::SupportedContentListing get_conversations_messaging_supportedcontent(page_size, page_number)
Get a list of Supported Content profiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::SupportedContentListing**](SupportedContentListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_supportedcontent_default

> crate::models::SupportedContent get_conversations_messaging_supportedcontent_default()
Get the organization's default supported content profile that will be used as the default when creating an integration.

When an integration is created a supported content ID may be assigned to it. If the supported content ID is not supplied, the default supported content profile will be assigned to it.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SupportedContent**](SupportedContent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_supportedcontent_supported_content_id

> crate::models::SupportedContent get_conversations_messaging_supportedcontent_supported_content_id(supported_content_id)
Get a supported content profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supported_content_id** | **String** | Supported Content ID | [required] |

### Return type

[**crate::models::SupportedContent**](SupportedContent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations_messaging_threadingtimeline

> crate::models::ConversationThreadingWindow get_conversations_messaging_threadingtimeline()
Get conversation threading window timeline for each messaging type

Conversation messaging threading timeline is a setting defined for each messenger type in your organization. This setting will dictate whether a new message is added to the most recent existing conversation, or creates a new Conversation. If the existing Conversation is still in a connected state the threading timeline setting will never play a role. After the conversation is disconnected, if an inbound message is received or an outbound message is sent after the setting for threading timeline expires, a new conversation is created.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ConversationThreadingWindow**](ConversationThreadingWindow.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversation_participant

> patch_conversation_participant(conversation_id, participant_id, body)
Update a participant.

Update conversation participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Update request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversation_participant_attributes

> patch_conversation_participant_attributes(conversation_id, participant_id, body)
Update the attributes on a conversation participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |
**body** | [**ParticipantAttributes**](ParticipantAttributes.md) | Participant attributes | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_call

> crate::models::Conversation patch_conversations_call(conversation_id, body)
Update a conversation by setting it's recording state, merging in other conversations to create a conference, or disconnecting all of the participants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**Conversation**](Conversation.md) | Conversation | [required] |

### Return type

[**crate::models::Conversation**](Conversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_call_participant

> patch_conversations_call_participant(conversation_id, participant_id, body)
Update conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Participant request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_call_participant_attributes

> patch_conversations_call_participant_attributes(conversation_id, participant_id, body)
Update the attributes on a conversation participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**ParticipantAttributes**](ParticipantAttributes.md) | Participant attributes | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_call_participant_communication

> serde_json::Value patch_conversations_call_participant_communication(conversation_id, participant_id, communication_id, body)
Update conversation participant's communication by disconnecting it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**communication_id** | **String** | communicationId | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Participant | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_call_participant_consult

> crate::models::ConsultTransferResponse patch_conversations_call_participant_consult(conversation_id, participant_id, body)
Change who can speak

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**ConsultTransferUpdate**](ConsultTransferUpdate.md) | new speak to | [required] |

### Return type

[**crate::models::ConsultTransferResponse**](ConsultTransferResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_callback

> crate::models::Conversation patch_conversations_callback(conversation_id, body)
Update a conversation by disconnecting all of the participants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**Conversation**](Conversation.md) | Conversation | [required] |

### Return type

[**crate::models::Conversation**](Conversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_callback_participant

> patch_conversations_callback_participant(conversation_id, participant_id, body)
Update conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Participant | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_callback_participant_attributes

> patch_conversations_callback_participant_attributes(conversation_id, participant_id, body)
Update the attributes on a conversation participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**ParticipantAttributes**](ParticipantAttributes.md) | Attributes | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_callback_participant_communication

> serde_json::Value patch_conversations_callback_participant_communication(conversation_id, participant_id, communication_id, body)
Update conversation participant's communication by disconnecting it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**communication_id** | **String** | communicationId | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Participant | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_chat

> crate::models::Conversation patch_conversations_chat(conversation_id, body)
Update a conversation by disconnecting all of the participants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**Conversation**](Conversation.md) | Conversation | [required] |

### Return type

[**crate::models::Conversation**](Conversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_chat_participant

> patch_conversations_chat_participant(conversation_id, participant_id, body)
Update conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Update request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_chat_participant_attributes

> patch_conversations_chat_participant_attributes(conversation_id, participant_id, body)
Update the attributes on a conversation participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**ParticipantAttributes**](ParticipantAttributes.md) | Participant attributes | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_chat_participant_communication

> serde_json::Value patch_conversations_chat_participant_communication(conversation_id, participant_id, communication_id, body)
Update conversation participant's communication by disconnecting it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**communication_id** | **String** | communicationId | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Participant | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_cobrowsesession

> crate::models::Conversation patch_conversations_cobrowsesession(conversation_id, body)
Update a conversation by disconnecting all of the participants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**Conversation**](Conversation.md) | Conversation | [required] |

### Return type

[**crate::models::Conversation**](Conversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_cobrowsesession_participant

> patch_conversations_cobrowsesession_participant(conversation_id, participant_id, body)
Update conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | Option<[**MediaParticipantRequest**](MediaParticipantRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_cobrowsesession_participant_attributes

> patch_conversations_cobrowsesession_participant_attributes(conversation_id, participant_id, body)
Update the attributes on a conversation participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | Option<[**ParticipantAttributes**](ParticipantAttributes.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_cobrowsesession_participant_communication

> serde_json::Value patch_conversations_cobrowsesession_participant_communication(conversation_id, participant_id, communication_id, body)
Update conversation participant's communication by disconnecting it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**communication_id** | **String** | communicationId | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Participant | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_email

> crate::models::Conversation patch_conversations_email(conversation_id, body)
Update a conversation by disconnecting all of the participants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**Conversation**](Conversation.md) | Conversation | [required] |

### Return type

[**crate::models::Conversation**](Conversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_email_participant

> patch_conversations_email_participant(conversation_id, participant_id, body)
Update conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Update request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_email_participant_attributes

> patch_conversations_email_participant_attributes(conversation_id, participant_id, body)
Update the attributes on a conversation participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**ParticipantAttributes**](ParticipantAttributes.md) | Participant attributes | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_email_participant_communication

> serde_json::Value patch_conversations_email_participant_communication(conversation_id, participant_id, communication_id, body)
Update conversation participant's communication by disconnecting it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**communication_id** | **String** | communicationId | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Participant | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_message

> crate::models::Conversation patch_conversations_message(conversation_id, body)
Update a conversation by disconnecting all of the participants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**Conversation**](Conversation.md) | Conversation | [required] |

### Return type

[**crate::models::Conversation**](Conversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_message_participant

> patch_conversations_message_participant(conversation_id, participant_id, body)
Update conversation participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** |  conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | Option<[**MediaParticipantRequest**](MediaParticipantRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_message_participant_attributes

> patch_conversations_message_participant_attributes(conversation_id, participant_id, body)
Update the attributes on a conversation participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** |  conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | Option<[**ParticipantAttributes**](ParticipantAttributes.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_message_participant_communication

> serde_json::Value patch_conversations_message_participant_communication(conversation_id, participant_id, communication_id, body)
Update conversation participant's communication by disconnecting it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** |  conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**communication_id** | **String** | communicationId | [required] |
**body** | [**MediaParticipantRequest**](MediaParticipantRequest.md) | Participant | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_messaging_integrations_facebook_integration_id

> crate::models::FacebookIntegration patch_conversations_messaging_integrations_facebook_integration_id(integration_id, body)
Update Facebook messaging integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |
**body** | [**FacebookIntegrationUpdateRequest**](FacebookIntegrationUpdateRequest.md) | FacebookIntegrationUpdateRequest | [required] |

### Return type

[**crate::models::FacebookIntegration**](FacebookIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_messaging_integrations_open_integration_id

> crate::models::OpenIntegration patch_conversations_messaging_integrations_open_integration_id(integration_id, body)
Update an Open messaging integration

See https://developer.genesys.cloud/api/digital/openmessaging/ for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |
**body** | [**OpenIntegrationUpdateRequest**](OpenIntegrationUpdateRequest.md) | OpenIntegrationUpdateRequest | [required] |

### Return type

[**crate::models::OpenIntegration**](OpenIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_messaging_integrations_twitter_integration_id

> crate::models::TwitterIntegration patch_conversations_messaging_integrations_twitter_integration_id(integration_id, body)
Update Twitter messaging integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |
**body** | [**TwitterIntegrationRequest**](TwitterIntegrationRequest.md) | TwitterIntegrationRequest | [required] |

### Return type

[**crate::models::TwitterIntegration**](TwitterIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_messaging_integrations_whatsapp_integration_id

> crate::models::WhatsAppIntegration patch_conversations_messaging_integrations_whatsapp_integration_id(integration_id, body)
Update or activate a WhatsApp messaging integration.

The following steps are required in order to fully activate a Whatsapp Integration: Initially, you will need to get an activation code by sending: an action set to Activate, and an authenticationMethod choosing from Sms or Voice. Finally, once you have been informed of an activation code on selected authenticationMethod, you will need to confirm the code by sending: an action set to Confirm, and the confirmationCode you have received from Whatsapp.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |
**body** | [**WhatsAppIntegrationUpdateRequest**](WhatsAppIntegrationUpdateRequest.md) | WhatsAppIntegrationUpdateRequest | [required] |

### Return type

[**crate::models::WhatsAppIntegration**](WhatsAppIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_conversations_messaging_supportedcontent_supported_content_id

> crate::models::SupportedContent patch_conversations_messaging_supportedcontent_supported_content_id(supported_content_id, body)
Update a supported content profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supported_content_id** | **String** | Supported Content ID | [required] |
**body** | [**SupportedContent**](SupportedContent.md) | SupportedContent | [required] |

### Return type

[**crate::models::SupportedContent**](SupportedContent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_conversation_details_properties

> crate::models::PropertyIndexRequest post_analytics_conversation_details_properties(conversation_id, body)
Index conversation properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**PropertyIndexRequest**](PropertyIndexRequest.md) | request | [required] |

### Return type

[**crate::models::PropertyIndexRequest**](PropertyIndexRequest.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_conversations_aggregates_query

> crate::models::ConversationAggregateQueryResponse post_analytics_conversations_aggregates_query(body)
Query for conversation aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ConversationAggregationQuery**](ConversationAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::ConversationAggregateQueryResponse**](ConversationAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_conversations_details_jobs

> crate::models::AsyncQueryResponse post_analytics_conversations_details_jobs(body)
Query for conversation details asynchronously

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AsyncConversationQuery**](AsyncConversationQuery.md) | query | [required] |

### Return type

[**crate::models::AsyncQueryResponse**](AsyncQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_conversations_details_query

> crate::models::AnalyticsConversationQueryResponse post_analytics_conversations_details_query(body)
Query for conversation details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ConversationQuery**](ConversationQuery.md) | query | [required] |

### Return type

[**crate::models::AnalyticsConversationQueryResponse**](AnalyticsConversationQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversation_assign

> String post_conversation_assign(conversation_id, body)
Attempts to manually assign a specified conversation to a specified user.  Ignores bullseye ring, PAR score, skills, and languages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**body** | [**ConversationUser**](ConversationUser.md) | Targeted user | [required] |

### Return type

**String**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversation_disconnect

> String post_conversation_disconnect(conversation_id)
Performs a full conversation teardown. Issues disconnect requests for any connected media. Applies a system wrap-up code to any participants that are pending wrap-up. This is not intended to be the normal way of ending interactions but is available in the event of problems with the application to allow a resynchronization of state across all components. It is recommended that users submit a support case if they are relying on this endpoint systematically as there is likely something that needs investigation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |

### Return type

**String**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversation_participant_callbacks

> post_conversation_participant_callbacks(conversation_id, participant_id, body)
Create a new callback for the specified participant on the conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |
**body** | Option<[**CreateCallbackOnConversationCommand**](CreateCallbackOnConversationCommand.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversation_participant_digits

> post_conversation_participant_digits(conversation_id, participant_id, body)
Sends DTMF to the participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |
**body** | Option<[**Digits**](Digits.md)> | Digits |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversation_participant_replace

> post_conversation_participant_replace(conversation_id, participant_id, body)
Replace this participant with the specified user and/or address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |
**body** | [**TransferRequest**](TransferRequest.md) | Transfer request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversation_participant_secureivrsessions

> crate::models::SecureSession post_conversation_participant_secureivrsessions(conversation_id, participant_id, body)
Create secure IVR session. Only a participant in the conversation can invoke a secure IVR.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |
**body** | Option<[**CreateSecureSession**](CreateSecureSession.md)> |  |  |

### Return type

[**crate::models::SecureSession**](SecureSession.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_call

> crate::models::Conversation post_conversations_call(conversation_id, body)
Place a new call as part of a callback conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**CallCommand**](CallCommand.md) | Conversation | [required] |

### Return type

[**crate::models::Conversation**](Conversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_call_participant_coach

> post_conversations_call_participant_coach(conversation_id, participant_id)
Listen in on the conversation from the point of view of a given participant while speaking to just the given participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_call_participant_consult

> crate::models::ConsultTransferResponse post_conversations_call_participant_consult(conversation_id, participant_id, body)
Initiate and update consult transfer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**ConsultTransfer**](ConsultTransfer.md) | Destination address & initial speak to | [required] |

### Return type

[**crate::models::ConsultTransferResponse**](ConsultTransferResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_call_participant_monitor

> post_conversations_call_participant_monitor(conversation_id, participant_id)
Listen in on the conversation from the point of view of a given participant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_call_participant_replace

> post_conversations_call_participant_replace(conversation_id, participant_id, body)
Replace this participant with the specified user and/or address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**TransferRequest**](TransferRequest.md) | Transfer request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_call_participants

> crate::models::Conversation post_conversations_call_participants(conversation_id, body)
Add participants to a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**Conversation**](Conversation.md) | Conversation | [required] |

### Return type

[**crate::models::Conversation**](Conversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_callback_participant_replace

> post_conversations_callback_participant_replace(conversation_id, participant_id, body)
Replace this participant with the specified user and/or address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**TransferRequest**](TransferRequest.md) | Transfer request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_callbacks

> crate::models::CreateCallbackResponse post_conversations_callbacks(body)
Create a Callback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateCallbackCommand**](CreateCallbackCommand.md) | Callback | [required] |

### Return type

[**crate::models::CreateCallbackResponse**](CreateCallbackResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_calls

> crate::models::CreateCallResponse post_conversations_calls(body)
Create a call conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateCallRequest**](CreateCallRequest.md) | Call request | [required] |

### Return type

[**crate::models::CreateCallResponse**](CreateCallResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_chat_communication_messages

> crate::models::WebChatMessage post_conversations_chat_communication_messages(conversation_id, communication_id, body)
Send a message on behalf of a communication in a chat conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**communication_id** | **String** | communicationId | [required] |
**body** | [**CreateWebChatMessageRequest**](CreateWebChatMessageRequest.md) | Message | [required] |

### Return type

[**crate::models::WebChatMessage**](WebChatMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_chat_communication_typing

> crate::models::WebChatTyping post_conversations_chat_communication_typing(conversation_id, communication_id)
Send a typing-indicator on behalf of a communication in a chat conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**communication_id** | **String** | communicationId | [required] |

### Return type

[**crate::models::WebChatTyping**](WebChatTyping.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_chat_participant_replace

> post_conversations_chat_participant_replace(conversation_id, participant_id, body)
Replace this participant with the specified user and/or address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**TransferRequest**](TransferRequest.md) | Transfer request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_chats

> crate::models::ChatConversation post_conversations_chats(body)
Create a web chat conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateWebChatRequest**](CreateWebChatRequest.md) | Create web chat request | [required] |

### Return type

[**crate::models::ChatConversation**](ChatConversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_cobrowsesession_participant_replace

> post_conversations_cobrowsesession_participant_replace(conversation_id, participant_id, body)
Replace this participant with the specified user and/or address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | Option<[**TransferRequest**](TransferRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_email_inboundmessages

> crate::models::EmailConversation post_conversations_email_inboundmessages(conversation_id, body)
Send an email to an external conversation. An external conversation is one where the provider is not PureCloud based. This endpoint allows the sender of the external email to reply or send a new message to the existing conversation. The new message will be treated as part of the existing conversation and chained to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**InboundMessageRequest**](InboundMessageRequest.md) | Send external email reply | [required] |

### Return type

[**crate::models::EmailConversation**](EmailConversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_email_messages

> crate::models::EmailMessage post_conversations_email_messages(conversation_id, body)
Send an email reply

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**EmailMessage**](EmailMessage.md) | Reply | [required] |

### Return type

[**crate::models::EmailMessage**](EmailMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_email_messages_draft_attachments_copy

> crate::models::EmailMessage post_conversations_email_messages_draft_attachments_copy(conversation_id, body)
Copy attachments from an email message to the current draft.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**CopyAttachmentsRequest**](CopyAttachmentsRequest.md) | Copy Attachment Request | [required] |

### Return type

[**crate::models::EmailMessage**](EmailMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_email_participant_replace

> post_conversations_email_participant_replace(conversation_id, participant_id, body)
Replace this participant with the specified user and/or address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**TransferRequest**](TransferRequest.md) | Transfer request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_emails

> crate::models::EmailConversation post_conversations_emails(body)
Create an email conversation

If the direction of the request is INBOUND, this will create an external conversation with a third party provider. If the direction of the the request is OUTBOUND, this will create a conversation to send outbound emails on behalf of a queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateEmailRequest**](CreateEmailRequest.md) | Create email request | [required] |

### Return type

[**crate::models::EmailConversation**](EmailConversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_faxes

> crate::models::FaxSendResponse post_conversations_faxes(body)
Create Fax Conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FaxSendRequest**](FaxSendRequest.md) | Fax | [required] |

### Return type

[**crate::models::FaxSendResponse**](FaxSendResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_message_communication_messages

> crate::models::MessageData post_conversations_message_communication_messages(conversation_id, communication_id, body)
Send message

Send message on existing conversation/communication. Only one message body field can be accepted, per request. Example: 1 textBody, 1 mediaId, 1 stickerId, or 1 messageTemplate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**communication_id** | **String** | communicationId | [required] |
**body** | [**AdditionalMessage**](AdditionalMessage.md) | Message | [required] |

### Return type

[**crate::models::MessageData**](MessageData.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_message_communication_messages_media

> crate::models::MessageMediaData post_conversations_message_communication_messages_media(conversation_id, communication_id)
Create media

See https://developer.genesys.cloud/api/rest/v2/conversations/messaging-media-upload for example usage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**communication_id** | **String** | communicationId | [required] |

### Return type

[**crate::models::MessageMediaData**](MessageMediaData.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_message_messages_bulk

> crate::models::TextMessageListing post_conversations_message_messages_bulk(conversation_id, body)
Get messages in batch

The path parameter [conversationId] should contain the conversationId of the conversation being filtered. The body should contain the messageId(s) of messages being requested. For example: [\"a3069a33b-bbb1-4703-9d68-061d9e9db96e\", \"55bc6be3-078c-4a49-a4e6-1e05776ed7e8\"]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** |  | [required] |
**body** | Option<[**Vec<String>**](String.md)> | messageIds |  |

### Return type

[**crate::models::TextMessageListing**](TextMessageListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_message_participant_replace

> post_conversations_message_participant_replace(conversation_id, participant_id, body)
Replace this participant with the specified user and/or address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**body** | [**TransferRequest**](TransferRequest.md) | Transfer request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_messages

> crate::models::MessageConversation post_conversations_messages(body)
Create an outbound messaging conversation.

If there is an existing conversation between the remote address and the address associated with the queue specified in createOutboundRequest then the result of this request depends on the state of that conversation and the useExistingConversation field of createOutboundRequest. If the existing conversation is in alerting or connected state, then the request will fail. If the existing conversation is disconnected but still within the conversation window then the request will fail unless useExistingConversation is set to true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateOutboundMessagingConversationRequest**](CreateOutboundMessagingConversationRequest.md) | Create outbound messaging conversation | [required] |

### Return type

[**crate::models::MessageConversation**](MessageConversation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_messages_agentless

> crate::models::SendAgentlessOutboundMessageResponse post_conversations_messages_agentless(body)
Send an agentless outbound message

Send an agentlesss (api participant) outbound message using a client credential grant. In order to call this endpoint you will need OAuth token generated using OAuth client credentials authorized with at least messaging scope. This will generate a new Conversation, if there is an existing active Conversation between the fromAddress and toAddress already, then this POST will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SendAgentlessOutboundMessageRequest**](SendAgentlessOutboundMessageRequest.md) | Create agentless outbound messaging request | [required] |

### Return type

[**crate::models::SendAgentlessOutboundMessageResponse**](SendAgentlessOutboundMessageResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_messages_inbound_open

> crate::models::OpenNormalizedMessage post_conversations_messages_inbound_open(body)
Send an inbound Open Message

Send an inbound message to an Open Messaging integration. In order to call this endpoint you will need OAuth token generated using OAuth client credentials authorized with at least messaging scope. This will either generate a new Conversation, or be a part of an existing conversation. See https://developer.genesys.cloud/api/digital/openmessaging/ for example usage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OpenNormalizedMessage**](OpenNormalizedMessage.md) | NormalizedMessage | [required] |

### Return type

[**crate::models::OpenNormalizedMessage**](OpenNormalizedMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_messaging_integrations_facebook

> crate::models::FacebookIntegration post_conversations_messaging_integrations_facebook(body)
Create a Facebook Integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FacebookIntegrationRequest**](FacebookIntegrationRequest.md) | FacebookIntegrationRequest | [required] |

### Return type

[**crate::models::FacebookIntegration**](FacebookIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_messaging_integrations_line

> crate::models::LineIntegration post_conversations_messaging_integrations_line(body)
Create a LINE messenger Integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LineIntegrationRequest**](LineIntegrationRequest.md) | LineIntegrationRequest | [required] |

### Return type

[**crate::models::LineIntegration**](LineIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_messaging_integrations_open

> crate::models::OpenIntegration post_conversations_messaging_integrations_open(body)
Create an Open messaging integration

See https://developer.genesys.cloud/api/digital/openmessaging/ for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OpenIntegrationRequest**](OpenIntegrationRequest.md) | OpenIntegrationRequest | [required] |

### Return type

[**crate::models::OpenIntegration**](OpenIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_messaging_integrations_twitter

> crate::models::TwitterIntegration post_conversations_messaging_integrations_twitter(body)
Create a Twitter Integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TwitterIntegrationRequest**](TwitterIntegrationRequest.md) | TwitterIntegrationRequest | [required] |

### Return type

[**crate::models::TwitterIntegration**](TwitterIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_messaging_integrations_whatsapp

> crate::models::WhatsAppIntegration post_conversations_messaging_integrations_whatsapp(body)
Create a WhatsApp Integration

You must be approved by WhatsApp to use this feature. Your approved e164-formatted phone number and valid WhatsApp certificate for your number are required. Your WhatsApp certificate must have valid base64 encoding. Please paste carefully and do not add any leading or trailing spaces. Do not alter any characters. An integration must be activated within 7 days of certificate generation. If you cannot complete the addition and activation of the number within 7 days, please obtain a new certificate before creating the integration. Integrations created with an invalid number or certificate may immediately incur additional integration fees. Please carefully enter your number and certificate as described.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WhatsAppIntegrationRequest**](WhatsAppIntegrationRequest.md) | WhatsAppIntegrationRequest | [required] |

### Return type

[**crate::models::WhatsAppIntegration**](WhatsAppIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversations_messaging_supportedcontent

> crate::models::SupportedContent post_conversations_messaging_supportedcontent(body)
Create a Supported Content profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SupportedContent**](SupportedContent.md) | SupportedContent | [required] |

### Return type

[**crate::models::SupportedContent**](SupportedContent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_conversation_participant_flaggedreason

> put_conversation_participant_flaggedreason(conversation_id, participant_id)
Set flagged reason on conversation participant to indicate bad conversation quality.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**participant_id** | **String** | participant ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_conversation_tags

> String put_conversation_tags(conversation_id, body)
Update the tags on a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversation ID | [required] |
**body** | [**ConversationTagsUpdate**](ConversationTagsUpdate.md) | Conversation Tags | [required] |

### Return type

**String**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_conversations_call_participant_communication_uuidata

> serde_json::Value put_conversations_call_participant_communication_uuidata(conversation_id, participant_id, communication_id, body)
Set uuiData to be sent on future commands.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**participant_id** | **String** | participantId | [required] |
**communication_id** | **String** | communicationId | [required] |
**body** | [**SetUuiDataRequest**](SetUuiDataRequest.md) | UUIData Request | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_conversations_email_messages_draft

> crate::models::EmailMessage put_conversations_email_messages_draft(conversation_id, body)
Update conversation draft reply

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**EmailMessage**](EmailMessage.md) | Draft | [required] |

### Return type

[**crate::models::EmailMessage**](EmailMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_conversations_messaging_integrations_line_integration_id

> crate::models::LineIntegration put_conversations_messaging_integrations_line_integration_id(integration_id, body)
Update a LINE messenger integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration ID | [required] |
**body** | [**LineIntegrationRequest**](LineIntegrationRequest.md) | LineIntegrationRequest | [required] |

### Return type

[**crate::models::LineIntegration**](LineIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_conversations_messaging_supportedcontent_default

> crate::models::SupportedContent put_conversations_messaging_supportedcontent_default(body)
Set the organization's default supported content profile that may be assigned to an integration when it is created.

When an integration is created a supported content ID may be assigned to it. If the supported content ID is not supplied, the default supported content profile will be assigned to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SupportedContentReference**](SupportedContentReference.md) | SupportedContent | [required] |

### Return type

[**crate::models::SupportedContent**](SupportedContent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_conversations_messaging_threadingtimeline

> crate::models::ConversationThreadingWindow put_conversations_messaging_threadingtimeline(body)
Update conversation threading window timeline for each messaging type

PUT Conversation messaging threading timeline is intended to set the conversation threading settings for ALL messengerTypes. If you omit a messengerType in the request body then the setting for that messengerType will use the platform default value. The PUT replaces the existing setting(s) that were previously set for each messengerType.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ConversationThreadingWindow**](ConversationThreadingWindow.md) | ConversationThreadingWindowRequest | [required] |

### Return type

[**crate::models::ConversationThreadingWindow**](ConversationThreadingWindow.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

