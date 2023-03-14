use std::io::prelude::*;
use ssh2::{Session, Channel};

pub fn ssh_test() {
    // SSHセッションを開始する
    let tcp = std::net::TcpStream::connect("pi2.local:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // ユーザー名とパスワードを使用して認証する
    sess.userauth_password("pi", "Umgsq313").unwrap();

    // コマンドを発行する
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls -la").unwrap();

    // 結果を読み取る
    let mut buffer = String::new();
    channel.read_to_string(&mut buffer).unwrap();

    // 結果を表示する
    println!("{}", buffer);

    // セッションを終了する
    channel.wait_close();
    //channel.close().unwrap();
    //sess.disconnect(None).unwrap();
}
