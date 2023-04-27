# lab1

思路清晰
- 在task info中增加保存对于数据的字段
- 分别在 `run_first_task`, `run_next_task` 处初始化时间戳
- 由于需要的是总时间，因此在 `task_exit` 记录一个总时间，此外，每次syscall时，根据task的状态(可能并没有必要), 获取当前时间相减得到一个总时间

## 问题

栈空间不足
- 将 task 保存到 Vec 中