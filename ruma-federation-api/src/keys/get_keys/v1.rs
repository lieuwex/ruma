//! [POST /_matrix/federation/v1/user/keys/query](https://matrix.org/docs/spec/server_server/r0.1.4#post-matrix-federation-v1-user-keys-claim)

use std::collections::BTreeMap;

use ruma_api::ruma_api;
use ruma_common::encryption::IncomingDeviceKeys;
use ruma_identifiers::{DeviceIdBox, UserId};

ruma_api! {
    metadata: {
        description: "Returns the current devices and identity keys for the given users.",
        method: POST,
        name: "get_keys",
        path: "/_matrix/federation/v1/user/keys/query",
        rate_limited: false,
        requires_authentication: true,
    }

    #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
    request: {
        /// The keys to be downloaded. Gives all keys for a given user if the list of device ids is
        /// empty.
        pub device_keys: BTreeMap<UserId, Vec<DeviceIdBox>>,
    }

    #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
    response: {
        /// Keys from the queried devices.
        pub device_keys: BTreeMap<UserId, BTreeMap<DeviceIdBox, IncomingDeviceKeys>>,
    }
}

impl Request {
    /// Creates a new `Request` asking for the given device keys.
    pub fn new(device_keys: BTreeMap<UserId, Vec<DeviceIdBox>>) -> Self {
        Self { device_keys }
    }
}

impl Response {
    /// Creates a new `Response` with the given device keys.
    pub fn new(device_keys: BTreeMap<UserId, BTreeMap<DeviceIdBox, IncomingDeviceKeys>>) -> Self {
        Self { device_keys }
    }
}
