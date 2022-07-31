/*
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301, USA.
 *
 */

//开发、编译环境 = Debian 11
//IDE = GEANY
//语言 = Rust
//作者 = 苏尔雅
//项目成立时间 = 2022年3月

//第一阶段：处理gtp命令的读写。
use std::io;
//引用
fn main() {
    //fn 函数名（参数）{内容}
    println!("OriginGo 2022 Dev");
    println!("请用 Sabaki 或 Lizzie 加载!");
    //println!("代码都没写好，你怎么可能用得了？");
    let mut gtplog = String::new(); //mut 代表可变的变量。让可以变的什么什么 = 一些东西;
    loop {
        let mut gtp = String::new(); //等号右边是 字符串::新（）;
        gtp_read(&mut gtp); //见下面的函数
        gtplog += &mut gtp; //存储功能
        println!("History:{}", gtplog); //用来显示测试gtplog是否正常运作的，并使用gtplog。
                                        //可最后存在文件里，可作为命令导出，甚至还能让它参与产生Ramdom seed
                                        //也能砍掉23333
                                        //有没有办法把上面的两行放到gtp_read函数里？或者把整个放进去？
    }
}

fn gtp_read(gtp: &mut String) {
    io::stdin().read_line(gtp).expect("Gtp reading failed.");

    //这里要做匹配的工作，将命令识别出来，对我来说难度很大。
    //要把程序分成几个状态：
    //注意，可能会遇到野狐围棋ai连接器的加上数字的形式。到时候再做改动。
    //也许命令输出要紧跟着输入的，所以上面一行的命令可能要延迟放出来？

    //第一个状态是初始化状态，负责gtp引擎和界面的信息交流。这是要支持的几个gtp命令：
    //name = origingo
    //version = 0.01
    //protocol_version = 2
    //list_commands =
    //许多许多的命令，要用空行隔开...
    //quit =	

    //第二个状态就是下围棋的状态了。（以后再分析）//输入第二个状态不懂的东西就回到第一个状态分析问题。
    //未完待续

    println!("Input:{}", gtp);
	let name = "Origingo";
    println!("= {}", &name);
}
