@0xd29582d26b2fb073;

struct ApiResult {
    union {
        ok                  @0  :Text;
        err                 @1  :Text;
    }
}

interface Registration {}

interface VeilidServer {
    register @0 (veilidClient :VeilidClient) -> (registration :Registration, state :Text);
    debug @1 (command :Text) -> (result :ApiResult);
    attach @2 () -> (result :ApiResult);
    detach @3 () -> (result :ApiResult);
    shutdown @4 ();
    getState @5 () -> (result :ApiResult);
    changeLogLevel @6 (layer :Text, logLevel :Text) -> (result :ApiResult);
    appCallReply @7 (id :UInt64, message :Data) -> (result :ApiResult);
}

interface VeilidClient {
    update @0 (veilidUpdate :Text);
}