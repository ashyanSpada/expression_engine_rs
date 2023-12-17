use crate::function::InnerFunctionManager;
use crate::operator::{InfixOpManager, PostfixOpManager, PrefixOpManager};
use once_cell::sync::OnceCell;

pub fn init() {
    static INITED: OnceCell<()> = OnceCell::new();
    INITED.get_or_init(|| {
        PrefixOpManager::new().init();
        InfixOpManager::new().init();
        PostfixOpManager::new().init();
        InnerFunctionManager::new().init();
    });
}
