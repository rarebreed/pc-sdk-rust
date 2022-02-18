# TrustUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**division** | Option<[**crate::models::Division**](Division.md)> |  | [optional]
**chat** | Option<[**crate::models::Chat**](Chat.md)> |  | [optional]
**department** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**primary_contact_info** | Option<[**Vec<crate::models::Contact>**](Contact.md)> | Auto populated from addresses. | [optional][readonly]
**addresses** | Option<[**Vec<crate::models::Contact>**](Contact.md)> | Email addresses and phone numbers for this user | [optional]
**state** | Option<**String**> | The current state for this user. | [optional][readonly]
**title** | Option<**String**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**manager** | Option<[**crate::models::User**](User.md)> |  | [optional]
**images** | Option<[**Vec<crate::models::UserImage>**](UserImage.md)> |  | [optional]
**version** | **i32** | Required when updating a user, this value should be the current version of the user.  The current version can be obtained with a GET on the user before doing a PATCH. | 
**certifications** | Option<**Vec<String>**> |  | [optional]
**biography** | Option<[**crate::models::Biography**](Biography.md)> |  | [optional]
**employer_info** | Option<[**crate::models::EmployerInfo**](EmployerInfo.md)> |  | [optional]
**routing_status** | Option<[**crate::models::RoutingStatus**](RoutingStatus.md)> |  | [optional]
**presence** | Option<[**crate::models::UserPresence**](UserPresence.md)> |  | [optional]
**integration_presence** | Option<[**crate::models::UserPresence**](UserPresence.md)> |  | [optional]
**conversation_summary** | Option<[**crate::models::UserConversationSummary**](UserConversationSummary.md)> |  | [optional]
**out_of_office** | Option<[**crate::models::OutOfOffice**](OutOfOffice.md)> |  | [optional]
**geolocation** | Option<[**crate::models::Geolocation**](Geolocation.md)> |  | [optional]
**station** | Option<[**crate::models::UserStations**](UserStations.md)> |  | [optional]
**authorization** | Option<[**crate::models::UserAuthorization**](UserAuthorization.md)> |  | [optional]
**profile_skills** | Option<**Vec<String>**> | Profile skills possessed by the user | [optional][readonly]
**locations** | Option<[**Vec<crate::models::Location>**](Location.md)> | The user placement at each site location. | [optional][readonly]
**groups** | Option<[**Vec<crate::models::Group>**](Group.md)> | The groups the user is a member of | [optional][readonly]
**team** | Option<[**crate::models::Team**](Team.md)> |  | [optional]
**skills** | Option<[**Vec<crate::models::UserRoutingSkill>**](UserRoutingSkill.md)> | Routing (ACD) skills possessed by the user | [optional][readonly]
**languages** | Option<[**Vec<crate::models::UserRoutingLanguage>**](UserRoutingLanguage.md)> | Routing (ACD) languages possessed by the user | [optional][readonly]
**acd_auto_answer** | Option<**bool**> | acd auto answer | [optional]
**language_preference** | Option<**String**> | preferred language by the user | [optional][readonly]
**last_token_issued** | Option<[**crate::models::OAuthLastTokenIssued**](OAuthLastTokenIssued.md)> |  | [optional]
**date_last_login** | Option<**String**> | The last time the user logged in using username and password. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**trust_user_details** | Option<[**crate::models::TrustUserDetails**](TrustUserDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


