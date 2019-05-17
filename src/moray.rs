/*
 * Copyright 2019 Joyent, Inc.
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(tag = "type")]
pub enum ObjectType {
    #[serde(alias = "object")]
    Object(MantaObject),

    #[serde(alias = "directory")]
    Directory(MantaDirectory)
}

#[derive(Deserialize, Serialize, Default, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MantaObject {
    pub headers: Value,
    pub key: String,
    pub mtime: u64,
    pub name: String,
    pub creator: String,
    pub dirname: String,
    pub owner: String,
    pub roles: Vec<String>, // TODO: double check this is a String
    pub vnode: u64,

    #[serde(alias = "contentLength", default)]
    pub content_length: u64,

    #[serde(alias = "contentMD5", default)]
    pub content_md5: String,

    #[serde(alias = "contentType", default)]
    pub content_type: String,

    #[serde(alias = "objectId", default)]
    pub object_id: String,

    #[serde(default)]
    pub etag: String,

    #[serde(default)]
    pub sharks: Vec<MantaObjectShark>,

}

#[derive(Deserialize, Serialize, Default, PartialEq, Debug, Clone)]
pub struct MantaObjectShark {
    pub datacenter: String,
    pub manta_storage_id: String,
}

#[derive(Deserialize, Default, Serialize, PartialEq, Debug, Clone)]
pub struct MantaDirectory {
    pub creator: String,
    pub dirname: String,
    pub headers: Value,
    pub key: String,
    pub mtime: u64,
    pub name: String,
    pub owner: String,
    pub roles: Vec<String>, // TODO: double check this is a String
    pub vnode: u64,
}
