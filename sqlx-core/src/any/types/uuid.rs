use crate::any::{Any, AnyTypeInfo, AnyTypeInfoKind, AnyValueKind};
use crate::database::Database;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use std::sync::Arc;
use uuid::{
    fmt::{Hyphenated, Simple},
    Uuid,
};

impl Type<Any> for Uuid {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Uuid,
        }
    }

    fn compatible(ty: &AnyTypeInfo) -> bool {
        matches!(ty.kind, AnyTypeInfoKind::Uuid | AnyTypeInfoKind::Blob | AnyTypeInfoKind::Text)
    }
}

impl Encode<'_, Any> for Uuid {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        buf.0.push(AnyValueKind::Uuid(*self.as_bytes()));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for Uuid {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Uuid(bytes) => Ok(Uuid::from_bytes(*bytes)),
            AnyValueKind::Blob(bytes) => {
                if bytes.len() != 16 {
                    return Err(format!(
                        "expected 16 bytes for UUID, got {}",
                        bytes.len()
                    )
                    .into());
                }
                Uuid::from_slice(bytes).map_err(Into::into)
            }
            AnyValueKind::Text(text) => Uuid::parse_str(text).map_err(Into::into),
            other => other.unexpected(),
        }
    }
}

impl Type<Any> for Hyphenated {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Text,
        }
    }

    fn compatible(ty: &AnyTypeInfo) -> bool {
        matches!(ty.kind, AnyTypeInfoKind::Text | AnyTypeInfoKind::Blob)
    }
}

impl Encode<'_, Any> for Hyphenated {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        buf.0
            .push(AnyValueKind::Text(Arc::new(self.to_string())));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for Hyphenated {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Text(text) => Uuid::parse_str(text)
                .map(|u| u.hyphenated())
                .map_err(Into::into),
            AnyValueKind::Blob(bytes) => {
                if bytes.len() != 16 {
                    return Err(format!(
                        "expected 16 bytes for UUID, got {}",
                        bytes.len()
                    )
                    .into());
                }
                Uuid::from_slice(bytes)
                    .map(|u| u.hyphenated())
                    .map_err(Into::into)
            }
            other => other.unexpected(),
        }
    }
}

impl Type<Any> for Simple {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Text,
        }
    }

    fn compatible(ty: &AnyTypeInfo) -> bool {
        matches!(ty.kind, AnyTypeInfoKind::Text | AnyTypeInfoKind::Blob)
    }
}

impl Encode<'_, Any> for Simple {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        buf.0
            .push(AnyValueKind::Text(Arc::new(self.to_string())));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for Simple {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Text(text) => Uuid::parse_str(text)
                .map(|u| u.simple())
                .map_err(Into::into),
            AnyValueKind::Blob(bytes) => {
                if bytes.len() != 16 {
                    return Err(format!(
                        "expected 16 bytes for UUID, got {}",
                        bytes.len()
                    )
                    .into());
                }
                Uuid::from_slice(bytes)
                    .map(|u| u.simple())
                    .map_err(Into::into)
            }
            other => other.unexpected(),
        }
    }
}
