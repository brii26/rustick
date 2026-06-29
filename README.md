# rustick 🦀

> Real-time market data streaming server built with Rust and Tokio.

## Overview

Rustick simulates a market data pipeline similar to what securities firms operate in production. A price generator broadcasts fake tick data via UDP, a feed handler normalizes and distributes it, and multiple clients receive real-time price updates over WebSocket, all built on Tokio's async runtime.

```
Price Generator => (UDP) => Feed Handler => (WebSocket) => Clients
```