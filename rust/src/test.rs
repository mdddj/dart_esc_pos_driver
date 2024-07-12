use crate::api::printer::{LddAlignment, LddConsoleDriver};

#[test]
fn test() {
    let driver = LddConsoleDriver::open();
    let mut printer = driver.open_printer().expect("获取打印失败");
    printer.init();
    printer.align(LddAlignment::Center);
    printer.text("你好梁典典".to_owned());
    printer.println("test println".to_owned());
    printer.feed(4);
    printer.cut();
    printer.flush();
}
