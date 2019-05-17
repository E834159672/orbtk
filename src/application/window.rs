use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::Entity;

use crate::prelude::*;
use crate::backend::*;

use crate::backend::platform;

/// Represents a window. Each window has its own tree, event pipeline and backend.
pub struct WindowAdapter {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    // pub update: Rc<Cell<bool>>,
    // pub running: Rc<Cell<bool>>,
}

impl platform::WindowAdapter for WindowAdapter {
    fn update(&mut self) {

    }
}

impl Into<Box<platform::WindowAdapter>> for WindowAdapter {
    fn into(self) -> Box<platform::WindowAdapter> {
        Box::new(self)
    }
}