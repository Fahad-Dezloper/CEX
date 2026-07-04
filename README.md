# CEX - Centralized Exchange on Solana

A high-performance centralized cryptocurrency exchange built on Solana with MPC (Multi-Party Computation) wallet management. This exchange provides real-time order matching, depth management, and trade execution with low latency and high throughput.

## 🎯 Project Overview

This is a professional-grade centralized exchange (CEX) that provides:
- **Real-time order matching** using an in-memory matching engine
- **MPC-based wallet management** for enhanced security
- **WebSocket real-time updates** for trades, order depth, and market data
- **High-performance architecture** with async processing and Redis pub/sub
- **TimescaleDB** for time-series data storage (trades, orders, klines)
- **Multiple market support** with configurable precision and limits

## 🏗️ Architecture

The exchange follows a microservices architecture with four main services:

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Client    │────▶│  API Server │────▶│   Engine    │────▶│    WS       │
│  (Browser)  │◀────│  (REST)     │◀────│  (Matching) │────▶│  (Real-time)│
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
                           │                    │                    │
                           ▼                    ▼                    ▼
                    ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
                    │   Redis     │     │   Redis     │     │   Redis     │
                    │   (Queue)   │     │  (Pub/Sub)  │     │   (DB Q)    │
                    └─────────────┘     └─────────────┘     └─────────────┘
                                                 │
                                                 ▼
                                          ┌─────────────┐
                                          │  Database   │
                                          │ (Timescale) │
                                          └─────────────┘
