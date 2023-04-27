const MAX_THREAD: usize = 10;
const MAX_RESOURCE: usize = 10;

/// DeadLockChecker保存必要信息，基于死锁检测算法来判断是否发生死锁
#[derive(Debug, Clone, Copy)]
pub struct DeadLockChecker {
    /// 资源-数量
    pub avail: [usize; MAX_RESOURCE],
    /// 线程-分配资源
    pub alloc: [[usize; MAX_RESOURCE]; MAX_THREAD],
    /// 线程-所需资源
    pub need: [[usize; MAX_RESOURCE]; MAX_THREAD],
}

impl DeadLockChecker {
    /// 返回一个初始化的 DeadLockChecker
    pub fn new() -> Self {
        Self {
            avail: [0; MAX_RESOURCE],
            alloc: [[0; MAX_RESOURCE]; MAX_THREAD],
            need: [[0; MAX_RESOURCE]; MAX_THREAD],
        }
    }

    /// 增加目标资源的数量
    pub fn add_res(&mut self, res_id: usize, mount: usize) {
        self.avail[res_id] = mount;
    }

    /// 回收tid占用的资源
    pub fn recycle_res(&mut self, tid: usize, res_id: usize) {
        self.avail[res_id] += 1;
        self.alloc[tid][res_id] -= 1;
    }

    /// 判断当前分配状态是否安全
    pub fn is_safe(&mut self, thread_count: usize, res_count: usize) -> bool {
        let mut work = self.avail;
        let mut finish = [false; MAX_THREAD];

        loop {
            let mut tid: Option<usize> = None;

            for i in 0..thread_count {
                if finish[i] {
                    continue;
                }

                let mut flag = false;
                for j in 0..res_count {
                    if self.need[i][j] > work[j] {
                        flag = true;
                        break;
                    }
                }

                if flag {
                    continue;
                } else {
                    tid = Some(i);
                }
            }

            if let Some(tid) = tid {
                for j in 0..res_count {
                    work[j] += self.alloc[tid][j];
                }
                finish[tid] = true
            } else {
                break;
            }
        }

        (0..thread_count).into_iter().all(|idx| finish[idx])
    }
}
