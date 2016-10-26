// This file was generated by gir (d9591be+) from gir-files (???)
// DO NOT EDIT

use DOMDocument;
use DOMElement;
use DOMEventTarget;
use DOMNodeList;
use DOMObject;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct DOMNode(Object<ffi::WebKitDOMNode>): DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_node_get_type(),
    }
}

pub trait DOMNodeExt {
    fn append_child<T: IsA<DOMNode>>(&self, newChild: &T) -> Result<DOMNode, Error>;

    fn clone_node(&self, deep: bool) -> Option<DOMNode>;

    fn compare_document_position<T: IsA<DOMNode>>(&self, other: &T) -> u16;

    fn contains<T: IsA<DOMNode>>(&self, other: &T) -> bool;

    fn get_base_uri(&self) -> Option<String>;

    fn get_child_nodes(&self) -> Option<DOMNodeList>;

    fn get_first_child(&self) -> Option<DOMNode>;

    fn get_last_child(&self) -> Option<DOMNode>;

    fn get_local_name(&self) -> Option<String>;

    fn get_namespace_uri(&self) -> Option<String>;

    fn get_next_sibling(&self) -> Option<DOMNode>;

    fn get_node_name(&self) -> Option<String>;

    fn get_node_type(&self) -> u16;

    fn get_node_value(&self) -> Option<String>;

    fn get_owner_document(&self) -> Option<DOMDocument>;

    fn get_parent_element(&self) -> Option<DOMElement>;

    fn get_parent_node(&self) -> Option<DOMNode>;

    fn get_prefix(&self) -> Option<String>;

    fn get_previous_sibling(&self) -> Option<DOMNode>;

    fn get_text_content(&self) -> Option<String>;

    fn has_child_nodes(&self) -> bool;

    fn insert_before<T: IsA<DOMNode>, U: IsA<DOMNode>>(&self, newChild: &T, refChild: Option<&U>) -> Result<DOMNode, Error>;

    fn is_default_namespace(&self, namespaceURI: &str) -> bool;

    fn is_equal_node<T: IsA<DOMNode>>(&self, other: &T) -> bool;

    fn is_same_node<T: IsA<DOMNode>>(&self, other: &T) -> bool;

    fn is_supported(&self, feature: &str, version: &str) -> bool;

    fn lookup_namespace_uri(&self, prefix: &str) -> Option<String>;

    fn lookup_prefix(&self, namespaceURI: &str) -> Option<String>;

    fn normalize(&self);

    fn remove_child<T: IsA<DOMNode>>(&self, oldChild: &T) -> Result<DOMNode, Error>;

    fn replace_child<T: IsA<DOMNode>, U: IsA<DOMNode>>(&self, newChild: &T, oldChild: &U) -> Result<DOMNode, Error>;

    fn set_node_value(&self, value: &str) -> Result<(), Error>;

    fn set_prefix(&self, value: &str) -> Result<(), Error>;

    fn set_text_content(&self, value: &str) -> Result<(), Error>;
}

impl<O: IsA<DOMNode>> DOMNodeExt for O {
    fn append_child<T: IsA<DOMNode>>(&self, newChild: &T) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_append_child(self.to_glib_none().0, newChild.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn clone_node(&self, deep: bool) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_clone_node(self.to_glib_none().0, deep.to_glib()))
        }
    }

    fn compare_document_position<T: IsA<DOMNode>>(&self, other: &T) -> u16 {
        unsafe {
            ffi::webkit_dom_node_compare_document_position(self.to_glib_none().0, other.to_glib_none().0)
        }
    }

    fn contains<T: IsA<DOMNode>>(&self, other: &T) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_contains(self.to_glib_none().0, other.to_glib_none().0))
        }
    }

    fn get_base_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_base_uri(self.to_glib_none().0))
        }
    }

    fn get_child_nodes(&self) -> Option<DOMNodeList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_child_nodes(self.to_glib_none().0))
        }
    }

    fn get_first_child(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_first_child(self.to_glib_none().0))
        }
    }

    fn get_last_child(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_last_child(self.to_glib_none().0))
        }
    }

    fn get_local_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_local_name(self.to_glib_none().0))
        }
    }

    fn get_namespace_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_namespace_uri(self.to_glib_none().0))
        }
    }

    fn get_next_sibling(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_next_sibling(self.to_glib_none().0))
        }
    }

    fn get_node_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_node_name(self.to_glib_none().0))
        }
    }

    fn get_node_type(&self) -> u16 {
        unsafe {
            ffi::webkit_dom_node_get_node_type(self.to_glib_none().0)
        }
    }

    fn get_node_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_node_value(self.to_glib_none().0))
        }
    }

    fn get_owner_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_owner_document(self.to_glib_none().0))
        }
    }

    fn get_parent_element(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_parent_element(self.to_glib_none().0))
        }
    }

    fn get_parent_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_parent_node(self.to_glib_none().0))
        }
    }

    fn get_prefix(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_prefix(self.to_glib_none().0))
        }
    }

    fn get_previous_sibling(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_previous_sibling(self.to_glib_none().0))
        }
    }

    fn get_text_content(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_text_content(self.to_glib_none().0))
        }
    }

    fn has_child_nodes(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_has_child_nodes(self.to_glib_none().0))
        }
    }

    fn insert_before<T: IsA<DOMNode>, U: IsA<DOMNode>>(&self, newChild: &T, refChild: Option<&U>) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_insert_before(self.to_glib_none().0, newChild.to_glib_none().0, refChild.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_default_namespace(&self, namespaceURI: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_default_namespace(self.to_glib_none().0, namespaceURI.to_glib_none().0))
        }
    }

    fn is_equal_node<T: IsA<DOMNode>>(&self, other: &T) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_equal_node(self.to_glib_none().0, other.to_glib_none().0))
        }
    }

    fn is_same_node<T: IsA<DOMNode>>(&self, other: &T) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_same_node(self.to_glib_none().0, other.to_glib_none().0))
        }
    }

    fn is_supported(&self, feature: &str, version: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_supported(self.to_glib_none().0, feature.to_glib_none().0, version.to_glib_none().0))
        }
    }

    fn lookup_namespace_uri(&self, prefix: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_lookup_namespace_uri(self.to_glib_none().0, prefix.to_glib_none().0))
        }
    }

    fn lookup_prefix(&self, namespaceURI: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_lookup_prefix(self.to_glib_none().0, namespaceURI.to_glib_none().0))
        }
    }

    fn normalize(&self) {
        unsafe {
            ffi::webkit_dom_node_normalize(self.to_glib_none().0);
        }
    }

    fn remove_child<T: IsA<DOMNode>>(&self, oldChild: &T) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_remove_child(self.to_glib_none().0, oldChild.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn replace_child<T: IsA<DOMNode>, U: IsA<DOMNode>>(&self, newChild: &T, oldChild: &U) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_replace_child(self.to_glib_none().0, newChild.to_glib_none().0, oldChild.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_node_value(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_node_set_node_value(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_prefix(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_node_set_prefix(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_text_content(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_node_set_text_content(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
