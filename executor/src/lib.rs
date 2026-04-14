// 将核心实现放在独立模块中，便于后续继续扩展执行器逻辑。
mod executor;

// 采用公共类型，让调用者可以直接使用 executor::Executor。
pub use crate::executor::{Executor, Pose};
