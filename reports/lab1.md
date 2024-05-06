# 多道程序与分时任务-获取任务信息

## TaskInfo

```rust
struct TaskInfo{
	status: TaskStatus,
	syscall_times: [u32;MAX_SYSCALL_NUM],
	time: usize
}
```

为task.task.TaskControlBlock新增syscall_times和first_time记录每种系统调用次数和任务第一次被调度时间。

### Status

由于查询的是当前任务的状态，因此 TaskStatus 一定是 Running。

### SyscallTimes

每次系统调用时 `TASK_MANAGER.add_syscall_times(syscall_id);`

```rust
    pub fn add_syscall_times(&self, syscall_id: usize) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].syscall_times[syscall_id] += 1;
        // 打印当前任务的系统调用次数
        // println!("syscall_id: {}, syscall_times: {}", syscall_id, inner.tasks[current].syscall_times[syscall_id]);
    }
```

### SyscallTimes

记录初次调用时间，用当前时间减去即可。（直接用当前时间也没问题，测例考虑不完善）
