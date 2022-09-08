pub struct Page {
  pub engine: i32
}

pub trait AsPage {
  fn get_page(&self) -> &Page;

  fn do_something(&self) {
    println!("{}", self.get_page().engine)
  }

  fn take_screenshot(&self) {

  }

  fn refresh(&self) {

  }

  fn detect_drift(&self) {
    
  }
}
