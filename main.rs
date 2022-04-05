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

use std::io;
fn main() {
	println!("OriginGo 2022 Dev");
	println!("请用 Sabaki 或 Lizzie 加载!");
	println!("咳咳，代码都没写好，你怎么可能用得了？");
	let mut gtplog = String::new();
	loop{
		let mut gtp = String::new();
		gtp_read (&mut gtp);//见下面的函数
		gtplog += &mut gtp;//存储功能
		println!("Gtp history is:  {}",gtplog);//用来显示测试gtplog是否正常运作的，并使用gtplog。
		//有没有办法把上面的两行放到gtp_read函数里？
		}
}

fn gtp_read (gtp: &mut String) {
	io::stdin().read_line(gtp).expect("Gtp reading failed.");
	println!("Input:{}", gtp);
	}
