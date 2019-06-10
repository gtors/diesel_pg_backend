use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::io::prelude::*;

use diesel::deserialize::{self, FromSql};
use crate::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::{Integer, SmallInt, BigInt};
use crate::sql_types::{Oid};

impl FromSql<Oid, Pg> for u32 {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let mut bytes = not_none!(bytes);
        bytes.read_u32::<NetworkEndian>().map_err(Into::into)
    }
}

impl ToSql<Oid, Pg> for u32 {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_u32::<NetworkEndian>(*self)
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}

#[test]
fn i16_to_sql() {
    let mut bytes = Output::test();
    ToSql::<SmallInt, Pg>::to_sql(&1i16, &mut bytes).unwrap();
    ToSql::<SmallInt, Pg>::to_sql(&0i16, &mut bytes).unwrap();
    ToSql::<SmallInt, Pg>::to_sql(&-1i16, &mut bytes).unwrap();
    assert_eq!(bytes, vec![0, 1, 0, 0, 255, 255]);
}

#[test]
fn i32_to_sql() {
    let mut bytes = Output::test();
    ToSql::<Integer, Pg>::to_sql(&1i32, &mut bytes).unwrap();
    ToSql::<Integer, Pg>::to_sql(&0i32, &mut bytes).unwrap();
    ToSql::<Integer, Pg>::to_sql(&-1i32, &mut bytes).unwrap();
    assert_eq!(bytes, vec![0, 0, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255]);
}

#[test]
fn i64_to_sql() {
    let mut bytes = Output::test();
    ToSql::<BigInt, Pg>::to_sql(&1i64, &mut bytes).unwrap();
    ToSql::<BigInt, Pg>::to_sql(&0i64, &mut bytes).unwrap();
    ToSql::<BigInt, Pg>::to_sql(&-1i64, &mut bytes).unwrap();
    assert_eq!(
        bytes,
        vec![
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
        ]
    );
}
