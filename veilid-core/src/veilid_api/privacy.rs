use super::*;

/////////////////////////////////////////////////////////////////////////////////////////////////////
// Privacy Specs

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RouteHopSpec {
    pub dial_info: NodeDialInfo,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PrivateRouteSpec {
    //
    pub public_key: DHTKey,
    pub secret_key: DHTKeySecret,
    pub hops: Vec<RouteHopSpec>,
}

impl PrivateRouteSpec {
    pub fn new() -> Self {
        let (pk, sk) = generate_secret();
        PrivateRouteSpec {
            public_key: pk,
            secret_key: sk,
            hops: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SafetyRouteSpec {
    pub public_key: DHTKey,
    pub secret_key: DHTKeySecret,
    pub hops: Vec<RouteHopSpec>,
}

impl SafetyRouteSpec {
    pub fn new() -> Self {
        let (pk, sk) = generate_secret();
        SafetyRouteSpec {
            public_key: pk,
            secret_key: sk,
            hops: Vec::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Compiled Privacy Objects

#[derive(Clone, Debug)]
pub struct RouteHopData {
    pub nonce: Nonce,
    pub blob: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct RouteHop {
    pub dial_info: NodeDialInfo,
    pub next_hop: Option<RouteHopData>,
}

#[derive(Clone, Debug)]
pub struct PrivateRoute {
    pub public_key: DHTKey,
    pub hop_count: u8,
    pub hops: Option<RouteHop>,
}

impl PrivateRoute {
    pub fn new_stub(public_key: DHTKey) -> Self {
        Self {
            public_key,
            hop_count: 0,
            hops: None,
        }
    }
}

impl fmt::Display for PrivateRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PR({:?}+{}{})",
            self.public_key,
            self.hop_count,
            if let Some(hops) = &self.hops {
                format!("->{}", hops.dial_info)
            } else {
                "".to_owned()
            }
        )
    }
}

#[derive(Clone, Debug)]
pub enum SafetyRouteHops {
    Data(RouteHopData),
    Private(PrivateRoute),
}

#[derive(Clone, Debug)]
pub struct SafetyRoute {
    pub public_key: DHTKey,
    pub hop_count: u8,
    pub hops: SafetyRouteHops,
}

impl fmt::Display for SafetyRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SR({:?}+{}{})",
            self.public_key,
            self.hop_count,
            match &self.hops {
                SafetyRouteHops::Data(_) => "".to_owned(),
                SafetyRouteHops::Private(p) => format!("->{}", p),
            }
        )
    }
}

// xxx impl to_blob and from_blob using capnp here
