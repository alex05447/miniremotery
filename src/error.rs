use {
    std::convert::TryFrom,
    num_enum::TryFromPrimitive
};

#[allow(non_camel_case_types)]
#[derive(TryFromPrimitive)] // num_enum
#[derive(EnumMessage)] // strum
#[repr(u32)]
pub enum rmtError {
    // >>> START REMOTERY ERRORS
    #[strum(message = "No error")]
    RMT_ERROR_NONE = 0,
    #[strum(message = "Not an error but an internal message to calling code")]
    RMT_ERROR_RECURSIVE_SAMPLE = 1,

    // System errors
    #[strum(message = "Malloc call within remotery failed")]
    RMT_ERROR_MALLOC_FAIL = 2,
    #[strum(message = "Attempt to allocate thread local storage failed")]
    RMT_ERROR_TLS_ALLOC_FAIL = 3,
    #[strum(message = "Failed to create a virtual memory mirror buffer")]
    RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL = 4,
    #[strum(message = "Failed to create a thread for the server")]
    RMT_ERROR_CREATE_THREAD_FAIL = 5,

    // Network TCP/IP socket errors
    #[strum(message = "Network initialisation failure (e.g. on Win32, WSAStartup fails)")]
    RMT_ERROR_SOCKET_INIT_NETWORK_FAIL = 6,
    #[strum(message = "Can't create a socket for connection to the remote viewer")]
    RMT_ERROR_SOCKET_CREATE_FAIL = 7,
    #[strum(message = "Can't bind a socket for the server")]
    RMT_ERROR_SOCKET_BIND_FAIL = 8,
    #[strum(message = "Created server socket failed to enter a listen state")]
    RMT_ERROR_SOCKET_LISTEN_FAIL = 9,
    #[strum(message = "Created server socket failed to switch to a non-blocking state")]
    RMT_ERROR_SOCKET_SET_NON_BLOCKING_FAIL = 10,
    #[strum(message = "Poll attempt on an invalid socket")]
    RMT_ERROR_SOCKET_INVALID_POLL = 11,
    #[strum(message = "Server failed to call select on socket")]
    RMT_ERROR_SOCKET_SELECT_FAIL = 12,
    #[strum(message = "Poll notified that the socket has errors")]
    RMT_ERROR_SOCKET_POLL_ERRORS = 13,
    #[strum(message = "Server failed to accept connection from client")]
    RMT_ERROR_SOCKET_ACCEPT_FAIL = 14,
    #[strum(message = "Timed out trying to send data")]
    RMT_ERROR_SOCKET_SEND_TIMEOUT = 15,
    #[strum(message = "Unrecoverable error occured while client/server tried to send data")]
    RMT_ERROR_SOCKET_SEND_FAIL = 16,
    #[strum(message = "No data available when attempting a receive")]
    RMT_ERROR_SOCKET_RECV_NO_DATA = 17,
    #[strum(message = "Timed out trying to receive data")]
    RMT_ERROR_SOCKET_RECV_TIMEOUT = 18,
    #[strum(message = "Unrecoverable error occured while client/server tried to receive data")]
    RMT_ERROR_SOCKET_RECV_FAILED = 19,

    // WebSocket errors
    #[strum(message = "WebSocket server handshake failed, not HTTP GET")]
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NOT_GET = 20,
    #[strum(message = "WebSocket server handshake failed, can't locate WebSocket version")]
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_VERSION = 21,
    #[strum(message = "WebSocket server handshake failed, unsupported WebSocket version")]
    RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_VERSION = 22,
    #[strum(message = "WebSocket server handshake failed, can't locate host")]
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_HOST = 23,
    #[strum(message = "WebSocket server handshake failed, host is not allowed to connect")]
    RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_HOST = 24,
    #[strum(message = "WebSocket server handshake failed, can't locate WebSocket key")]
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_KEY = 25,
    #[strum(message = "WebSocket server handshake failed, WebSocket key is ill-formed")]
    RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_KEY = 26,
    #[strum(message = "WebSocket server handshake failed, internal error, bad string code")]
    RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL = 27,
    #[strum(message = "WebSocket server received a disconnect request and closed the socket")]
    RMT_ERROR_WEBSOCKET_DISCONNECTED = 28,
    #[strum(message = "Couldn't parse WebSocket frame header")]
    RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER = 29,
    #[strum(message = "Partially received wide frame header size")]
    RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER_SIZE = 30,
    #[strum(message = "Partially received frame header data mask")]
    RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER_MASK = 31,
    #[strum(message = "Timeout receiving frame header")]
    RMT_ERROR_WEBSOCKET_RECEIVE_TIMEOUT = 32,

    #[strum(message = "Remotery object has not been created")]
    RMT_ERROR_REMOTERY_NOT_CREATED = 33,
    #[strum(message = "An attempt was made to send an incomplete profile tree to the client")]
    RMT_ERROR_SEND_ON_INCOMPLETE_PROFILE = 34,

    // CUDA error messages
    #[strum(message = "This indicates that the CUDA driver is in the process of shutting down")]
    RMT_ERROR_CUDA_DEINITIALIZED = 35,
    #[strum(message = "This indicates that the CUDA driver has not been initialized with cuInit() or that initialization has failed")]
    RMT_ERROR_CUDA_NOT_INITIALIZED = 36,
    #[strum(message = "This most frequently indicates that there is no context bound to the current thread")]
    RMT_ERROR_CUDA_INVALID_CONTEXT = 37,
    #[strum(message = "This indicates that one or more of the parameters passed to the API call is not within an acceptable range of values")]
    RMT_ERROR_CUDA_INVALID_VALUE = 38,
    #[strum(message = "This indicates that a resource handle passed to the API call was not valid")]
    RMT_ERROR_CUDA_INVALID_HANDLE = 39,
    #[strum(message = "The API call failed because it was unable to allocate enough memory to perform the requested operation")]
    RMT_ERROR_CUDA_OUT_OF_MEMORY = 40,
    #[strum(message = "This indicates that a resource handle passed to the API call was not valid")]
    RMT_ERROR_ERROR_NOT_READY = 41,

    // Direct3D 11 error messages
    #[strum(message = "Failed to create query for sample")]
    RMT_ERROR_D3D11_FAILED_TO_CREATE_QUERY = 42,

    // OpenGL error messages
    #[strum(message = "Generic OpenGL error, no need to expose detail since app will need an OpenGL error callback registered")]
    RMT_ERROR_OPENGL_ERROR = 43,

    #[strum(message = "Unknown CUDA error")]
    RMT_ERROR_CUDA_UNKNOWN = 44,

    // <<< END REMOTERY ERRORS

    // Added for `miniremotery`.
    #[strum(message = "Unknown error")]
    RMT_ERROR_UNKNOWN = 45,
}

#[allow(non_snake_case)]
pub(crate) fn u32_to_rmtError(error: u32) -> rmtError {
    rmtError::try_from(error).unwrap_or(rmtError::RMT_ERROR_UNKNOWN)
}
