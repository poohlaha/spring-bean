//! 定义 `Actix` 中的 `Actor`

use crate::bean::BeanInstance;
use crate::factory::BeanFactory;
use actix::prelude::*;
use std::any::{type_name, Any};
use std::collections::HashMap;
use std::sync::Arc;

/// 类型, 静态的, 可以被线程安全的传递和共享
pub type DynAny = dyn Any + 'static + Send + Sync;

/// 所有注入的 Bean Map
pub type InjectBeanMap = HashMap<String, BeanInstance>;
pub type ProviderBeanMap = HashMap<String, Arc<DynAny>>;

/// 定义工厂消息体
#[derive(Message)]
#[rtype(result = "Option<ContainerData>")]
pub struct Factory;

/// 查询
#[derive(Message)]
#[rtype(result = "Option<BeanQueryFactoryResult>")]
pub enum BeanQueryFactory {
    Init,
    QueryName(String),
    QueryNames,
}

/// 获取查询结果
pub enum BeanQueryFactoryResult {
    None,
    Names(Vec<String>),
    Bean(Option<Arc<DynAny>>),
}

/// `Actor` 返回的工厂数据
#[derive(Debug, Clone)]
pub struct ContainerData(pub Arc<ProviderBeanMap>);

impl ContainerData {
    /// 通过 `name` 获取 `Actor` 中的 `地址`
    pub fn get_address_by_name<T: Actor>(&self, name: &str) -> Option<Addr<T>> {
        return self.0.get(name).map(|x| x.clone().downcast::<Addr<T>>().ok()).flatten().map(|x| x.as_ref().clone());
    }

    /// 获取 `Actor` 中的 `地址`
    pub fn get_address<T: Actor>(&self) -> Option<Addr<T>> {
        return self.get_address_by_name(type_name::<T>());
    }

    /// 通过 `name` 获取 `Bean`
    pub fn get_bean_by_name<T: 'static + Sync + Send>(&self, name: &str) -> Option<Arc<T>> {
        return self.0.get(name).map(|x| x.clone().downcast::<Addr<T>>().ok()).flatten();
    }

    /// 获取 `Bean`
    pub fn get_bean<T: Actor>(&self) -> Option<Arc<T>> {
        return self.get_bean_by_name(type_name::<T>());
    }
}

/// 定义 `Provider`
#[derive(Clone)]
pub enum Provider {
    Fn(Arc<dyn Fn() -> Option<Arc<DynAny>> + Send + Sync>),
    Value(Arc<DynAny>),
}

/// 定义容器事件
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub enum ContainerEvent {
    Inject { factory: BeanFactory, data: ContainerData },
    Complete,
}
