use crate::prelude::*;

pub fn update_resource(mut resource: ResMut<ScopedSystemValue>) {
    *resource = ScopedSystemValue(resource.0 + 1);
    info!("Update resource : {resource:?}")
}

pub fn systems() -> ServiceSystems {
    ServiceSystems::new(update_resource)
}
