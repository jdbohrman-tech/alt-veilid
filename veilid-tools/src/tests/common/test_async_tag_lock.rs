use crate::*;

pub async fn test_simple_no_contention() {
    info!("test_simple_no_contention");

    let table = AsyncTagLockTable::new();

    let a1 = SocketAddr::new("1.2.3.4".parse().unwrap(), 1234);
    let a2 = SocketAddr::new("6.9.6.9".parse().unwrap(), 6969);

    {
        let g1 = table.lock_tag(a1).await;
        let g2 = table.lock_tag(a2).await;
        drop(g2);
        drop(g1);
    }

    {
        let g1 = table.lock_tag(a1).await;
        let g2 = table.lock_tag(a2).await;
        drop(g1);
        drop(g2);
    }

    assert_eq!(table.len(), 0);
}

pub async fn test_simple_single_contention() {
    info!("test_simple_single_contention");

    let table = AsyncTagLockTable::new();

    let a1 = SocketAddr::new("1.2.3.4".parse().unwrap(), 1234);

    let g1 = table.lock_tag(a1).await;

    info!("locked");
    let t1 = spawn(async move {
        // move the guard into the task
        let _g1_take = g1;
        // hold the guard for a bit
        info!("waiting");
        sleep(1000).await;
        // release the guard
        info!("released");
    });

    // wait to lock again, will contend until spawned task exits
    let _g1_b = table.lock_tag(a1).await;
    info!("locked");

    // Ensure task is joined
    t1.await;

    assert_eq!(table.len(), 1);
}

pub async fn test_simple_try() {
    info!("test_simple_try");

    let table = AsyncTagLockTable::new();

    let a1 = SocketAddr::new("1.2.3.4".parse().unwrap(), 1234);
    let a2 = SocketAddr::new("1.2.3.5".parse().unwrap(), 1235);

    {
        let _g1 = table.lock_tag(a1).await;

        let opt_g2 = table.try_lock_tag(a1);
        let opt_g3 = table.try_lock_tag(a2);

        assert!(opt_g2.is_none());
        assert!(opt_g3.is_some());
    }
    let opt_g4 = table.try_lock_tag(a1);
    assert!(opt_g4.is_some());

    assert_eq!(table.len(), 1);
}

pub async fn test_simple_double_contention() {
    info!("test_simple_double_contention");

    let table = AsyncTagLockTable::new();

    let a1 = SocketAddr::new("1.2.3.4".parse().unwrap(), 1234);
    let a2 = SocketAddr::new("6.9.6.9".parse().unwrap(), 6969);

    let g1 = table.lock_tag(a1).await;
    let g2 = table.lock_tag(a2).await;

    info!("locked");
    let t1 = spawn(async move {
        // move the guard into the tas
        let _g1_take = g1;
        // hold the guard for a bit
        info!("waiting");
        sleep(1000).await;
        // release the guard
        info!("released");
    });
    let t2 = spawn(async move {
        // move the guard into the task
        let _g2_take = g2;
        // hold the guard for a bit
        info!("waiting");
        sleep(500).await;
        // release the guard
        info!("released");
    });

    // wait to lock again, will contend until spawned task exits
    let _g1_b = table.lock_tag(a1).await;
    // wait to lock again, should complete immediately
    let _g2_b = table.lock_tag(a2).await;

    info!("locked");

    // Ensure tasks are joined
    t1.await;
    t2.await;

    assert_eq!(table.len(), 2);
}

pub async fn test_parallel_single_contention() {
    info!("test_parallel_single_contention");

    let table = AsyncTagLockTable::new();

    let a1 = SocketAddr::new("1.2.3.4".parse().unwrap(), 1234);

    let table1 = table.clone();
    let t1 = spawn(async move {
        // lock the tag
        let _g = table1.lock_tag(a1).await;
        info!("locked t1");
        // hold the guard for a bit
        info!("waiting t1");
        sleep(500).await;
        // release the guard
        info!("released t1");
    });

    let table2 = table.clone();
    let t2 = spawn(async move {
        // lock the tag
        let _g = table2.lock_tag(a1).await;
        info!("locked t2");
        // hold the guard for a bit
        info!("waiting t2");
        sleep(500).await;
        // release the guard
        info!("released t2");
    });

    let table3 = table.clone();
    let t3 = spawn(async move {
        // lock the tag
        let _g = table3.lock_tag(a1).await;
        info!("locked t3");
        // hold the guard for a bit
        info!("waiting t3");
        sleep(500).await;
        // release the guard
        info!("released t3");
    });

    // Ensure tasks are joined
    t1.await;
    t2.await;
    t3.await;

    assert_eq!(table.len(), 0);
}

pub async fn test_all() {
    test_simple_no_contention().await;
    test_simple_try().await;
    test_simple_single_contention().await;
    test_parallel_single_contention().await;
}
