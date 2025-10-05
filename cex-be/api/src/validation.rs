use serde_json::json;
use log::warn;

#[derive(Debug, Clone)]
pub struct MarketConfig {
    pub base_asset: String,
    pub quote_asset: String,
    pub min_order_size: f64,
    pub max_order_size: f64,
    pub price_precision: u32,
    pub quantity_precision: u32,
    pub min_price: f64,
    pub max_price: f64,
}

pub struct OrderValidator {
    markets: std::collections::HashMap<String, MarketConfig>,
}

impl OrderValidator {
    pub fn new() -> Self {
        let mut markets = std::collections::HashMap::new();
        
        let market_configs = vec![
            MarketConfig {
                base_asset: "BTC".to_string(),
                quote_asset: "USD".to_string(),
                min_order_size: 0.001, 
                max_order_size: 100.0,  
                price_precision: 2,     
                quantity_precision: 8,  
                min_price: 0.01,
                max_price: 1_000_000.0,
            },
            MarketConfig {
                base_asset: "ETH".to_string(),
                quote_asset: "USD".to_string(),
                min_order_size: 0.01,   
                max_order_size: 1000.0, 
                price_precision: 2,     
                quantity_precision: 6,  
                min_price: 0.01,
                max_price: 100_000.0,
            },
            MarketConfig {
                base_asset: "BTC".to_string(),
                quote_asset: "USDT".to_string(),
                min_order_size: 0.001,
                max_order_size: 100.0, 
                price_precision: 2,    
                quantity_precision: 8, 
                min_price: 0.01,
                max_price: 1_000_000.0,
            },
            MarketConfig {
                base_asset: "ETH".to_string(),
                quote_asset: "USDT".to_string(),
                min_order_size: 0.01,  
                max_order_size: 1000.0,
                price_precision: 2,    
                quantity_precision: 6, 
                min_price: 0.01,
                max_price: 100_000.0,
            },
            MarketConfig {
                base_asset: "SOL".to_string(),
                quote_asset: "USD".to_string(),
                min_order_size: 0.1,   
                max_order_size: 10000.0,
                price_precision: 4,    
                quantity_precision: 4, 
                min_price: 0.0001,
                max_price: 1000.0,
            },
        ];

        for config in market_configs {
            let market = format!("{}-{}", config.base_asset, config.quote_asset);
            markets.insert(market, config);
        }

        Self { markets }
    }

    pub fn validate_order(&self, market: &str, price: f64, quantity: f64) -> Result<(), serde_json::Value> {
        let config = match self.markets.get(market) {
            Some(config) => config,
            None => {
                warn!("Invalid market: {}", market);
                return Err(json!({
                    "error": "Unsupported market",
                    "supported_markets": self.get_supported_markets()
                }));
            }
        };

        if price <= 0.0 {
            warn!("Invalid price: {}", price);
            return Err(json!({
                "error": "Price must be greater than 0"
            }));
        }

        if price < config.min_price {
            warn!("Price too low: {} < {}", price, config.min_price);
            return Err(json!({
                "error": format!("Price must be at least {}", config.min_price)
            }));
        }

        if price > config.max_price {
            warn!("Price too high: {} > {}", price, config.max_price);
            return Err(json!({
                "error": format!("Price must be at most {}", config.max_price)
            }));
        }

        if quantity <= 0.0 {
            warn!("Invalid quantity: {}", quantity);
            return Err(json!({
                "error": "Quantity must be greater than 0"
            }));
        }

        if quantity < config.min_order_size {
            warn!("Quantity too low: {} < {}", quantity, config.min_order_size);
            return Err(json!({
                "error": format!("Minimum order size is {}", config.min_order_size)
            }));
        }

        if quantity > config.max_order_size {
            warn!("Quantity too high: {} > {}", quantity, config.max_order_size);
            return Err(json!({
                "error": format!("Maximum order size is {}", config.max_order_size)
            }));
        }

        if !self.validate_precision(price, config.price_precision) {
            warn!("Invalid price precision: {} (expected {})", price, config.price_precision);
            return Err(json!({
                "error": format!("Price precision must be {} decimal places", config.price_precision)
            }));
        }

        if !self.validate_precision(quantity, config.quantity_precision) {
            warn!("Invalid quantity precision: {} (expected {})", quantity, config.quantity_precision);
            return Err(json!({
                "error": format!("Quantity precision must be {} decimal places", config.quantity_precision)
            }));
        }

        Ok(())
    }

    fn validate_precision(&self, value: f64, precision: u32) -> bool {
        let multiplier = 10_f64.powi(precision as i32);
        let rounded = (value * multiplier).round() / multiplier;
        (value - rounded).abs() < f64::EPSILON
    }

    pub fn get_supported_markets(&self) -> Vec<String> {
        self.markets.keys().cloned().collect()
    }

    pub fn get_market_config(&self, market: &str) -> Option<&MarketConfig> {
        self.markets.get(market)
    }
}

pub fn validate_market_format(market: &str) -> bool {
    let parts: Vec<&str> = market.split('-').collect();
    if parts.len() != 2 {
        return false;
    }
    
    let base = parts[0];
    let quote = parts[1];
    
    base.len() >= 3 && base.len() <= 10 && 
    quote.len() >= 3 && quote.len() <= 10 &&
    base.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) &&
    quote.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
}
