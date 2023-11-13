//! `Bean` 装配工厂
mod factory;

mod core;

mod actor;
mod bean;

/// 使用 `inventory` 的 `submit`, 用于宏
pub use inventory::submit;

pub mod prelude {
    pub use bean_assembly::*;

    pub use crate::bean::BeanInstance;

    pub use crate::factory::BeanFactory;
}
