mod lp_pool;

const FIXED_POINT_SCALE: u64 = 10000;

fn to_fixed_point(value: f64) -> u64 {
    (value * FIXED_POINT_SCALE as f64) as u64
}

fn from_fixed_point(value: u64) -> f64 {
    value as f64 / FIXED_POINT_SCALE as f64
}

fn main() {
    use lp_pool::{LpPool, Price, Percentage, TokenAmount, StakedTokenAmount, Errors};

    let price = Price((1.5 * lp_pool::FIXED_POINT_SCALE as f64) as u64);
    let min_fee = Percentage((0.1 * 10000.0) as u64);
    let max_fee = Percentage((90.0 * 10000.0) as u64);
    let liquidity_target = TokenAmount((900.0 * lp_pool::FIXED_POINT_SCALE as f64) as u64);


    let mut lp_pool = match LpPool::init(price, min_fee, max_fee, liquidity_target) {
        Ok(pool) => pool,
        Err(_) => {
            println!("Failed to initialize LpPool");
            return;
        }
    };

    match lp_pool.add_liquidity(TokenAmount((100.0 * lp_pool::FIXED_POINT_SCALE as f64) as u64)) {
        Ok(lp_tokens) => {
            println!("Successfully added liquidity. Received {} LP tokens.", lp_tokens.0);
        }
        Err(error) => {
            match error {
                Errors::InvalidTokenAmount => {
                    println!("Invalid token amount provided for adding liquidity.");
                }
                // Errors::InsufficientLiquidity => {
                //     println!("Insufficient liquidity to mint LP tokens.");
                // }
                // Handle other possible errors here
                _ => {
                    println!("Failed to add liquidity");
                }
            }
        }
    };

    match lp_pool.swap(StakedTokenAmount((6.0 * lp_pool::FIXED_POINT_SCALE as f64) as u64)) {
        Ok(received_tokens) => {
            println!("Successfully swapped staked tokens. Received {} tokens.", received_tokens.0);
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

    // let received_tokens = match lp_pool.swap(StakedTokenAmount(6)) {
    //     Ok(tokens) => tokens,
    //     Err(Errors::InvalidStakedTokenAmount) => {
    //         println!("Invalid staked token amount for swap");
    //         return;
    //     }
    //     Err(_) => {
    //         println!("An error occurred during swap");
    //         return;
    //     }
    // };


    // let lp_tokens_after_add = match lp_pool.add_liquidity(TokenAmount((10.0 * lp_pool::FIXED_POINT_SCALE as f64) as u64)) {
    //     Ok(tokens) => tokens,
    //     Err(_) => {
    //         println!("Failed to add liquidity after swap");
    //         return;
    //     }
    // };

    // let received_tokens_after_swap = match lp_pool.swap(StakedTokenAmount(30)) {
    //     Ok(tokens) => tokens,
    //     // Err(Errors::InsufficientLiquidity) => {
    //     //     println!("Insufficient liquidity for second swap");
    //     //     return;
    //     // }
    //     Err(_) => {
    //         println!("Failed to perform second swap");
    //         return;
    //     }
    // };

    // let (returned_tokens, returned_staked_tokens) = match lp_pool.remove_liquidity(lp_tokens_after_add) {
    //     Ok(tokens) => tokens,
    //     _ => {
    //         println!("Failed to remove liquidity");
    //         return;
    //     }
    // };
    
    // println!("lp tokens after adding liquidity: {:?}", lp_tokens);
    // println!("tokens after swap: {:?}", received_tokens);
    // // println!("lp tokens after second adding liquidity: {:?}", lp_tokens_after_add);
    // println!("tokens after second swap: {:?}", received_tokens_after_swap);
    // println!("tokens after removing liquidity: {:?}", returned_tokens);
    // println!("st tokens after removing liquidity: {:?}", returned_staked_tokens);
}
