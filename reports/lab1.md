# lab1

## 编程作业

**目标**

实现系统调用 `TaskInfo`， 获取正在执行的任务控制块相关信息（任务状态）、任务使用的系统调用及调用次数、任务总运行时长（单位ms）

**思路**

OS已实现的部分中，可以很容易的获得任务状态， 但对于系统调用及调用次数与任务总运行时长，OS中并没有提供相应的机制，因此重点应考虑这两者如何获得。

首先选择在 `TaskControlBlock` 中添加这两个字段，这样在稍后的过程中，只要能够拿到当前任务的控制块，就能够很容易地获取这些字段或进行更新。
- `time`: 初始为0， 当task首次被调度时，更新为当前时间，而当task退出时，更新为总运行时间
- `syscall_times`： 在每次系统调用时，拿到当前正在执行的任务的控制块，并更新syscall_times

基于以上的设计，在 `os/src/task/mod.rs` 中增加 `current_task_control_block` 与 `update_task_syscall_info` 方法，并在进行任务调度的 `run_first_task` 、 `mark_current_exited` 和 `run_next_task` 进行时间戳的更新, 以及 `os/src/trap/mod.rs` 中进行系统调用的更新，就足够支持系统调用的完成了。

考虑到 `syscall_times` 是个比较大的数组, 而在初始化 `TaskManager` 时需要构造整个tasks数组，这显然超过了内核栈的默认大小(4096 * 2), 而为了解决这一问题，可以通过使用堆数据结构，如使用Box包裹task，或者使用Vec来存储task。

## 简答作业

### 1

|             |                                    |
| ----------- | ---------------------------------- |
| SBI/Version | RustSBI-QEMU Version 0.2.0-alpha.2 |

报错日志
- 访问了非法地址 `0x0`
- 使用了u mode 下的非法指令 `sret`
- 访问来 u mode 下不允许访问的 `sstatus` 寄存器

```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x80400414, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.

```

### 2


1. `a0` 代表的了当前任务的trap context
- 初始时应用内核栈中会压入一个TrapContext,其中设置了应用Entry和用户栈，同时设置了应用首次switch的ra为 `__restore`，此时 `__restore`的用法为从内核态跳转至用户程序并开始执行
- 其余时间，`__restore` 都用来恢复task上下文并返回中断处继续执行
2. `sstatus`， `sepc`， `sscratch` 三个CSR寄存器, 更改`sstatus`意味着恢复CPU状态，`sepc` 保存了中断前要执行的指令，`sscratch`包存了用户栈指针
3.  `x2` 为sp寄存器，其值已经保存到 `sscratch`中 ，`x4` 为tp(Thread Pointer)寄存器，当前实现中并不存在线程切换，也没有必要保存 
4.  `sp` 恢复为了用户栈， `sscratch` 则保存了内核栈
5.  `sret`, 该指令执行时，会修改 `sstatus` 中的 `spp` 恢复为之前的特权级，同时将 `pc` 设置为 `sepc` 中的值以恢复到用户程序执行
6.  `sp`设置为内核栈，而`sscratch`保存了用户栈
7.  `ecall`


## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。