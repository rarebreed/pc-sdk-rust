/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DialerAction {
    /// The type of this DialerAction.
    #[serde(rename = "type")]
    pub _type: Type,
    /// Additional type specification for this DialerAction.
    #[serde(rename = "actionTypeName")]
    pub action_type_name: ActionTypeName,
    /// Specifies how a contact attribute should be updated. Required for MODIFY_CONTACT_ATTRIBUTE.
    #[serde(rename = "updateOption", skip_serializing_if = "Option::is_none")]
    pub update_option: Option<UpdateOption>,
    /// A map of key-value pairs pertinent to the DialerAction. Different types of DialerActions require different properties. MODIFY_CONTACT_ATTRIBUTE with an updateOption of SET takes a contact column as the key and accepts any value. SCHEDULE_CALLBACK takes a key 'callbackOffset' that specifies how far in the future the callback should be scheduled, in minutes. SET_CALLER_ID takes two keys: 'callerAddress', which should be the caller id phone number, and 'callerName'. For either key, you can also specify a column on the contact to get the value from. To do this, specify 'contact.Column', where 'Column' is the name of the contact column from which to get the value. SET_SKILLS takes a key 'skills' with an array of skill ids wrapped into a string (Example: {'skills': '['skillIdHere']'} ).
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}

impl DialerAction {
    pub fn new(_type: Type, action_type_name: ActionTypeName) -> DialerAction {
        DialerAction {
            _type,
            action_type_name,
            update_option: None,
            properties: None,
        }
    }
}

/// The type of this DialerAction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Action")]
    Action,
    #[serde(rename = "modifyContactAttribute")]
    ModifyContactAttribute,
    #[serde(rename = "dataActionBehavior")]
    DataActionBehavior,
}

impl Default for Type {
    fn default() -> Type {
        Self::Action
    }
}
/// Additional type specification for this DialerAction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionTypeName {
    #[serde(rename = "DO_NOT_DIAL")]
    DONOTDIAL,
    #[serde(rename = "MODIFY_CONTACT_ATTRIBUTE")]
    MODIFYCONTACTATTRIBUTE,
    #[serde(rename = "SWITCH_TO_PREVIEW")]
    SWITCHTOPREVIEW,
    #[serde(rename = "APPEND_NUMBER_TO_DNC_LIST")]
    APPENDNUMBERTODNCLIST,
    #[serde(rename = "SCHEDULE_CALLBACK")]
    SCHEDULECALLBACK,
    #[serde(rename = "CONTACT_UNCALLABLE")]
    CONTACTUNCALLABLE,
    #[serde(rename = "NUMBER_UNCALLABLE")]
    NUMBERUNCALLABLE,
    #[serde(rename = "SET_CALLER_ID")]
    SETCALLERID,
    #[serde(rename = "SET_SKILLS")]
    SETSKILLS,
    #[serde(rename = "DATA_ACTION")]
    DATAACTION,
}

impl Default for ActionTypeName {
    fn default() -> ActionTypeName {
        Self::DONOTDIAL
    }
}
/// Specifies how a contact attribute should be updated. Required for MODIFY_CONTACT_ATTRIBUTE.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UpdateOption {
    #[serde(rename = "SET")]
    SET,
    #[serde(rename = "INCREMENT")]
    INCREMENT,
    #[serde(rename = "DECREMENT")]
    DECREMENT,
    #[serde(rename = "CURRENT_TIME")]
    CURRENTTIME,
}

impl Default for UpdateOption {
    fn default() -> UpdateOption {
        Self::SET
    }
}

