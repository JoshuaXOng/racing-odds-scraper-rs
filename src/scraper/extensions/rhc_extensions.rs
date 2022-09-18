use headless_chrome::{Element, Tab, protocol::cdp::DOM};

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
  let remote_object_id = object
    .object_id
    .ok_or(())?;

  Ok(Element {
    remote_object_id,
    backend_node_id: backend_node_id,
    node_id: 0,
    parent: tab_engine,
  })
}