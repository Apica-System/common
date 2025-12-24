use std::collections::HashMap;
use crate::element::Element;

pub struct Context {
    scopes: Vec<HashMap<String, Element>>,
}

impl Context {
    pub fn init() -> Context {
        return Context { scopes: vec![HashMap::new()] };
    }

    pub fn get_element(&self, name: &str, is_global: bool) -> Option<&Element> {
        if is_global {
            if let Some(scope) = self.scopes.first() {
                return scope.get(name);
            }
        } else {
            for scope in self.scopes.iter().rev() {
                if let Some(element) = scope.get(name) {
                    return Some(element);
                }
            }
        }

        return None;
    }

    pub fn get_element_mut(&mut self, name: &str, is_global: bool) -> Option<&mut Element> {
        if is_global {
            if let Some(scope) = self.scopes.first_mut() {
                return scope.get_mut(name);
            }
        } else {
            for scope in self.scopes.iter_mut().rev() {
                if let Some(element) = scope.get_mut(name) {
                    return Some(element);
                }
            }
        }

        return None;
    }

    pub fn set_element(&mut self, name: String, element: Element, is_global: bool) -> bool {
        let scope = if is_global {
            self.scopes.first_mut().unwrap()
        } else {
            self.scopes.last_mut().unwrap()
        };

        return scope.insert(name, element).is_none();
    }

    pub fn push_scope(&mut self) { 
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
}