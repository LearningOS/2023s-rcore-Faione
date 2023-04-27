# lab3

## spawn 系统调用

spawn的实现在于结合 fork 与 exec, 即参考 exec 中, 读取elf文件夹初始化进程的部分，以及 fork 中构成父子进程关系的部分，将两部分代码结合，就是spawn的实现

## stride 调度算法

首先构造 `Priority` 用来描述进程的优先级信息，并为 `Priority` 实现 比较 的trait，并依据算法的要求，在 `suspend` 处进行 `Priority` 更新