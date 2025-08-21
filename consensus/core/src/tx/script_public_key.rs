use alloc::borrow::Cow;
use borsh::{BorshDeserialize, BorshSerialize};
use js_sys::Object;
use bascoin_utils::{hex::{FromHex, ToHex}, serde_bytes};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    str::{self, FromStr},
};
use wasm_bindgen::prelude::*;
use workflow_wasm::prelude::*;

/// Size of the underlying script vector of a script.
pub const SCRIPT_VECTOR_SIZE: usize = 34;

/// Used as the underlying type for script public key data, optimized for the common p2pk script size (34).
pub type ScriptVec = Vec<u8>;

/// Represents the ScriptPublicKey Version
pub type ScriptPublicKeyVersion = u16;

/// Alias the `smallvec!` macro to ease maintenance
pub use smallvec::smallvec as scriptvec;
use wasm_bindgen::prelude::wasm_bindgen;

//Represents a Set of [`ScriptPublicKey`]s
pub type ScriptPublicKeys = HashSet<ScriptPublicKey>;

#[wasm_bindgen(typescript_custom_section)]
const TS_SCRIPT_PUBLIC_KEY: &'static str = r#"
/**
 * Interface defines the structure of a Script Public Key.
 * 
 * @category Consensus
 */
export interface IScriptPublicKey {
    version : number;
    script: HexString;
}
"#;

/// Represents a Bascoin ScriptPublicKey
/// @category Consensus
#[derive(Default, PartialEq, Eq, Clone, Hash, CastFromJs)]
#[wasm_bindgen(inspectable)]
pub struct ScriptPublicKey {
    pub version: ScriptPublicKeyVersion,
    pub(super) script: ScriptVec, // Kept private to preserve read-only semantics
}

impl std::fmt::Debug for ScriptPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScriptPublicKey").field("version", &self.version).field("script", &self.script.to_hex()).finish()
    }
}

impl FromHex for ScriptPublicKey {
    type Error = faster_hex::Error;
    fn from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        ScriptPublicKey::from_str(hex_str)
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "ScriptPublicKey")]
pub struct ScriptPublicKeyInternal {
    version: ScriptPublicKeyVersion,
    #[serde(with = "serde_bytes")]
    script: Vec<u8>,
}

impl FromStr for ScriptPublicKey {
    type Err = faster_hex::Error;
    fn from_str(hex_str: &str) -> Result<Self, Self::Err> {
        let hex_len = hex_str.len();
        if hex_len < 4 {
            return Err(faster_hex::Error::InvalidLength(hex_len));
        }
        let mut bytes = vec![0u8; hex_len / 2];
        faster_hex::hex_decode(hex_str.as_bytes(), bytes.as_mut_slice())?;
        let version = u16::from_be_bytes(bytes[0..2].try_into().unwrap());
        Ok(Self { version, script: bytes[2..].to_vec() })
    }
}

impl Serialize for ScriptPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // We serialize as a hex string when human readable or as bytes otherwise.
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.script.as_slice().to_hex())
        } else {
            serializer.serialize_bytes(self.script.as_slice())
        }
    }
}