```

### Data Flow

1. **User submits order** → API validates and queues to Engine
2. **Engine processes** → Matches order, updates balances, creates fills
3. **Real-time updates** → Engine publishes to Redis pub/sub → WS broadcasts to clients
4. **Persistence** → Engine queues to DB processor → TimescaleDB stores trades/orders

## 📁 Project Structure

```
CEX/
├── cex-be/                          # Backend services (Rust)
│   ├── api/                         # REST API Server
│   │   └── src/
│   │       ├── main.rs             # Server entry point (port 3010)
│   │       ├── redismanager.rs     # Redis client for API-Engine communication
│   │       ├── auth_service.rs     # JWT authentication logic
│   │       ├── middleware.rs       # Auth middleware, request validation
│   │       ├── validation.rs       # Order validation, market checks
│   │       ├── types.rs            # Request/response types
│   │       └── routes/             # API endpoints
│   │           ├── auth.rs         # /api/v1/auth - login, register
│   │           ├── order.rs        # /api/v1/order - create, cancel, get open orders
│   │           ├── markets.rs      # /api/v1/markets - list markets
│   │           ├── depth.rs        # /api/v1/depth - order book snapshot
│   │           ├── trades.rs       # /api/v1/trades - trade history
│   │           ├── ticker.rs       # /api/v1/tickers - 24h stats
│   │           └── klines.rs       # /api/v1/klines - OHLCV candles
│   │
│   ├── engine/                      # Matching Engine
│   │   └── src/
│   │       ├── main.rs             # Engine entry point - listens to Redis queue
│   │       ├── engine.rs           # Core matching logic (705 lines)
│   │       ├── orderbook.rs        # Order book data structure (bids/asks)
│   │       ├── redis_manager.rs    # Redis clients (3 instances: queue, pubsub, db)
│   │       └── types.rs            # Internal message types
│   │
│   ├── ws/                          # WebSocket Server
│   │   └── src/
│   │       ├── main.rs             # WS server entry point (port 8000)
│   │       ├── subscription_manager.rs  # Manages user subscriptions to channels
│   │       ├── user_manager.rs     # Maps user IDs to WebSocket connections
│   │       ├── user.rs             # User connection state
│   │       └── types.rs            # WS message types
│   │
│   ├── db/                          # Database Layer
│   │   └── src/
│   │       ├── lib.rs              # DB pool, message processing
│   │       ├── schema.rs           # Diesel ORM schema definitions
│   │       ├── model.rs            # Database models (User, Trade, Order, Market, UserAsset)
│   │       └── start/
│   │           └── db.rs           # DB processor main - consumes db_processor queue
│   │
│   ├── docker/                      # Docker configuration
│   │   ├── docker-compose.yml      # TimescaleDB + Redis containers
│   │   └── clear_data.sh           # Script to truncate tables and flush Redis
│   │
│   ├── env.example                  # Environment variables template
│   └── Cargo.toml                  # Workspace configuration
│
├── cex-fe/                          # Frontend (Next.js + TypeScript)
│   ├── app/
│   │   ├── page.tsx                # Landing page
│   │   ├── login/                  # Authentication pages
│   │   ├── trade/[market]/         # Dynamic trading page for each market
│   │   ├── components/
│   │   │   ├── TradeView.tsx       # Main trading interface
│   │   │   ├── SwapUI.tsx          # Swap interface
│   │   │   ├── MarketBar.tsx       # Market selector
│   │   │   ├── depth/              # Order book depth components
│   │   │   └── home/Trades.tsx     # Recent trades display
│   │   ├── context/
│   │   │   ├── MarketContext.tsx   # Global market state
│   │   │   └── UserContext.tsx     # User authentication state
│   │   └── utils/
│   │       ├── httpClient.ts       # Axios wrapper for API calls
│   │       ├── wsClient.ts         # WebSocket client wrapper
│   │       ├── ChartManager.ts     # Trading chart integration
│   │       └── types.ts            # TypeScript type definitions
│   └── components/ui/               # Reusable UI components (shadcn)
│
└── README.md                        # This file
```

## 🔧 Services Overview

### 1. **API Server** (`cex-be/api/`)
- **Purpose**: HTTP REST API for all client requests
- **Port**: 3010
- **Tech**: Rust (Poem web framework)
- **Key Responsibilities**:
  - User authentication (JWT)
  - Order submission and validation
  - Market data retrieval
  - Communicates with Engine via Redis queue (`messages`)
  - CORS enabled for frontend integration

### 2. **Matching Engine** (`cex-be/engine/`)
- **Purpose**: Core order matching and trade execution
- **Tech**: Rust
- **Key Responsibilities**:
  - Listens to Redis queue for orders
  - Maintains in-memory order books per market
  - Matches buy/sell orders (price-time priority)
  - Manages user balances (available/locked)
  - Publishes real-time updates to WS via Redis pub/sub
  - Queues persistence events to DB processor
  - Supports: CREATE_ORDER, CANCEL_ORDER, GET_DEPTH, GET_OPEN_ORDERS

### 3. **WebSocket Server** (`cex-be/ws/`)
- **Purpose**: Real-time data streaming to clients
- **Port**: 8000
- **Tech**: Rust (tokio-tungstenite)
- **Key Responsibilities**:
  - Manages WebSocket connections per user
  - Subscribes to Redis channels (e.g., `trade@BTC-USD`, `depth@BTC-USD`)
  - Broadcasts trades, depth updates to subscribed clients
  - Handles subscription/unsubscription logic

### 4. **Database** (`cex-be/db/`)
- **Purpose**: Data persistence and time-series storage
- **Tech**: TimescaleDB (PostgreSQL extension), Diesel ORM
- **Key Responsibilities**:
  - Consumes DB queue from Engine
  - Stores trades, orders, market data
  - Provides OHLCV data for charts (klines)
  - Used by API for historical queries

### 5. **Frontend** (`cex-fe/`)
- **Purpose**: User-facing trading interface
- **Tech**: Next.js 14, TypeScript, Tailwind CSS
- **Features**:
  - Trading view with order book, charts, trade history
  - User authentication
  - Real-time market data via WebSocket
  - Responsive design

## 🔄 Message Flow Examples

### Creating an Order
```
1. Client → POST /api/v1/order → API Server
2. API validates order (price, quantity, market, user balance)
3. API → LPUSH "messages" → Redis Queue
4. Engine ← BRPOP "messages" ← Redis Queue
5. Engine matches order against orderbook
6. Engine → PUBLISH "trade@BTC-USD" → Redis Pub/Sub
7. Engine → RPUSH "db_events" → Redis DB Queue
8. WS ← Receives pub/sub message ← Redis
9. WS → Broadcasts to all subscribed clients
10. Client ← Receives trade update via WebSocket
```

### Getting Order Depth
```
1. Client → GET /api/v1/depth?market=BTC-USD → API
2. API → LPUSH "messages" → Redis Queue
3. Engine pops message, queries in-memory orderbook
4. Engine → PUBLISH response to API's unique channel
5. API receives response, returns to client
```

## 🗄️ Database Schema

### Core Tables
- **users**: User accounts with encrypted JWT tokens
- **markets**: Supported trading pairs (e.g., BTC-USD) with precision/limits
- **orders**: Historical and active orders
- **trades**: Executed trades with fill details
- **user_assets**: User balances per asset (base/quote balances)

## 🔐 Security Features

- **JWT Authentication**: All protected endpoints require valid JWT token
- **Order Validation**: Price/quantity validation before processing
- **Balance Checks**: Engine verifies sufficient funds before locking
- **MPC Wallet Management**: (Planned) Secure multi-party computation for private keys
- **Rate Limiting**: (Planned) Prevent abuse on public endpoints

## 🚀 Setup Instructions

<!-- TODO: Add detailed setup instructions -->
- [ ] Install Rust toolchain
- [ ] Install Node.js and npm
- [ ] Set up TimescaleDB
- [ ] Configure Redis instances
- [ ] Set environment variables
- [ ] Run database migrations
- [ ] Start backend services
- [ ] Start frontend development server

## 📊 Performance Characteristics

- **Latency**: Sub-millisecond order matching
- **Throughput**: Handles thousands of orders per second
- **Order Book**: In-memory for fast matching, updates published in real-time
- **Persistence**: Async queue-based for non-blocking writes

## 🛠️ Technologies Used

**Backend:**
- Rust (async with Tokio)
- Redis (queues, pub/sub, caching)
- TimescaleDB (time-series data)
- Diesel ORM (type-safe SQL)
- Poem (REST API framework)
- JWT (authentication)

**Frontend:**
- Next.js 14 (App Router)
- TypeScript
- Tailwind CSS
- Recharts/TradingView (charts)
- WebSocket (real-time updates)

## 📝 Development Status

- ✅ Core matching engine implemented
- ✅ REST API endpoints functional
- ✅ WebSocket real-time updates working
- ✅ Frontend trading interface in progress
- ⏳ MPC wallet integration (planned)
- ⏳ Advanced order types (limit, stop-loss) (planned)
- ⏳ Admin dashboard (planned)

## 🤝 Contributing

This is a developer-focused project. Contributions welcome!

## 📄 License

[Your License Here]


