use std::io::{self, Write};

use executor::{Executor, Pose};

fn main() {
    // 初始化执行器，默认位置在原点，朝北。
    let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
    println!("Executor 的初始状态为 (0, 0, N).");
    println!("请输入命令序列，包含 M（前进）、L（左转）、R（右转），或输入 Q 退出程序。");
    print_pose(&executor);
    // 进入交互式命令输入循环，允许用户连续输入命令并查看结果。
    loop {
        // 确保输出被刷新，以便用户看到提示。
        if let Err(err) = io::stdout().flush() {
            eprintln!("Failed to flush stdout: {err}");
            break;
        }

        // read_line 会把用户输入添加到 String 中。
        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("读入失败: {err}");
            break;
        }

        // trim() 去掉换行等空白字符，to_uppercase() 允许用户输入小写命令。
        let cmd = input.trim().to_uppercase();
        if cmd.is_empty() {
            continue;
        }

        // 单独输入 Q 时退出交互程序。
        if cmd == "Q" {
            println!("Over.");
            break;
        }

        // 只允许 M/L/R 三种命令，其他输入视为无效。
        if !cmd.chars().all(|ch| matches!(ch, 'M' | 'L' | 'R')) {
            println!("无效命令: {cmd}. 请输入仅包含 M、L、R 的命令序列，或输入 Q 退出程序。");
            continue;
        }

        // 执行整串命令，并在每次执行后展示当前位置和朝向。
        executor.execute(&cmd);
        print_pose(&executor);
    }
}

fn print_pose(executor: &Executor) {
    // 查询当前`Pose`状态并打印。
    let pose = executor.query();
    println!("当前位置: ({}, {}, {})", pose.x, pose.y, pose.heading);
}
