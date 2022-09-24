use headless_chrome::{
    protocol::cdp::DOM::{self, Node},
    Element, Tab,
};

pub fn create_element_from_bnid(tab_engine: &Tab, backend_node_id: u32) -> Result<Element, ()> {
    let object = tab_engine
        .call_method(DOM::ResolveNode {
            backend_node_id: Some(backend_node_id),
            node_id: None,
            object_group: None,
            execution_context_id: None,
        })
        .or(Err(()))?
        .object;
    let remote_object_id = object.object_id.ok_or(())?;

    Ok(Element {
        remote_object_id,
        backend_node_id,
        node_id: 0,
        parent: tab_engine,
    })
}

pub fn is_node_of_class(node: &Node, class: &str) -> bool {
    let attributes = node.attributes.clone().unwrap_or_default();

    let c_attr_key_index =
        if let Some(c_attr_index) = attributes.iter().position(|attribute| attribute == "class") {
            c_attr_index
        } else {
            return false;
        };

    let c_attr_value_index = c_attr_key_index + 1;

    let c_attr_value = if let Some(c_attr_value) = attributes.get(c_attr_value_index) {
        c_attr_value
    } else {
        return false;
    };

    c_attr_value.contains(class)
}

pub fn for_each_node(node: &Node, callback: &mut dyn FnMut(&Node)) {
    callback(node);

    let children = match &node.children {
        Some(children) => children,
        _ => return
    };

    for child in children {
        for_each_node(child, callback);
    }
}
