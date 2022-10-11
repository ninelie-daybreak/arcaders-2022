# Arcaders-2022

> 一次偶然的机会，看到某乎上推荐的帖子：“有哪些学习 `Rust` 的好项目”，点进去后第一个回答就是 [`jadpole`](https://github.com/jadpole) 的 `arcaders`，作为一个初学者，当然不会放过把它收入 `star` （吃灰）的好机会，但是点开链接后却发现是 `404 not found` 。根据项目的基本介绍，我仍然找到了作者在写该项目的同时编写的[教程](https://github.com/jadpole/jadpole.github.io/tree/update-arcaders-1.13/_posts)，便跟着写了起来，那时候还是大四，写到中途由于毕设，上海疫情等因素的影响，更多的时间都在宿舍偷着乐，所以该项目写到一半就算搁置了。直到毕业后的暑假，闲来无事，本着第二次入门 `Rust` ，第二次入门 `Git`，第三次入门 `Markdown`，顺便翻译翻译教程学学英语的想法继续研究该项目，所幸没有虎头蛇尾，算是将教程中的内容大体实现。
>
> 实际上，项目的最后一次更新也早在六年前，这期间，无论是 `Rust` 编译器，亦或是项目中所用的主要的 `crate`，都发生了较大的改变，为了顺应时代的发展，需要在原项目上做些许改动，这可能导致项目漏洞百出，但是最终项目能够编译运行对于一个新手来说也是莫大的欣慰，但同时也欢迎批评指正！

简介：该系列的目标是通过开发一个简单的老式射击游戏来探索 `Rust` 编程语言和生态系统，教程由 `13` 个部分组成（已经基本完成，正在整理，后续会陆续更新）：

1. [A Simple Window](https://github.com/ninelie-daybreak/arcaders-2022/blob/main/tutorial/1.%20A%20Simple%20Window.md)，本章用来安装 `SDL2`
2. [Event Handling](https://github.com/ninelie-daybreak/arcaders-2022/blob/main/tutorial/2.%20Event%20Handling.md)，本章用来讨论 `Rust` 的生命周期
3. [More Event Handing](https://github.com/ninelie-daybreak/arcaders-2022/blob/main/tutorial/3.%20More%20Event%20Handling.md)，本章用来讨论设计宏
4. [Views](https://github.com/ninelie-daybreak/arcaders-2022)，本章中将学习 `boxes`，`pattern matching`， `trait objects` 和 `dynamic dispatch` 等相关知识
5. [Switching Views](https://github.com/ninelie-daybreak/arcaders-2022)， 本章将会使用 `boxes`，`pattern matching`， `trait objects` 和 `dynamic dispatch`
6. [A Moving Rectangle](https://github.com/ninelie-daybreak/arcaders-2022)， 本章将绘制相关画面
7. [Sprites](https://github.com/ninelie-daybreak/arcaders-2022)，本章中将创建玩家的 `ship`
8. [Backgrounds](https://github.com/ninelie-daybreak/arcaders-2022)，本章中调整窗口大小，规模等
9. [Main Menu](https://github.com/ninelie-daybreak/arcaders-2022)，本章中处理 `textures` 和 `Rust vectors`
10. [Asteroid Attack](https://github.com/ninelie-daybreak/arcaders-2022)，本章将渲染 `animated asteroids`
11. [Shooting Bullets](https://github.com/ninelie-daybreak/arcaders-2022)，本章中将探索 `iterators`
12. [Brawl, at last](https://github.com/ninelie-daybreak/arcaders-2022)，本章中将处理对象间的交互和“爆炸”
13. [Boom!](https://github.com/ninelie-daybreak/arcaders-2022)，本章中将添加音乐并对项目进行完善。

![The result of the 12th episode](https://github.com/jadpole/jadpole.github.io/blob/update-arcaders-1.13/images/arcade-20.png)