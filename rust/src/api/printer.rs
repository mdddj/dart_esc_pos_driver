use flutter_rust_bridge::DartFnFuture;
use recibo::{
    Alignment, BarcodeFont, BarcodeSystem, BarcodeTextPosition, ConsoleDriver, FileDriver, Font,
    GraphicSize, NetworkDriver, Printer, QrCorrectionLevel, QrModel, UnderlineMode,
};

pub struct LddNetworkDriver {
    network_driver: Box<NetworkDriver>,
}

pub struct LddFileDriver {
    file_driver: Box<FileDriver>,
}

pub struct LddConsoleDriver {
    console_driver: Box<ConsoleDriver>,
}

pub struct LddPrinter {
    printer: Box<Printer>,
}

unsafe impl Send for LddPrinter {}
unsafe impl Sync for LddPrinter {}
unsafe impl Send for LddFileDriver {}
unsafe impl Sync for LddFileDriver {}
unsafe impl Send for LddNetworkDriver {}
unsafe impl Sync for LddNetworkDriver {}
unsafe impl Send for LddConsoleDriver {}
unsafe impl Sync for LddConsoleDriver {}

pub enum LddAlignment {
    Left,
    Center,
    Right,
}

impl Into<Alignment> for LddAlignment {
    fn into(self) -> Alignment {
        match self {
            LddAlignment::Left => Alignment::Left,
            LddAlignment::Center => Alignment::Center,
            LddAlignment::Right => Alignment::Right,
        }
    }
}

pub enum LddFont {
    A,
    B,
    C,
}

impl Into<Font> for LddFont {
    fn into(self) -> Font {
        match self {
            LddFont::A => Font::A,
            LddFont::B => Font::B,
            LddFont::C => Font::C,
        }
    }
}

pub enum LddUnderlineMode {
    None,
    Single,
    Double,
}

impl Into<UnderlineMode> for LddUnderlineMode {
    fn into(self) -> UnderlineMode {
        match self {
            LddUnderlineMode::None => UnderlineMode::None,
            LddUnderlineMode::Single => UnderlineMode::Single,
            LddUnderlineMode::Double => UnderlineMode::Double,
        }
    }
}

impl LddPrinter {
    ///初始化打印机。
    pub fn init(&mut self) {
        let _ = self.printer.init().expect("init failed");
    }

    ///将打印机重置为默认设置。
    pub fn reset(&mut self) {
        let _ = self.printer.reset();
    }

    ///将文本向左、向右或居中对齐。
    pub fn align(&mut self, alignment: LddAlignment) {
        let _ = self.printer.align(alignment.into());
    }

    ///将左间隔设置为n个点。
    pub fn left(&mut self, dots: u16) {
        let _ = self.printer.left(dots);
    }

    ///设置可打印区域的宽度。
    pub fn width(&mut self, margin: u16) {
        let _ = self.printer.width(margin);
    }

    ///将字体设置为样式“a”、“b”或“c”。
    pub fn font(&mut self, font: LddFont) {
        let _ = self.printer.font(font.into());
    }

    ///将文本的重点设置为粗体。
    pub fn bold(&mut self, enabled: bool) {
        let _ = self.printer.bold(enabled);
    }

    ///设置文本的字体大小。
    pub fn text_size(&mut self, width_multiplier: u8, height_multiplier: u8) {
        let _ = self.printer.text_size(width_multiplier, height_multiplier);
    }

    ///重置文本的字体大小。
    pub fn reset_text_size(&mut self) {
        let _ = self.printer.reset_text_size();
    }

    ///用单笔或双笔在文本下划线。
    pub fn underline(&mut self, mode: LddUnderlineMode) {
        let _ = self.printer.underline(mode.into());
    }

    ///对文本应用双击效果。
    pub fn doublestrike(&mut self, enabled: bool) {
        let _ = self.printer.doublestrike(enabled);
    }

    ///调整文本行间距。
    pub fn linespacing(&mut self, height: u8) {
        let _ = self.printer.linespacing(height);
    }

    ///将行距重置为默认值。
    pub fn reset_linespacing(&mut self) {
        let _ = self.printer.reset_linespacing();
    }

    ///将文本颠倒过来。
    pub fn flip(&mut self, enabled: bool) {
        let _ = self.printer.flip(enabled);
    }

    /// 启用黑色背景上的白色文本。
    pub fn reverse_colours(&mut self, enabled: bool) {
        let _ = self.printer.reverse_colours(enabled);
    }

