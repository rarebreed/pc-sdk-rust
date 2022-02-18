# ScimV2SchemaAttribute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the attribute. | [optional][readonly]
**_type** | Option<**String**> | The data type of the attribute. | [optional][readonly]
**sub_attributes** | Option<[**Vec<crate::models::ScimV2SchemaAttribute>**](ScimV2SchemaAttribute.md)> | The list of subattributes for an attribute of the type \"complex\". Uses the same schema as \"attributes\". | [optional][readonly]
**multi_valued** | Option<**bool**> | Indicates whether an attribute contains multiple values. | [optional][readonly]
**description** | Option<**String**> | The description of the attribute. | [optional][readonly]
**required** | Option<**bool**> | Indicates whether an attribute is required. | [optional][readonly]
**canonical_values** | Option<**Vec<String>**> | The list of standard values that service providers may use. Service providers may ignore unsupported values. | [optional][readonly]
**case_exact** | Option<**bool**> | Indicates whether a string attribute is case-sensitive. If set to \"true\", the server preserves case sensitivity. If set to \"false\", the server may change the case. The server also uses case sensitivity when evaluating filters. See section 3.4.2.2 \"Filtering\" in RFC 7644 for details. | [optional][readonly]
**mutability** | Option<**String**> | The circumstances under which an attribute can be defined or redefined. The default is \"readWrite\". | [optional][readonly]
**returned** | Option<**String**> | The circumstances under which an attribute and its values are returned in response to a GET, PUT, POST, or PATCH request. | [optional][readonly]
**uniqueness** | Option<**String**> | The method by which the service provider enforces the uniqueness of an attribute value. A server can reject a value by returning the HTTP response code 400 (Bad Request). A client can enforce uniqueness to a greater degree than the server provider enforces. For example, a client could make a value unique even though the server has \"uniqueness\" set to \"none\". | [optional][readonly]
**reference_types** | Option<**Vec<String>**> | The list of SCIM resource types that may be referenced. Only applies when \"type\" is set to \"reference\". | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


