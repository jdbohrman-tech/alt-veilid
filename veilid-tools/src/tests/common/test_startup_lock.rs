use crate::*;

pub async fn test_startup_shutdown() {
    info!("test_startup_shutdown");

    let lock = StartupLock::new();

    // Normal case
    {
        let guard = lock.startup().expect("should startup");
        guard.success();
    }
    assert!(lock.is_started());
    assert!(!lock.is_shut_down());

    {
        let guard = lock.shutdown().await.expect("should shutdown");
        guard.success();
    }
    assert!(!lock.is_started());
    assert!(lock.is_shut_down());

    // Startup fail case
    {
        lock.startup().expect("should startup");
        // Don't call success()
    }
    assert!(!lock.is_started());
    {
        lock.shutdown().await.expect_err("should not shutdown");
    }
    assert!(!lock.is_started());

    // Shutdown fail case
    {
        let guard = lock.startup().expect("should startup");
        guard.success();
    }
    assert!(lock.is_started());
    {
        lock.shutdown().await.expect("should shutdown");
        // Don't call success()
    }
    assert!(lock.is_started());
    {
        let guard = lock.shutdown().await.expect("should shutdown");
        guard.success();
    }
    assert!(!lock.is_started());
}

pub async fn test_contention() {
    info!("test_contention");

    let lock = Arc::new(StartupLock::new());
    let val = Arc::new(AtomicBool::new(false));

    {
        let guard = lock.startup().expect("should startup");
        guard.success();
    }
    assert!(lock.is_started());
    let lock2 = lock.clone();
    let val2 = val.clone();
    let jh = spawn("task", async move {
        let _guard = lock2.enter().expect("should enter");
        sleep(2000).await;
        val2.store(true, Ordering::Release);
    });
    sleep(1000).await;
    {
        let guard = lock.shutdown().await.expect("should shutdown");
        assert!(
            val.load(Ordering::Acquire),
            "should have waited for enter to exit"
        );
        guard.success();
    }
    assert!(!lock.is_started());
    jh.await;
}

pub async fn test_bad_enter() {
    info!("test_bad_enter");

    let lock = Arc::new(StartupLock::new());

    lock.enter()
        .expect_err("should not enter when not started up");
    {
        let guard = lock.startup().expect("should startup");
        guard.success();
    }
    assert!(lock.is_started());
    assert!(!lock.is_shut_down());

    let lock2 = lock.clone();
    let jh = spawn("task", async move {
        let guard = lock2.shutdown().await.expect("should shutdown");
        sleep(2000).await;
        guard.success();
    });
    sleep(1000).await;
    assert!(!lock.is_started());
    assert!(!lock.is_shut_down());

    lock.enter()
        .expect_err("should not enter when shutting down");

    jh.await;

    assert!(!lock.is_started());
    assert!(lock.is_shut_down());

    lock.enter().expect_err("should not enter when shut down");
}

pub async fn test_multiple_enter() {
    info!("test_multiple_enter");

    let lock = Arc::new(StartupLock::new());

    let s1 = lock.startup().expect("should startup");
    s1.success();

    {
        let _e1 = lock.enter().expect("should enter 1");
        {
            let _e2 = lock.enter().expect("should enter 2");
            {
                let _e3 = lock.enter().expect("should enter 3");
            }
        }
    }

    let e4 = lock.enter().expect("should enter 4");
    let e5 = lock.enter().expect("should enter 5");
    let e6 = lock.enter().expect("should enter 6");

    //eprintln!("1");
    let lock2 = lock.clone();
    let jh = spawn("task", async move {
        //eprintln!("2");
        let guard = lock2.shutdown().await.expect("should shutdown");
        //eprintln!("7");
        sleep(2000).await;
        //eprintln!("8");
        guard.success();
    });
    sleep(1000).await;
    //eprintln!("3");
    assert!(!lock.is_started());
    assert!(!lock.is_shut_down());

    // Now drop the enter created before shutdown
    drop(e4);
    //eprintln!("4");
    drop(e5);
    //eprintln!("5");
    drop(e6);
    //eprintln!("6");

    // This should finally exit
    jh.await;

    //eprintln!("9");

    assert!(!lock.is_started());
    assert!(lock.is_shut_down());

    lock.enter().expect_err("should not enter");
}

pub async fn test_all() {
    test_startup_shutdown().await;
    test_contention().await;
    test_bad_enter().await;
    test_multiple_enter().await;
}