    ///构建二维码
    pub async fn qr(&mut self, fun_call: impl Fn() -> DartFnFuture<DartQrBuilder>) {
        let dart_return_obj = fun_call().await;
        let _ = self.printer.qr(|builder| {
            let obj = dart_return_obj.clone();
            if let Some(size) = obj.size {
                builder.size(size);
            }
            if let Some(text) = obj.text {
                builder.text(text);
            }
            if let Some(model) = obj.model {
                builder.model(model.into());
            }
            if let Some(level) = obj.level {
                builder.correction_level(level.into());
            }
            builder
        });
    }

    ///构建条形码
    pub async fn barcode(&mut self, fun_call: impl Fn() -> DartFnFuture<DartBarcodeBuilder>) {
        let dart_return_obj = fun_call().await;
        let _ = self.printer.barcode(|builder| {
            let obj = dart_return_obj.clone();
            if let Some(text) = obj.text {
                builder.text(text);
            }
            if let Some(text_position) = obj.text_position {
                builder.text_position(text_position.into());
            }
            if let Some(system) = obj.system {
                builder.system(system.into());
            }
            if let Some(font) = obj.font {
                builder.font(font.into());
            }
            // if let Some(width) = obj.width {
            //     &mut builder.width(width);
            // }
            // if let Some(height) = obj.height {
            //     &mut builder.height(height);
            // }
            builder
        });
    }

    ///打印图形
    pub async fn graphic(&mut self, fun_call: impl Fn() -> DartFnFuture<DartGraphicBuilder>) {
        let dart_return_obj = fun_call().await;
        let _ = self.printer.graphic(|builder| {
            let obj = dart_return_obj.clone();
            builder.path(obj.path);
            if let Some(size) = obj.size {
                builder.size(size.into());
            }
            if let Some(max_width) = obj.max_width {
                builder.max_width(max_width);
            }
            builder
        });
    }

    ///送入n行纸张。
    pub fn feed(&mut self, n: u8) {
        let _ = self.printer.feed(n);
    }

    /// 将进纸反转n行。
    pub fn reverse_feed(&mut self, n: u8) {
        let _ = self.printer.reverse_feed(n);
    }

    ///对纸张执行完整的剪切。
    pub fn cut(&mut self) {
        let _ = self.printer.cut();
    }

    /// 对纸张执行部分剪切。
    pub fn partial_cut(&mut self) {
        let _ = self.printer.partial_cut();
    }

    ///打印指定的文本。
    pub fn print(&mut self, text: String) {
        let _ = self.printer.print(text);
    }

    ///打印以新行结尾的指定文本。
    pub fn println(&mut self, text: String) {
        let _ = self.printer.println(text);
    }
    /// 与println相同，打印以新行结尾的指定文本。
    pub fn text(&mut self, text: String) {
        let _ = self.printer.text(text);
    }

    ///推送
    pub fn flush(&mut self) {
        let _ = self.printer.flush();
    }
}

