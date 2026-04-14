// Pose 包含了车辆在二维平面上的位置和朝向信息：x、y 负责描述二维平面中的位置，heading 负责描述当前朝向。
// trait 派生：
// - Debug: 便于调试时打印结构体内容，查看当前状态。
// - Copy和Clone: 让 Pose 可以按值复制。
// - PartialEq和Eq: 便于在测试里直接使用 assert_eq! 得到测试结果。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pose {
    // x 轴表示东西方向上的位置，设定东为正方向，向东移动时 x + 1，向西移动时 x - 1。
    pub x: i32,
    // y 轴表示南北方向上的位置，设定北为正方向，向北移动时 y + 1，向南移动时 y - 1。
    pub y: i32,
    // heading 使用单个字符表示朝向。
    pub heading: char,
}

impl Pose {
    // 构造函数，用来创建指定位置和朝向的 Pose。
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }
}

impl Default for Pose {
    // Pose 的初始状态为(0, 0, N)。
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}
// Executor 的相关方法
pub struct Executor {
    pose: Pose,
}

impl Executor {
    // with_pose() 作为构造函数，允许调用者直接指定初始 Pose。
    pub fn with_pose(pose: Pose) -> Self {
        Self { pose }
    }

    // 对with_pose() 的外壳包装。
    pub fn new(pose: Pose) -> Self {
        Self::with_pose(pose)
    }

    // 根据读取到的命令字符串执行相应操作，更新内部状态。
    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            // 使用 match方法处理穷尽所有情况。
            match cmd {
                // M 表示沿当前朝向前进一步。
                'M' => self.move_forward(),
                // L 表示左转 90 度，位置不变。
                'L' => self.turn_left(),
                // R 表示右转 90 度，位置不变。
                'R' => self.turn_right(),
                _ => {}
            }
        }
    }

    // 查询当前 Pose 的方法，返回一个 Pose 的副本，便于调用者直接使用查询结果。
    pub fn query(&self) -> Pose {
        self.pose
    }

    // 前进操作
    fn move_forward(&mut self) {
        match self.pose.heading {
            'E' => self.pose.x += 1,
            'S' => self.pose.y -= 1,
            'W' => self.pose.x -= 1,
            'N' => self.pose.y += 1,
            _ => {}
        }
    }

    // 左转操作
    fn turn_left(&mut self) {
        self.pose.heading = match self.pose.heading {
            'E' => 'N',
            'N' => 'W',
            'W' => 'S',
            'S' => 'E',
            heading => heading,
        };
    }

    // 右转操作
    fn turn_right(&mut self) {
        self.pose.heading = match self.pose.heading {
            'E' => 'S',
            'S' => 'W',
            'W' => 'N',
            'N' => 'E',
            heading => heading,
        };
    }
}
// Executor 实现了 Default trait，允许使用默认构造函数创建初始状态为(0, 0, N) 的执行器。
impl Default for Executor {
    // Executor 的初始状态为(0, 0, N)。
    fn default() -> Self {
        Self {
            pose: Pose::default(),
        }
    }
}
