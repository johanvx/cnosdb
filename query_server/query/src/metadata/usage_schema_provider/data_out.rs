use std::sync::Arc;

use coordinator::service::CoordinatorRef;
use datafusion::datasource::TableProvider;
use meta::MetaClientRef;
use models::auth::user::User;

use crate::metadata::usage_schema_provider::{
    create_usage_schema_view_table, UsageSchemaTableFactory,
};

pub const USAGE_SCHEMA_DATA_OUT: &str = "data_out";

pub struct DataOut {}

impl UsageSchemaTableFactory for DataOut {
    fn table_name(&self) -> &str {
        USAGE_SCHEMA_DATA_OUT
    }
    fn create(
        &self,
        user: &User,
        coord: CoordinatorRef,
        meta: MetaClientRef,
        default_catalog: MetaClientRef,
    ) -> spi::Result<Arc<dyn TableProvider>> {
        create_usage_schema_view_table(user, coord, meta, USAGE_SCHEMA_DATA_OUT, default_catalog)
    }
}
