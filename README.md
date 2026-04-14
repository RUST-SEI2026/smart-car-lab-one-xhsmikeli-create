[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/STsxwOT3)

# first_program

`first_program` 是《Rust 语言企业软件开发实践》实验 1 的代码工程。  
本实验围绕 `Executor` 组件展开，目标是模拟自动驾驶车辆执行器对基础控制指令的处理过程，并通过测试先行的方式构建一套清晰、可靠的测试防护网。

## 1. 实验目标

根据 PPT 中“需求 1：支持基本控制指令”的描述，`Executor` 需要完成以下职责：

- 提供初始化接口，设置车辆初始位置和朝向 `(x, y, heading)`
- 支持三种基础指令
  - `M`：沿当前朝向前进一步
  - `L`：左转 90 度，位置不变
  - `R`：右转 90 度，位置不变
- 提供查询接口，返回当前车辆坐标和朝向

约束条件：

- `x` 和 `y` 使用 `i32`
- `heading` 使用 `char`，范围为 `E`、`S`、`W`、`N`
- 不考虑整数溢出
- 调用者保证输入参数合法

## 2. 项目结构

```text
first_program/
├── Cargo.toml                  # workspace 配置
├── README.md
└── executor/
    ├── Cargo.toml              # executor crate
    ├── src/
    │   ├── lib.rs              # 对外暴露公共接口
    │   ├── executor.rs         # Executor 与 Pose 的核心实现
    │   └── main.rs             # 命令行交互入口
    └── tests/
        └── executor_test.rs    # 集成测试文件
```

这个工程采用了 workspace + crate 的组织方式：

- 根目录 `Cargo.toml` 负责管理工作区
- `executor` 是真正实现业务逻辑和测试的 crate
- `tests/executor_test.rs` 通过公共接口测试组件行为，符合“从外部视角验证需求”的思路

## 3. 接口设计

PPT 中给出的接口设计思想是：先明确组件职责，再定义初始化、执行、查询这三类接口。  
本项目对应的 Rust 设计如下：

### 3.1 Pose

`Pose` 用于描述车辆当前位姿：

```rust
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}
```

相关能力：

- `Pose::new(x, y, heading)`：构造指定位置和朝向
- `Default for Pose`：返回默认位姿 `(0, 0, 'N')`

### 3.2 Executor

`Executor` 表示执行器组件，内部保存当前车辆状态：

```rust
pub struct Executor {
    pose: Pose,
}
```

对外接口：

- `Executor::with_pose(pose)`：按指定位姿初始化
- `Executor::new(pose)`：与 `with_pose` 语义一致，便于测试编写
- `execute(&mut self, cmds: &str)`：执行一串控制指令
- `query(&self) -> Pose`：查询当前位姿
- `Default for Executor`：创建默认状态的执行器

## 4. 命名实践

PPT 强调了 Rust 命名规范和领域语言的一致性。本项目中的命名体现如下：

- 类型名使用大驼峰：`Executor`、`Pose`
- 模块名、函数名、方法名使用蛇形：`move_forward`、`turn_left`、`turn_right`
- 测试函数名使用 `should_..._given_...` 格式

例如：

```rust
fn should_return_x_plus_1_given_command_is_m_and_facing_is_e()
```

这个名字直接表达了三层信息：

- 预期结果：`should_return_x_plus_1`
- 触发条件：`given_command_is_m`
- 前置状态：`and_facing_is_e`

测试失败时，只看函数名就能快速定位业务场景，这正是“测试即文档”的体现。

## 5. Rust 测试相关语法说明

这一部分是本实验最值得重点掌握的内容。

### 5.1 测试文件位置

本项目测试文件位于：

```text
executor/tests/executor_test.rs
```

放在 `tests/` 目录下的测试属于集成测试（integration test），它会像外部使用者一样，通过 crate 的公共接口来验证行为。

### 5.2 引入被测试对象

```rust
use executor::{Executor, Pose};
```

这里直接引入 crate 暴露出来的公共类型，说明测试依赖的是公开 API，而不是内部私有实现。

### 5.3 使用模块对测试分组

```rust
mod move_tests {
    use super::*;
}
```

作用：

- 用 `mod` 按功能组织测试
- `move_tests`、`turn_left_tests`、`turn_right_tests` 能清晰表达分组目的
- `use super::*;` 表示把外层已经引入的名字带进当前测试模块，减少重复书写

### 5.4 `#[test]` 属性

```rust
#[test]
fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
    // ...
}
```

`#[test]` 是 Rust 的测试标记。带有这个属性的函数会在 `cargo test` 时被测试框架自动发现和执行。

要求：

- 必须是无参数函数
- 一般返回 `()`
- 函数内部通过断言判断是否通过

