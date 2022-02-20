## Intro

此项目为`ostep code`的Rust版本，其C语言版本的代码地址为：

https://github.com/remzi-arpacidusseau/ostep-code

其主要内容介绍如下：（每个目录包含一个主题/章节，例子写在src/bin目录下，通过cargo run --bin *appname* 来运行）

- cpu-api

展示fork(),execv(),sleep(),wait()等与进程相关基本函数的功能。

- cpu-sched-lottery

  rust实现了链表linklist和模拟了一个基础的随机调度。

  
- dist-intro

  展示一个基础的客户端、服务端通过UDP通信的功能。

- intro

  展示进程、I/O、多线程、内存分配的相关功能。

- threads-api

  展示进程的创建和阻塞等功能。


- threads-bugs

  通过特定的程序来触发进程的死锁、读写不同步、不支持互斥等BUG。

- threads-cv

  利用锁和条件变量来实现一些基础的同步互斥功能。

  

- threads-intro

  通过多线程追踪变量的变化来分析多线程并行执行的特点。

  

- threads-sema

  用信号量来解决读写问题、哲学家就餐问题、生产者消费问题、基本同步问题

  

- vm-intro

  展示代码段数据（函数指针）、栈数据（局部变量）、堆上数据（alloc的数据）的地址。


- threads-locks/compare-and-swap（未实现）

  实现一个基本的lock功能通过compare&swap原语。


## Developers

Xu Shanpu, Liu Fengyuan, Chen Yu