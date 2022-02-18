# IntentDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the intent. | 
**entity_type_bindings** | Option<[**Vec<crate::models::NamedEntityTypeBinding>**](NamedEntityTypeBinding.md)> | The bindings for the named entity types used in this intent.This field is mutually exclusive with entityNameReferences and entities | [optional]
**entity_name_references** | Option<**Vec<String>**> | The references for the named entity used in this intent.This field is mutually exclusive with entityTypeBindings | [optional][readonly]
**utterances** | [**Vec<crate::models::NluUtterance>**](NluUtterance.md) | The utterances that act as training phrases for the intent. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


