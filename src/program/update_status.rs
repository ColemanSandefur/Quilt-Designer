use std::sync::{Arc, Mutex, Weak};
use std::ops::Deref;

pub type Update = Arc<Mutex<bool>>;

//
// UpdateStatus
//
// Used as a helper for chaining update flags
// Whenever it is set to update, it will set the parent to true also
// Setting it to false, it will not set its children to false (obviously since it doesn't have a ref anyway)
// This was initially used to help a quilt block tell the quilt that the quilt needs to update its buffers/update the renderer
//

#[derive(Clone)]
pub struct UpdateStatus {
    update: Update,
    parent: Option<WeakUpdateStatus>,
}

impl UpdateStatus {
    pub fn new() -> Self {
        Self {
            update: Arc::new(Mutex::new(false)),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: WeakUpdateStatus) -> Self {
        Self {
            update: Arc::new(Mutex::new(false)),
            parent: Some(parent),
        }
    }

    pub fn needs_updated(&self) {
        *self.update.lock().unwrap() = true;
        
        if let Some(parent) = &self.parent {
            if let Some(parent) = Weak::upgrade(&parent) {
                parent.lock().unwrap().needs_updated();
            }
        }
    }

    pub fn reset_updated(&self) {
        *self.update.lock().unwrap() = false;
    }

    pub fn get_needs_updated(&self) -> bool {
        *self.update.lock().unwrap()
    }
}

#[derive(Clone)]
pub struct SyncUpdateStatus {
    update_status: Arc<Mutex<UpdateStatus>>
}

impl Deref for SyncUpdateStatus {
    type Target = Arc<Mutex<UpdateStatus>>;

    fn deref(&self) -> &Self::Target {
        &self.update_status
    }
}

impl SyncUpdateStatus {
    pub fn new() -> Self {
        Self {
            update_status: Arc::new(Mutex::new(UpdateStatus::new())),
        }
    }
    
    pub fn new_with_parent(parent: WeakUpdateStatus) -> Self {
        Self {
            update_status: Arc::new(Mutex::new(UpdateStatus::new_with_parent(parent))),
        }
    }
    
    pub fn needs_updated(&self) {
        self.lock().unwrap().needs_updated();
    }
    
    pub fn reset_updated(&self) {
        self.lock().unwrap().reset_updated();
    }
    
    pub fn get_needs_updated(&self) -> bool {
        self.lock().unwrap().get_needs_updated()
    }

    pub fn weak(&self) -> WeakUpdateStatus {
        WeakUpdateStatus{ update_status: Arc::downgrade(&self.update_status) }
    }
}

#[derive(Clone)]
pub struct WeakUpdateStatus {
    update_status: Weak<Mutex<UpdateStatus>>
}

impl Deref for WeakUpdateStatus {
    type Target = Weak<Mutex<UpdateStatus>>;

    fn deref(&self) -> &Self::Target {
        &self.update_status
    }
}

impl WeakUpdateStatus {
    pub fn upgrade(&self) -> Option<SyncUpdateStatus> {
        let upgraded = Weak::upgrade(&self.update_status);

        if let Some(upgraded) = upgraded {

            return Some(SyncUpdateStatus {
                update_status: upgraded
            });
        }

        None
    }
}