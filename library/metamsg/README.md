Note that BoringSSL, which is used to implement QUIC's cryptographic handshake based on TLS,
needs to be built and linked to quiche. This is done automatically when building quiche using cargo,
but requires the cmake command to be available during the build process.
On Windows you also need [NASM](https://www.nasm.us/pub/nasm/releasebuilds/2.15.05/win64/).
The official BoringSSL documentation has more details.