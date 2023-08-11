mod lp_pool;

const FIXED_POINT_SCALE: u64 = 1_000_000;

fn to_fixed_point(value: f64) -> u64 {
    (value * FIXED_POINT_SCALE as f64) as u64
}

fn from_fixed_point(value: u64) -> f64 {
    value as f64 / FIXED_POINT_SCALE as f64
}

fn main() {
    use lp_pool::{LpPool, Price, Percentage, TokenAmount, StakedTokenAmount, LpTokenAmount, Errors};

    let price = Price((1.5 * lp_pool::FIXED_POINT_SCALE as f64) as u64);
    let min_fee = Percentage(to_fixed_point(0.1));
    let max_fee = Percentage(to_fixed_point(90.0));
    let liquidity_target = TokenAmount(to_fixed_point(900.0));

    let mut lp_pool = match LpPool::init(price, min_fee, max_fee, liquidity_target) {
        Ok(pool) => pool,
        Err(_) => {
            println!("Failed to initialize LpPool");
            return;
        }
    };

    match lp_pool.add_liquidity(TokenAmount(to_fixed_point(100.0))) {
        Ok(lp_tokens) => {
            println!("Successfully added liquidity. Received {} LP tokens.", from_fixed_point(lp_tokens.0));
        }
        Err(error) => {
            match error {
                Errors::InvalidTokenAmount => {
                    println!("Invalid token amount provided for adding liquidity.");
                }
                _ => {
                    println!("Failed to add liquidity");
                }
            }
        }
    };

    match lp_pool.swap(StakedTokenAmount(6)) {
        Ok(received_tokens) => {
            println!("Successfully swapped staked tokens. Received {} tokens.", from_fixed_point(received_tokens.0));
        }
        Err(error) => {
            match error {
                Errors::InvalidStakedTokenAmount  => {
                    println!("Invalid staked token amount provided for swap.");
                }
                Errors::InsufficientLiquidity => {
                    println!("Insufficient liquidity for swap.");
                }
                _ => {
                    println!("Failed to swap");
                }
            }
        }
    }

    match lp_pool.add_liquidity(TokenAmount(to_fixed_point(10.0))) {
        Ok(lp_tokens) => {
            println!("Successfully added liquidity. Received {} LP tokens.", from_fixed_point(lp_tokens.0));
        }
        Err(error) => {
            match error {
                Errors::InvalidTokenAmount => {
                    println!("Invalid token amount provided for adding liquidity.");
                }
                Errors::InsufficientLiquidity => {
                    println!("Insufficient liquidity to mint LP tokens.");
                }
                _ => {
                    println!("Failed to add liquidity");
                }
            }
        }
    };

    match lp_pool.swap(StakedTokenAmount(30)) {
        Ok(received_tokens) => {
            println!("Successfully swapped staked tokens. Received {} tokens.", from_fixed_point(received_tokens.0));
        }
        Err(error) => {
            match error {
                Errors::InvalidStakedTokenAmount  => {
                    println!("Invalid staked token amount provided for swap.");
                }
                Errors::InsufficientLiquidity => {
                    println!("Insufficient liquidity for swap.");
                }
                _ => {
                    println!("Failed to swap");
                }
            }
        }
    }

    match lp_pool.remove_liquidity(LpTokenAmount(to_fixed_point(109.9991))) {
        Ok((returned_tokens, returned_staked_tokens)) => {
            println!("Successfully removed liquidity.");
            println!("\tReceived {} tokens.", from_fixed_point(returned_tokens.0));
            println!("\tReceived {} staked tokens.", returned_staked_tokens.0);
        }
        Err(error) => {
            match error {
                _ => {
                    println!("Failed to add liquidity");
                }
            }
        }
    };

}