///图像生成器
#[derive(PartialEq, Debug, Clone)]
pub struct DartGraphicBuilder {
    pub path: String,
    pub size: Option<DartGraphicSize>,
    pub max_width: Option<u32>,
}
#[derive(PartialEq, Debug, Clone)]
pub enum DartGraphicSize {
    Normal,
    DoubleWidth,
    DoubleHeight,
    DoubleWidthAndHeight,
}
impl From<DartGraphicSize> for GraphicSize {
    fn from(size: DartGraphicSize) -> Self {
        match size {
            DartGraphicSize::Normal => GraphicSize::Normal,
            DartGraphicSize::DoubleWidth => GraphicSize::DoubleWidth,
            DartGraphicSize::DoubleHeight => GraphicSize::DoubleHeight,
            DartGraphicSize::DoubleWidthAndHeight => GraphicSize::DoubleWidthAndHeight,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum DartBarcodeFont {
    A,
    B,
}

impl From<DartBarcodeFont> for BarcodeFont {
    fn from(font: DartBarcodeFont) -> Self {
        match font {
            DartBarcodeFont::A => BarcodeFont::A,
            DartBarcodeFont::B => BarcodeFont::B,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum DartBarcodeSystem {
    UpcA,
    UpcE,
    Ean13,
    Ean8,
    Code39,
    Itf,
    Codabar,
}

impl From<DartBarcodeSystem> for BarcodeSystem {
    fn from(system: DartBarcodeSystem) -> Self {
        match system {
            DartBarcodeSystem::UpcA => BarcodeSystem::UpcA,
            DartBarcodeSystem::UpcE => BarcodeSystem::UpcE,
            DartBarcodeSystem::Ean13 => BarcodeSystem::Ean13,
            DartBarcodeSystem::Ean8 => BarcodeSystem::Ean8,
            DartBarcodeSystem::Code39 => BarcodeSystem::Code39,
            DartBarcodeSystem::Itf => BarcodeSystem::Itf,
            DartBarcodeSystem::Codabar => BarcodeSystem::Codabar,
        }
    }
}

///构建条形码的对象
#[derive(PartialEq, Debug, Clone)]
pub struct DartBarcodeBuilder {
    // pub width: Option<u8>,
    // pub height: Option<u8>,
    pub text: Option<String>,
    pub text_position: Option<DartBarcodeTextPosition>,
    pub system: Option<DartBarcodeSystem>,
    pub font: Option<DartBarcodeFont>,
}
#[derive(PartialEq, Debug, Clone)]
pub enum DartBarcodeTextPosition {
    None,
    Above,
    Below,
    Both,
}

impl From<DartBarcodeTextPosition> for BarcodeTextPosition {
    fn from(value: DartBarcodeTextPosition) -> Self {
        match value {
            DartBarcodeTextPosition::None => BarcodeTextPosition::None,
            DartBarcodeTextPosition::Above => BarcodeTextPosition::Above,
            DartBarcodeTextPosition::Below => BarcodeTextPosition::Below,
            DartBarcodeTextPosition::Both => BarcodeTextPosition::Both,
        }
    }
}

///构建二维码的对象
#[derive(PartialEq, Debug, Clone)]
pub struct DartQrBuilder {
    pub size: Option<u8>,
    pub text: Option<String>,
    pub model: Option<DartQrMode>,
    pub level: Option<LddQrCorrectionLevel>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DartQrMode {
    Model1,
    Model2,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LddQrCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}

impl From<LddQrCorrectionLevel> for QrCorrectionLevel {
    fn from(level: LddQrCorrectionLevel) -> Self {
        match level {
            LddQrCorrectionLevel::Low => QrCorrectionLevel::Low,
            LddQrCorrectionLevel::Medium => QrCorrectionLevel::Medium,
            LddQrCorrectionLevel::Quartile => QrCorrectionLevel::Quartile,
            LddQrCorrectionLevel::High => QrCorrectionLevel::High,
        }
    }
}

impl From<DartQrMode> for QrModel {
    fn from(value: DartQrMode) -> Self {
        match value {
            DartQrMode::Model1 => QrModel::Model1,
            DartQrMode::Model2 => QrModel::Model2,
        }
    }
}

impl LddFileDriver {
    pub fn open(path: String) -> Result<LddFileDriver, String> {
        let fd = FileDriver::new(path);
        match fd {
            Ok(ok) => Ok(LddFileDriver { file_driver: ok }),
            Err(e) => Err(e.to_string()),
        }
    }

    ///打开打印机
    pub fn open_printer(self) -> Result<LddPrinter, String> {
        match Printer::open(self.file_driver) {
            Ok(p) => Ok(LddPrinter {
                printer: Box::new(p),
            }),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl LddNetworkDriver {
    pub fn open(host: String, port: u16) -> Result<LddNetworkDriver, String> {
        let nd = NetworkDriver::open(host, port);
        match nd {
            Ok(ok) => Ok(LddNetworkDriver { network_driver: ok }),
            Err(e) => Err(e.to_string()),
        }
    }

    ///打开打印机
    pub fn open_printer(self) -> Result<LddPrinter, String> {
        let r = Printer::open(self.network_driver);
        match r {
            Ok(ok) => Ok(LddPrinter {
                printer: Box::new(ok),
            }),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl LddConsoleDriver {
    pub fn open() -> LddConsoleDriver {
        let nd = ConsoleDriver::open();
        LddConsoleDriver { console_driver: nd }
    }

    ///打开打印机
    pub fn open_printer(self) -> Result<LddPrinter, String> {
        let r = Printer::open(self.console_driver);
        match r {
            Ok(ok) => Ok(LddPrinter {
                printer: Box::new(ok),
            }),
            Err(e) => Err(e.to_string()),
        }
    }
}

///获取设备
pub fn connect_to_network_driver(host: String, port: u16) -> Result<LddNetworkDriver, String> {
    LddNetworkDriver::open(host, port)
}
