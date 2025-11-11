use crate::{prelude::*, test_impl::service::scoped_system::data::ScopedSystemValue};

pub fn update_resource(mut resource: ResMut<ScopedSystemValue>) {
    *resource = ScopedSystemValue(resource.0 + 1);
    info!("Update resource : {resource:?}")
}

pub fn systems() -> ServiceSystems {
    ServiceSystems::new(update_resource)
}
