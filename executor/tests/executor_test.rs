use executor::{Executor, Pose};// 优先最细粒度的引用方式，如果存在名字冲突，再使用引入模块的方式。
// 验证 M 指令在不同朝向下的行为。
mod move_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
        // 初始位置在原点，朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::new(original_pose);

        // 执行一次前进指令。
        executor.execute("M");

        // 按照需求文档，车辆应处在 (1, 0) 位置，且方向朝东。
        let expected_pose = Pose::new(1, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_minus_1_given_command_is_m_and_facing_is_s() {
        // 初始位置在原点，朝南方向。
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::new(original_pose);

        // 执行一次前进指令。
        executor.execute("M");

        // 按照需求文档，车辆应处在 (0, -1) 位置，且方向朝南。
        let expected_pose = Pose::new(0, -1, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_given_command_is_m_and_facing_is_w() {
        // 初始位置在原点，朝西方向。
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::new(original_pose);

        // 执行一次前进指令。
        executor.execute("M");

        // 按照需求文档，车辆应处在 (-1, 0) 位置，且方向朝西。
        let expected_pose = Pose::new(-1, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_plus_1_given_command_is_m_and_facing_is_n() {
        // 初始位置在原点，朝北方向。
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::new(original_pose);

        // 执行一次前进指令。
        executor.execute("M");

        // 按照需求文档，车辆应处在 (0, 1) 位置，且方向朝北。
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}