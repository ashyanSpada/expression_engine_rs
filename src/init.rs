use crate::keyword::KeywordManager;
use crate::operator::{BinaryOpFuncManager, UnaryOpFuncManager};
use once_cell::sync::OnceCell;

pub fn init() {
    static INITED: OnceCell<()> = OnceCell::new();
    INITED.get_or_init(|| {
        KeywordManager::new().init();
        UnaryOpFuncManager::new().init();
        BinaryOpFuncManager::new().init();
    });
}

#[cfg(test)]
mod tests {
    use crate::keyword::KeywordManager;

    use super::init;

    #[test]
    fn test_init() {
        init();
        println!("{:?}", KeywordManager::new().list());
        // assert_eq!(KeywordManager::new().list().len(), 6);
    }
}
