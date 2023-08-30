use crate::function::InnerFunctionManager;
use crate::operator::{BinaryOpFuncManager, UnaryOpFuncManager};
use once_cell::sync::OnceCell;

pub fn init() {
    static INITED: OnceCell<()> = OnceCell::new();
    INITED.get_or_init(|| {
        UnaryOpFuncManager::new().init();
        BinaryOpFuncManager::new().init();
        InnerFunctionManager::new().init();
    });
}
