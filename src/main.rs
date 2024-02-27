use std::error::Error;
use std::net::UdpSocket;

use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

fn send_email(body: &str) -> Result<(), Box<dyn Error>> {
    let from = "979616837@qq.com"; // 发件邮箱
    let to = "979616837@qq.com"; // 收件邮箱

    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject("推送开机密码") //邮件主题
        .body(body.to_string())?;//邮件内容

    let smtp_server = "smtp.qq.com"; // 根据邮件服务商而定
    let smtp_username = "979616837@qq.com"; // 发件邮箱
    let smtp_password = "xxxxx"; // 授权码,不同邮件服务商获取方式有所不同,可搜索解决;qq邮箱可参考 https://codeantenna.com/a/PwKbc0S5dd


    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

    let mailer = SmtpTransport::relay(smtp_server)?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("邮件发送成功"),
        Err(e) => eprintln!("不能发送邮件: {:?}", e),
    }

    Ok(())
}

pub fn get() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}

fn main() {
    // 获取本机的IP地址
    let body = format!("您好，您本机的局域网ip的地址为\n{}\n仅供参考", get().unwrap());
    let mut index = 0;
    loop {
        match send_email(&*body) {
            Ok(t) => {
                println!("程序退出:{:?}", t);
                break;
            }
            Err(e) => {
                println!("邮件发送失败:{}", e);
                index += 1;
                if index > 10 {
                    println!("发送{index}次，还是失败了:{}", e);
                    break;
                }
            }
        }
    }
}
