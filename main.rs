use std::io;
fn main() {
	println!("OriginGo 2022 Dev");
	println!("请用 Sabaki 或 Lizzie 加载!");
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
