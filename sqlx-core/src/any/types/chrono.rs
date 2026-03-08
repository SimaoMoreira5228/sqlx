use crate::any::{Any, AnyTypeInfo, AnyTypeInfoKind, AnyValueKind};
use crate::database::Database;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use std::sync::Arc;

impl Type<Any> for NaiveDate {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Text,
        }
    }
}

impl Encode<'_, Any> for NaiveDate {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        buf.0
            .push(AnyValueKind::Text(Arc::new(self.to_string())));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for NaiveDate {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Text(text) => text.parse().map_err(Into::into),
            other => other.unexpected(),
        }
    }
}

impl Type<Any> for NaiveTime {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Text,
        }
    }
}

impl Encode<'_, Any> for NaiveTime {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        buf.0
            .push(AnyValueKind::Text(Arc::new(self.to_string())));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for NaiveTime {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Text(text) => text.parse().map_err(Into::into),
            other => other.unexpected(),
        }
    }
}

impl Type<Any> for NaiveDateTime {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Text,
        }
    }
}

impl Encode<'_, Any> for NaiveDateTime {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        buf.0
            .push(AnyValueKind::Text(Arc::new(self.to_string())));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for NaiveDateTime {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Text(text) => text.parse().map_err(Into::into),
            other => other.unexpected(),
        }
    }
}

impl Type<Any> for DateTime<Utc> {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Text,
        }
    }
}

impl Encode<'_, Any> for DateTime<Utc> {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        buf.0
            .push(AnyValueKind::Text(Arc::new(self.to_rfc3339())));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for DateTime<Utc> {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Text(text) => {
                if let Ok(dt) = DateTime::parse_from_rfc3339(text) {
                    Ok(dt.with_timezone(&Utc))
                } else {
                    let ndt: NaiveDateTime = text.parse()?;
                    Ok(DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc))
                }
            }
            other => other.unexpected(),
        }
    }
}

impl Type<Any> for DateTime<Local> {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Text,
        }
    }
}

impl Encode<'_, Any> for DateTime<Local> {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        buf.0
            .push(AnyValueKind::Text(Arc::new(self.to_rfc3339())));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for DateTime<Local> {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Text(text) => {
                if let Ok(dt) = DateTime::parse_from_rfc3339(text) {
                    Ok(dt.with_timezone(&Local))
                } else {
                    let ndt: NaiveDateTime = text.parse()?;
                    Ok(Local.from_utc_datetime(&ndt))
                }
            }
            other => other.unexpected(),
        }
    }
}

impl Type<Any> for DateTime<FixedOffset> {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Text,
        }
    }
}

impl Encode<'_, Any> for DateTime<FixedOffset> {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        buf.0
            .push(AnyValueKind::Text(Arc::new(self.to_rfc3339())));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for DateTime<FixedOffset> {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Text(text) => {
                DateTime::parse_from_rfc3339(text).map_err(Into::into)
            }
            other => other.unexpected(),
        }
    }
}
