# lab5

## Deadlock Detect

死锁检测属于一种策略，因此可以单独进行开发，而具体过程就是将死锁检测算法一代码的进行进行实现，包装为一个 `DeadlockChecker` 用来保存资源分配的状态，并提供一个 `is_safe` 函数来判断分配是否会导致死锁

为一个进程设置中的需要进行检测的资源(mutex, semaphore)增加对应的 `DeadlockChecker` 进行资源分配追踪，并在资源创建/使用/回收的位置向 `DeadlockChecker` 同步资源变化的信息，同时在 使用 处进行死锁检测，决定是否分配资源

注意: 不要遗漏 `syscall/process` 中 `sys_get_time` 方法的实现
