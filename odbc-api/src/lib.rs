//! # About
//!
//! `odbc-api` enables you to write applications which utilize ODBC (Open Database Connectivity)
//! standard to access databases. See the [`guide`] for more information and code
//! examples.

mod columnar_bulk_inserter;
mod connection;
mod cursor;
mod driver_complete_option;
mod environment;
mod error;
mod execute;
mod fixed_sized;
mod into_parameter;
mod nullable;
mod parameter_collection;
mod preallocated;
mod prepared;
mod result_set_metadata;
mod sleep;
mod statement_connection;

pub mod buffers;
pub mod guide;
pub mod handles;
pub mod parameter;

pub use self::{
    columnar_bulk_inserter::{BoundInputSlice, ColumnarBulkInserter},
    connection::{escape_attribute_value, Connection},
    cursor::{
        BlockCursor, BlockCursorPolling, Cursor, CursorImpl, CursorPolling, CursorRow, RowSetBuffer,
    },
    driver_complete_option::DriverCompleteOption,
    environment::{DataSourceInfo, DriverInfo, Environment},
    error::{Error, TooLargeBufferSize},
    fixed_sized::Bit,
    handles::{ColumnDescription, DataType, Nullability},
    into_parameter::IntoParameter,
    nullable::Nullable,
    parameter::{InOut, Out, OutputParameter},
    parameter_collection::{ParameterCollection, ParameterCollectionRef, ParameterTupleElement},
    preallocated::{Preallocated, PreallocatedPolling},
    prepared::Prepared,
    result_set_metadata::ResultSetMetadata,
    sleep::Sleep,
    statement_connection::StatementConnection,
};
// Reexports
pub use force_send_sync;
/// Reexports `odbc-sys` as sys to enable applications to always use the same version as this
/// crate.
pub use odbc_sys as sys;
pub use widestring::{U16Str, U16String};

#[allow(deprecated)]
pub use crate::cursor::{RowSetCursor, RowSetCursorPolling};
