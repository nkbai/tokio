#![allow(clippy::cognitive_complexity)]
#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]
/// watch是一个single producer，多个consumer的模式，类似于listener
use tokio::sync::watch;
use tokio_test::task::spawn;
use tokio_test::{assert_pending, assert_ready};

#[test]
fn single_rx_recv() {
    let (tx, mut rx) = watch::channel("one");

    {
        let mut t = spawn(rx.recv());
        let v = assert_ready!(t.poll()).unwrap();
        assert_eq!(v, "one");
    }

    {
        let mut t = spawn(rx.recv());

        assert_pending!(t.poll());

        tx.broadcast("two").unwrap();

        assert!(t.is_woken());

        let v = assert_ready!(t.poll()).unwrap();
        assert_eq!(v, "two");
    }
    //如果发送放关闭，接收方立即ready，然后内容为空
    {
        let mut t = spawn(rx.recv());

        assert_pending!(t.poll());

        drop(tx);

        let res = assert_ready!(t.poll());
        assert!(res.is_none());
    }
}

#[test]
fn multi_rx() {
    let (tx, mut rx1) = watch::channel("one");
    let mut rx2 = rx1.clone();
    //一个发送方，多个接收方
    {
        let mut t1 = spawn(rx1.recv());
        let mut t2 = spawn(rx2.recv());

        let res = assert_ready!(t1.poll());
        assert_eq!(res.unwrap(), "one");

        let res = assert_ready!(t2.poll());
        assert_eq!(res.unwrap(), "one");
    }

    let mut t2 = spawn(rx2.recv());

    {
        let mut t1 = spawn(rx1.recv());

        assert_pending!(t1.poll());
        assert_pending!(t2.poll());

        tx.broadcast("two").unwrap();

        assert!(t1.is_woken());
        assert!(t2.is_woken());

        let res = assert_ready!(t1.poll());
        assert_eq!(res.unwrap(), "two");
    }

    {
        let mut t1 = spawn(rx1.recv());

        assert_pending!(t1.poll());

        tx.broadcast("three").unwrap();

        assert!(t1.is_woken());
        assert!(t2.is_woken());

        let res = assert_ready!(t1.poll());
        assert_eq!(res.unwrap(), "three");

        let res = assert_ready!(t2.poll());
        assert_eq!(res.unwrap(), "three");
    }

    drop(t2);

    {
        let mut t1 = spawn(rx1.recv());
        let mut t2 = spawn(rx2.recv());

        assert_pending!(t1.poll());
        assert_pending!(t2.poll());

        tx.broadcast("four").unwrap();

        let res = assert_ready!(t1.poll());
        assert_eq!(res.unwrap(), "four");
        drop(t1);

        let mut t1 = spawn(rx1.recv());
        assert_pending!(t1.poll());

        drop(tx);

        assert!(t1.is_woken());
        let res = assert_ready!(t1.poll());
        assert!(res.is_none());

        let res = assert_ready!(t2.poll());
        assert_eq!(res.unwrap(), "four");

        drop(t2);
        let mut t2 = spawn(rx2.recv());
        let res = assert_ready!(t2.poll());
        assert!(res.is_none());
    }
}

#[test]
fn rx_observes_final_value() {
    // Initial value

    let (tx, mut rx) = watch::channel("one");
    drop(tx);

    {
        //初始值是收听得到的，关闭后的第一个接收，能收到。
        let mut t1 = spawn(rx.recv());
        let res = assert_ready!(t1.poll());
        assert_eq!(res.unwrap(), "one");
    }

    {
        let mut t1 = spawn(rx.recv());
        let res = assert_ready!(t1.poll());
        assert!(res.is_none());
    }

    // Sending a value

    let (tx, mut rx) = watch::channel("one");

    tx.broadcast("two").unwrap();

    {
        //只能接收到到最后一次值？
        let mut rx2 = rx.clone();
        let mut t1 = spawn(rx2.recv());
        let res = assert_ready!(t1.poll());
        assert_eq!(res.unwrap(), "two");
        //启动以后会接收到最后一次的值
        let mut t2 = spawn(rx.recv());
        let res = assert_ready!(t2.poll());
        assert_eq!(res.unwrap(), "two");
    }

    {
        let mut t1 = spawn(rx.recv());
        assert_pending!(t1.poll());

        tx.broadcast("three").unwrap();
        drop(tx);

        assert!(t1.is_woken());

        let res = assert_ready!(t1.poll());
        assert_eq!(res.unwrap(), "three");
    }

    {
        let mut t1 = spawn(rx.recv());
        let res = assert_ready!(t1.poll());
        assert!(res.is_none());
    }
}

#[test]
fn poll_close() {
    let (mut tx, rx) = watch::channel("one");

    {
        let mut t = spawn(tx.closed());
        assert_pending!(t.poll());

        drop(rx);

        assert!(t.is_woken());
        assert_ready!(t.poll());
    }

    assert!(tx.broadcast("two").is_err());
}

#[test]
fn stream_impl() {
    use futures::StreamExt;

    let (tx, mut rx) = watch::channel("one");

    {
        let mut t = spawn(rx.next());
        let v = assert_ready!(t.poll()).unwrap();
        assert_eq!(v, "one");
    }

    {
        let mut t = spawn(rx.next());

        assert_pending!(t.poll());

        tx.broadcast("two").unwrap();

        assert!(t.is_woken());

        let v = assert_ready!(t.poll()).unwrap();
        assert_eq!(v, "two");
    }

    {
        let mut t = spawn(rx.next());

        assert_pending!(t.poll());

        drop(tx);

        let res = assert_ready!(t.poll());
        assert!(res.is_none());
    }
}