impl<'de> Deserialize<'de> for ScriptPublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Default)]
        pub struct ScriptPublicKeyVisitor<'de> {
            from_hex_visitor: serde_bytes::FromHexVisitor<'de, ScriptPublicKey>,
            marker: std::marker::PhantomData<ScriptPublicKey>,
            lifetime: std::marker::PhantomData<&'de ()>,
        }

        impl<'de> serde::de::Visitor<'de> for ScriptPublicKeyVisitor<'de> {
            type Value = ScriptPublicKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> core::fmt::Result {
                #[cfg(target_arch = "wasm32")]
                {
                    write!(formatter, "string-type: string, str; bytes-type: slice of bytes, vec of bytes; map; number-type - pointer")
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    write!(formatter, "string-type: string, str; bytes-type: slice of bytes, vec of bytes; map")
                }
            }

            // TODO - review integer conversions (SPK & Address)
            // This is currently used to allow for deserialization
            // of JsValues created via wasm-bindgen bindings in the
            // WASM context. This is not used in the native context
            // as serialization will never produce objects.
            // - review multiple integer mappings (are they all needed?)
            // - consider manual marshaling of RPC data structures
            // (which is now possible due to the introduction of the bascoin-consensus-wasm crate)
            #[cfg(target_arch = "wasm32")]
            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u32(v as u32)
            }
            #[cfg(target_arch = "wasm32")]
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u32(v as u32)
            }

            #[cfg(target_arch = "wasm32")]
            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u32(v as u32)
            }
            #[cfg(target_arch = "wasm32")]
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u32(v as u32)
            }
            #[cfg(target_arch = "wasm32")]
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use wasm_bindgen::convert::RefFromWasmAbi;
                let instance_ref = unsafe { Self::Value::ref_from_abi(v) }; // todo add checks for safecast
                Ok(instance_ref.clone())
            }
            #[cfg(target_arch = "wasm32")]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u32(v as u32)
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.from_hex_visitor.visit_str(v)
            }

            #[inline]
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.from_hex_visitor.visit_borrowed_str(v)
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.from_hex_visitor.visit_string(v)
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.from_hex_visitor.visit_bytes(v)
            }

            #[inline]
            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.from_hex_visitor.visit_borrowed_bytes(v)
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.from_hex_visitor.visit_byte_buf(v)
            }

            #[inline]
            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                #[derive(Deserialize, Copy, Clone)]
                #[serde(field_identifier, rename_all = "lowercase")]
                enum Field {
                    Version,
                    Script,
                }

                #[derive(Debug, Clone, Deserialize)]
                #[serde(untagged)]
                pub enum Value<'a> {
                    U16(u16),
                    #[serde(borrow)]
                    String(Cow<'a, str>),
                }
                impl From<Value<'_>> for u16 {
                    fn from(value: Value<'_>) -> Self {
                        let Value::U16(v) = value else { panic!("unexpected conversion: {value:?}") };
                        v
                    }
                }

                impl TryFrom<Value<'_>> for Vec<u8> {
                    type Error = faster_hex::Error;

                    fn try_from(value: Value) -> Result<Self, Self::Error> {
                        match value {
                            Value::U16(_) => {
                                panic!("unexpected conversion: {value:?}")
                            }
                            Value::String(script) => {
                                let mut script_bytes = vec![0u8; script.len() / 2];
                                faster_hex::hex_decode(script.as_bytes(), script_bytes.as_mut_slice())?;

                                Ok(script_bytes)
                            }
                        }
                    }
                }

                let mut version: Option<u16> = None;
                let mut script: Option<Vec<u8>> = None;

                while let Some((key, value)) = access.next_entry::<Field, Value>()? {
                    match key {
                        Field::Version => {
                            version = Some(value.into());
                        }
                        Field::Script => script = Some(value.try_into().map_err(serde::de::Error::custom)?),
                    }
                    if version.is_some() && script.is_some() {
                        break;
                    }
                }
                let (version, script) = match (version, script) {
                    (Some(version), Some(script)) => Ok((version, script)),
                    (None, _) => Err(serde::de::Error::missing_field("version")),
                    (_, None) => Err(serde::de::Error::missing_field("script")),
                }?;

                Ok(ScriptPublicKey::from_vec(version, script))
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_any(ScriptPublicKeyVisitor::default())
        } else {
            ScriptPublicKeyInternal::deserialize(deserializer)
                .map(|ScriptPublicKeyInternal { script, version }| Self { version, script })
        }
    }
}

impl ScriptPublicKey {
    pub fn new(version: ScriptPublicKeyVersion, script: ScriptVec) -> Self {
        Self { version, script }
    }

    pub fn from_vec(version: ScriptPublicKeyVersion, script: Vec<u8>) -> Self {
        Self { version, script }
    }

    pub fn version(&self) -> ScriptPublicKeyVersion {
        self.version
    }

    pub fn script(&self) -> &[u8] {
        &self.script.as_slice()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ScriptPublicKey | HexString")]
    pub type ScriptPublicKeyT;
}

#[wasm_bindgen]
impl ScriptPublicKey {
    #[wasm_bindgen(constructor)]
    pub fn constructor(version: u16, script: JsValue) -> Result<ScriptPublicKey, JsError> {
        let script = script.try_as_vec_u8()?;
        Ok(ScriptPublicKey::new(version, script.into()))
    }

    #[wasm_bindgen(getter = script)]
    pub fn script_as_hex(&self) -> String {
        self.script.as_slice().to_hex()
    }
}

//
// Borsh serializers need to be manually implemented for `ScriptPublicKey` since
// smallvec does not currently support Borsh
//

impl BorshSerialize for ScriptPublicKey {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.version, writer)?;
        // Vectors and slices are all serialized internally the same way
        borsh::BorshSerialize::serialize(&self.script.as_slice(), writer)?;
        Ok(())
    }
}

impl BorshDeserialize for ScriptPublicKey {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        // Deserialize into vec first since we have no custom smallvec support
        Ok(Self::from_vec(borsh::BorshDeserialize::deserialize_reader(reader)?, borsh::BorshDeserialize::deserialize_reader(reader)?))
    }
}

type CastError = workflow_wasm::error::Error;
impl TryCastFromJs for ScriptPublicKey {
    type Error = workflow_wasm::error::Error;
    fn try_cast_from<'a, R>(value: &'a R) -> Result<Cast<'a, Self>, Self::Error>
    where
        R: AsRef<JsValue> + 'a,
    {
        Self::resolve(value, || {
            if let Some(hex_str) = value.as_ref().as_string() {
                Ok(Self::from_str(&hex_str).map_err(CastError::custom)?)
            } else if let Some(object) = Object::try_from(value.as_ref()) {
                let version = object.try_get_value("version")?.ok_or(CastError::custom(
                    "ScriptPublicKey must be a hex string or an object with 'version' and 'script' properties",
                ))?;

                let version = if let Ok(version) = version.try_as_u16() {
                    version
                } else {
                    return Err(CastError::custom(format!("Invalid version value '{version:?}'")));
                };

                let script = object.get_vec_u8("script")?;
                Ok(ScriptPublicKey::from_vec(version, script))
            } else {
                Err(CastError::custom(format!("Unable to convert ScriptPublicKey from: {:?}", value.as_ref())))
            }
        })
    }
}
