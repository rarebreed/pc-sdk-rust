# JourneyEventsSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Whether or not journey event collection is enabled. | [optional]
**excluded_query_parameters** | Option<**Vec<String>**> | List of parameters to be excluded from the query string. | [optional]
**should_keep_url_fragment** | Option<**bool**> | Whether or not to keep the URL fragment. | [optional]
**search_query_parameters** | Option<**Vec<String>**> | List of query parameters used for search (e.g. 'q'). | [optional]
**pageview_config** | Option<**String**> | Controls how the pageview events are tracked. | [optional]
**click_events** | Option<[**Vec<crate::models::SelectorEventTrigger>**](SelectorEventTrigger.md)> | Tracks when and where a visitor clicks on a webpage. | [optional]
**forms_track_events** | Option<[**Vec<crate::models::FormsTrackTrigger>**](FormsTrackTrigger.md)> | Controls how the form submitted and form abandoned events are tracked after a visitor interacts with a form element. | [optional]
**idle_events** | Option<[**Vec<crate::models::IdleEventTrigger>**](IdleEventTrigger.md)> | Tracks when and where a visitor becomes inactive on a webpage. | [optional]
**in_viewport_events** | Option<[**Vec<crate::models::SelectorEventTrigger>**](SelectorEventTrigger.md)> | Tracks when elements become visible or hidden on screen. | [optional]
**scroll_depth_events** | Option<[**Vec<crate::models::ScrollPercentageEventTrigger>**](ScrollPercentageEventTrigger.md)> | Tracks when a visitor scrolls to a specific percentage of a webpage. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


