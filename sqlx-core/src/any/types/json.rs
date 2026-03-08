use crate::any::{Any, AnyTypeInfo, AnyTypeInfoKind, AnyValueKind};
use crate::database::Database;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use serde::{Deserialize, Serialize};
use crate::types::Json;
use std::sync::Arc;

impl<T> Type<Any> for Json<T> {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Json,
        }
    }

    fn compatible(ty: &AnyTypeInfo) -> bool {
        matches!(ty.kind, AnyTypeInfoKind::Json | AnyTypeInfoKind::Text | AnyTypeInfoKind::Blob)
    }
}

impl<T> Encode<'_, Any> for Json<T>
where
    T: Serialize,
{
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        let json_str = serde_json::to_string(&self.0)?;
        buf.0.push(AnyValueKind::Json(Arc::new(json_str)));
        Ok(IsNull::No)
    }
}

impl<'r, T> Decode<'r, Any> for Json<T>
where
    T: 'r + Deserialize<'r>,
{
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Json(text) | AnyValueKind::Text(text) => {
                serde_json::from_str(text).map(Json).map_err(Into::into)
            }
            AnyValueKind::Blob(ref bytes) => {
                let json_bytes = if bytes.first() == Some(&1) {
                    &bytes[1..]
                } else {
                    bytes.as_ref()
                };
                serde_json::from_slice(json_bytes).map(Json).map_err(Into::into)
            }
            other => other.unexpected(),
        }
    }
}
