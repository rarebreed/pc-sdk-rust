# ResponseConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**translation_map** | Option<**::std::collections::HashMap<String, String>**> | Map 'attribute name' and 'JSON path' pairs used to extract data from REST response. | [optional]
**translation_map_defaults** | Option<**::std::collections::HashMap<String, String>**> | Map 'attribute name' and 'default value' pairs used as fallback values if JSON path extraction fails for specified key. | [optional]
**success_template** | Option<**String**> | Velocity template to build response to return from Action. | [optional]
**success_template_uri** | Option<**String**> | URI to retrieve success template. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