### 5.5 `let mut` 与可变状态

```rust
let mut executor = Executor::new(original_pose);
```

因为 `execute()` 会修改执行器内部状态，所以变量必须声明为 `mut`。  
这也体现了 Rust 对可变性的显式要求：只有真正会变化的数据才标记为可变。

### 5.6 Given-When-Then 写法

PPT 明确要求测试函数体按 Given-When-Then 三段式组织：

```rust
#[test]
fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::new(original_pose);

    // when
    executor.execute("M");

    // then
    let expected_pose = Pose::new(1, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}
```

含义如下：

- Given：准备测试数据和初始环境
- When：执行被测动作
- Then：断言结果是否符合预期

这种写法的好处是结构稳定、可读性强，也更方便别人理解测试在验证什么。

### 5.7 `assert_eq!` 断言宏

```rust
assert_eq!(expected_pose, executor.query());
```

这是实验中最核心的断言语法之一，用于比较“期望值”和“实际值”是否相等。

要让这个断言工作正常，`Pose` 需要实现：

```rust
#[derive(PartialEq, Eq, Debug)]
```

原因：

- `PartialEq` / `Eq`：支持相等比较
- `Debug`：测试失败时，Rust 能把左右两边的值打印出来，方便定位问题

### 5.8 运行测试

在项目根目录执行：

```bash
cargo test
```

或者只运行 `executor` crate：

```bash
cargo test -p executor
```

如果只想运行某一个测试文件：

```bash
cargo test -p executor --test executor_test
```

## 6. 测试设计原理

README 不只是说明“怎么写”，更要说明“为什么这样写”。下面内容对应 PPT 中开发者测试部分的核心原则。

### 6.1 FIRST 原则

#### Fast

测试要快。  
本项目的测试全部是纯内存计算，不访问文件、网络、数据库，因此反馈很快，适合频繁运行。

#### Isolated

测试要独立。  
每个测试都会自己创建新的 `Pose` 和 `Executor`，互不共享状态，因此：

- 任意单个测试都可以独立运行
- 测试执行顺序不会影响结果

#### Repeatable

测试要可重复。  
测试中没有随机数、时间依赖和外部环境依赖，因此多次运行结果应保持一致。

#### Self-Validating

测试要自验。  
本项目通过 `assert_eq!` 自动判断测试是否成功，不需要人工看日志确认。

#### Timely

测试要及时。  
实验强调“测试先行”，先设计用例，再编写最小功能代码让测试通过，这样能更早发现需求理解错误。

### 6.2 正交分解法

PPT 提出了用正交分解法设计测试用例。  
本项目把行为拆成三个独立维度：

- `M` 指令：按当前朝向移动
- `L` 指令：改变朝向但不改变位置
- `R` 指令：改变朝向但不改变位置

再结合四个朝向分别验证：

- `M + E/S/W/N`
- `L + E/S/W/N`
- `R + E/S/W/N`

这样可以覆盖主要路径，同时保证每个测试用例足够小、足够清晰。

### 6.3 测试即文档

测试名称采用统一格式：

```text
should_<expected>_given_<condition>
```

例如：

- `should_return_heading_n_given_command_is_l_and_facing_is_e`
- `should_return_heading_e_given_command_is_r_and_facing_is_n`

这类命名的价值在于：

- 直接表达业务语义
- 测试失败时容易定位问题
- 新成员阅读测试时，不需要先钻进实现细节也能理解需求

## 7. 当前测试覆盖内容

本项目已经覆盖以下场景：

- 移动指令 `M` 在四个朝向下的行为
- 左转指令 `L` 在四个朝向下的行为
- 右转指令 `R` 在四个朝向下的行为
- 命令串 `MLMMRM` 的集成行为
- `Executor::default()` 的默认状态

其中集成测试：

```rust
executor.execute("MLMMRM");
```

对应 PPT 中的示意图路径，最终结果为：

```text
(-2, 2, N)
```

## 8. 如何运行程序

运行交互式命令行程序：

```bash
cargo run -p executor
```

输入示例：

```text
Command> MLMMRM
Current pose: (-2, 2, N)
Command> Q
Over.
```

## 9. 学习总结

这个实验的重点不只是“把功能写出来”，而是通过一个很小的业务案例练习高质量编程的基本习惯：

- 先澄清需求，再设计接口
- 用清晰的命名表达业务意图
- 用小而独立的测试建立防护网
- 使用 Rust 的模块系统、trait、模式匹配和断言宏，把代码写得可读、可测、可维护

如果把 `executor/src/executor.rs` 看作“业务实现”，那么 `executor/tests/executor_test.rs` 就是这份业务需求最直接的可执行文档。
