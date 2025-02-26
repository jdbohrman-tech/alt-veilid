use crate::*;

pub async fn test_startup_shutdown() {
    info!("test_startup_shutdown");

    let bus = EventBus::new();

    // Normal case
    bus.startup().expect("should startup");
    bus.shutdown().await;

    // Startup fail case
    bus.startup().expect("should startup");
    bus.startup().expect_err("should not startup a second time");
    bus.shutdown().await;

    // Multiple shutdown case
    bus.startup().expect("should startup");
    bus.shutdown().await;
    bus.shutdown().await;
}

struct Evt1 {
    field1: i32,
}

struct Evt2 {
    field1: i32,
    field2: i64,
}

pub async fn test_post() {
    info!("test_post");

    let bus = EventBus::new();

    bus.post("test").expect_err("should fail");

    bus.startup().expect("should startup");
    bus.post("test").expect("should post");
    bus.post(Evt1 { field1: 0 }).expect("should post");
    bus.post(Evt2 {
        field1: 1,
        field2: 2,
    })
    .expect("should post");
    bus.shutdown().await;
}

pub async fn test_subscribe() {
    info!("test_subscribe");

    let bus = EventBus::new();

    // Okay to subscribe at any time
    bus.subscribe(|evt: Arc<Evt1>| {
        Box::pin(async move {
            assert_eq!(evt.field1, 0);
        })
    });

    // Startup keeps existing subscriptions
    bus.startup().expect("should startup");

    // Okay to subscribe at any time
    bus.subscribe(|evt: Arc<Evt2>| {
        Box::pin(async move {
            assert_eq!(evt.field1, 1);
            assert_eq!(evt.field2, 2);
        })
    });

    // Shutdown clears subscriptions
    bus.shutdown().await;
}

pub async fn test_post_subscribe() {
    info!("test_post_subscribe");

    let bus = EventBus::new();

    // Should calculate this total
    let total = Arc::new(AtomicUsize::new(0));

    // Okay to subscribe at any time
    let total2 = total.clone();
    bus.subscribe(move |evt: Arc<Evt1>| {
        let total = total2.clone();
        Box::pin(async move {
            assert_eq!(evt.field1, 0);

            total.fetch_add(1, Ordering::AcqRel);
        })
    });
    bus.post(Evt1 { field1: 0 }).expect_err("should fail");

    // Startup keeps existing subscriptions
    bus.startup().expect("should startup");

    bus.post("test").expect("should post");
    bus.post(Evt1 { field1: 0 }).expect("should post");
    bus.post(Evt2 {
        field1: 1,
        field2: 2,
    })
    .expect("should post");

    // Wait for all posts to process
    while !bus.is_empty() {
        sleep(250).await;
    }

    // Okay to subscribe at any time
    let total2 = total.clone();
    bus.subscribe(move |evt: Arc<Evt2>| {
        let total = total2.clone();
        Box::pin(async move {
            assert_eq!(evt.field1, 1);
            assert_eq!(evt.field2, 2);
            total.fetch_add(10, Ordering::AcqRel);
        })
    });

    bus.post("test").expect("should post");
    bus.post(Evt1 { field1: 0 }).expect("should post");
    bus.post(Evt2 {
        field1: 1,
        field2: 2,
    })
    .expect("should post");

    // Wait for all posts to process
    while !bus.is_empty() {
        sleep(250).await;
    }

    // Shutdown clears subscriptions
    bus.shutdown().await;

    // Should fail since we're shut down
    bus.post(Evt1 { field1: 0 }).expect_err("should fail");

    // Startup keeps existing subscriptions
    bus.startup().expect("should startup");

    // Succeeds but has no subscriptions
    bus.post(Evt1 { field1: 0 }).expect("should post");

    // Shutdown cleanly
    bus.shutdown().await;

    // Should have gotten this total
    // Two Evt1 processed, and one Evt2 processed
    assert_eq!(total.load(Ordering::Acquire), 12);
}

pub async fn test_all() {
    test_startup_shutdown().await;
    test_post().await;
    test_subscribe().await;
    test_post_subscribe().await;
}
