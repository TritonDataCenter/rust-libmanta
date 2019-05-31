/*
 * Copyright 2019 Joyent, Inc.
 */

use crate::util;
use base64;
use md5;
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(tag = "type")]
pub enum ObjectType {
    #[serde(alias = "object")]
    Object(MantaObject),

    #[serde(alias = "directory")]
    Directory(MantaDirectory),
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

// Implement Arbitrary traits for testing
impl Arbitrary for MantaObjectShark {
    fn arbitrary<G: Gen>(g: &mut G) -> MantaObjectShark {
        let len = g.gen::<u8>() as usize;
        MantaObjectShark {
            datacenter: util::random_string(g, len),
            manta_storage_id: util::random_string(g, len),
        }
    }
}

impl Arbitrary for MantaObject {
    fn arbitrary<G: Gen>(g: &mut G) -> MantaObject {
        let len = g.gen::<u8>() as usize;

        let mut headers_map = Map::new();
        headers_map.insert(
            util::random_string(g, len),
            Value::String(util::random_string(g, len)),
        );

        headers_map.insert(
            util::random_string(g, len),
            Value::String(util::random_string(g, len)),
        );

        headers_map.insert(
            util::random_string(g, len),
            Value::String(util::random_string(g, len)),
        );

        let headers = Value::Object(headers_map);
        let key = util::random_string(g, len);
        let mtime: u64 = g.gen();
        let creator = util::random_string(g, len);
        let dirname = util::random_string(g, len);
        let name = util::random_string(g, len);
        let owner = Uuid::new_v4().to_string();
        let roles: Vec<String> = vec![util::random_string(g, len)];
        let vnode: u64 = g.gen();
        let content_length: u64 = g.gen();

        let md5_sum =
            format!("{:x}", md5::compute(util::random_string(g, len)));
        let content_md5: String = base64::encode(&md5_sum);

        let etag: String = Uuid::new_v4().to_string();
        let content_type: String = util::random_string(g, len);
        let object_id: String = Uuid::new_v4().to_string();
        let sharks: Vec<MantaObjectShark> = vec![
            MantaObjectShark::arbitrary(g),
            MantaObjectShark::arbitrary(g),
        ];

        MantaObject {
            headers,
            key,
            mtime,
            name,
            dirname,
            creator,
            owner,
            roles,
            vnode,
            content_length,
            content_md5,
            content_type,
            object_id,
            etag,
            sharks,
        }
    }
}
