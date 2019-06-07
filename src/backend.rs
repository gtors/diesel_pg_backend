//! The PostgreSQL backend

use std::convert::From;
use diesel::backend::*;
use diesel::byteorder::NetworkEndian;
use diesel::deserialize::Queryable;
use diesel::query_builder::bind_collector::RawBytesBindCollector;
use diesel::sql_types::{Oid, Bool, TypeMetadata};

use super::query_builder::PgQueryBuilder;
use super::PgMetadataLookup;

/// The PostgreSQL backend
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pg;

/// Format of the type
pub enum PgTypeFormat {
    Binary,
    Text,
}

impl From<bool> for PgTypeFormat {
    fn from(is_binary: bool) -> PgTypeFormat {
        match (is_binary) {
            true => PgTypeFormat::Binary,
            false => PgTypeFormat::Text,
        }
    }
}

/// The [OIDs] for a SQL type
///
/// [OIDs]: https://www.postgresql.org/docs/current/static/datatype-oid.html
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct PgTypeMetadata {
    /// The [OID] of `T`
    ///
    /// [OID]: https://www.postgresql.org/docs/current/static/datatype-oid.html
    pub oid: u32,
    /// The [OID] of `T[]`
    ///
    /// [OID]: https://www.postgresql.org/docs/current/static/datatype-oid.html
    pub array_oid: u32,
    pub recv_format: PgTypeFormat,
    pub send_format: PgTypeFormat,
}

impl Queryable<(Oid, Oid, Bool, Bool), Pg> for PgTypeMetadata {
    type Row = (u32, u32, bool, bool);

    fn build((oid, array_oid, recv_binary, send_binary): Self::Row) -> Self {
        PgTypeMetadata {
            oid: oid,
            array_oid: array_oid,
            recv_format: recv_binary::into(),
            send_format: send_binary::into(),
        }
    }

impl Backend for Pg {
    type QueryBuilder = PgQueryBuilder;
    type BindCollector = RawBytesBindCollector<Pg>;
    type ByteOrder = NetworkEndian;
}

impl<'a> HasRawValue<'a> for Pg {
    type RawValue = &'a [u8];
}

impl TypeMetadata for Pg {
    type TypeMetadata = PgTypeMetadata;
    type MetadataLookup = PgMetadataLookup;
}

impl SupportsReturningClause for Pg {}
impl SupportsDefaultKeyword for Pg {}
impl UsesAnsiSavepointSyntax for Pg {}
