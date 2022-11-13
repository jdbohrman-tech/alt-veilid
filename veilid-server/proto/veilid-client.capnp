@0xd29582d26b2fb073;

struct ApiResult @0x8111724bdb812929 {
    union {
        ok                  @0  :Text;
        err                 @1  :Text;
    }
}

interface Registration @0xdd45f30a7c22e391 {}

interface VeilidServer @0xcb2c699f14537f94 {
    register @0 (veilidClient :VeilidClient) -> (registration :Registration, state :Text);
    debug @1 (command :Text) -> (result :ApiResult);
    attach @2 () -> (result :ApiResult);
    detach @3 () -> (result :ApiResult);
    shutdown @4 ();
    getState @5 () -> (result :ApiResult);
    changeLogLevel @6 (layer :Text, logLevel :Text) -> (result :ApiResult);
    appCallReply @7 (id :UInt64, message :Data) -> (result :ApiResult);
}

interface VeilidClient @0xbfcea60fb2ba4736 {
    update @0 (veilidUpdate :Text);
}