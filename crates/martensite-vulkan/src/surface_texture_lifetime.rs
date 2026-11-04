//! Weak texture owners permit cache reclamation without extending application handle lifetimes.
use martensite_tkernel::standard::{Handle, WeakHandle};
use martensite_tkservice::graphic3d::{Graphic3dTextureEnv, Graphic3dTextureMap};
use std::sync::Mutex;
#[derive(Debug, Default)]
pub(crate) struct SurfaceTextureLifetime {
    owners: Mutex<Vec<WeakHandle<dyn Graphic3dTextureMap>>>,
    /// Views' environment textures (`Graphic3dTextureEnv` is a texture root, not a map).
    environment_owners: Mutex<Vec<WeakHandle<Graphic3dTextureEnv>>>,
}
impl PartialEq for SurfaceTextureLifetime {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl SurfaceTextureLifetime {
    /// Keep a view's environment texture as an owner; `lifetime.register_environment(&env)`.
    pub fn register_environment(&self, texture: &Handle<Graphic3dTextureEnv>) {
        let owner = texture.downgrade();
        let mut owners = self
            .environment_owners
            .lock()
            .expect("unpoisoned environment texture owners");
        owners.retain(|old| !old.upgrade().is_null());
        if !owners.contains(&owner) {
            owners.push(owner);
        }
    }
    pub fn register(&self, texture: &Handle<dyn Graphic3dTextureMap>) {
        let owner = texture.downgrade();
        let mut owners = self.owners.lock().expect("unpoisoned texture owners");
        owners.retain(|old| !old.upgrade().is_null());
        if !owners.contains(&owner) {
            owners.push(owner);
        }
    }
    pub fn is_alive(&self) -> bool {
        self.owners
            .lock()
            .expect("unpoisoned texture owners")
            .iter()
            .any(|owner| !owner.upgrade().is_null())
            || self
                .environment_owners
                .lock()
                .expect("unpoisoned environment texture owners")
                .iter()
                .any(|owner| !owner.upgrade().is_null())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkservice::graphic3d::Graphic3dTexture2D;
    #[test]
    fn texture_cache_lifetime_tracks_multiple_weak_owners_without_retaining_handles() {
        let first = Handle::new(Graphic3dTexture2D::from_pix_map(Handle::default())).cast();
        let second = Handle::new(Graphic3dTexture2D::from_pix_map(Handle::default())).cast();
        let lifetime = SurfaceTextureLifetime::default();
        assert!(!lifetime.is_alive());
        lifetime.register(&first);
        lifetime.register(&first);
        lifetime.register(&second);
        assert_eq!(lifetime.owners.lock().unwrap().len(), 2);
        drop(first);
        assert!(lifetime.is_alive());
        drop(second);
        assert!(!lifetime.is_alive());
    }
}
