#[path = "../src/lp_pool.rs"]
mod lp_pool;

#[cfg(test)]
mod tests {
    use super::lp_pool::*;

    #[test]
    fn test_init() {
        let price = Price(to_fixed_point(1.5));
        let min_fee = Percentage(to_fixed_point(0.1));
        let max_fee = Percentage(to_fixed_point(9.0));
        let liquidity_target = TokenAmount(to_fixed_point(90.0));

        let lp_pool = LpPool::init(price, min_fee, max_fee, liquidity_target).unwrap();

        assert_eq!(lp_pool.price.0, to_fixed_point(1.5));
        assert_eq!(lp_pool.min_fee.0, to_fixed_point(0.1));
        assert_eq!(lp_pool.max_fee.0, to_fixed_point(9.0));
        assert_eq!(lp_pool.liquidity_target.0, to_fixed_point(90.0));
        
        // fuck this shit
        assert_eq!(lp_pool.token_amount.0, 1 * FIXED_POINT_SCALE);
        assert_eq!(lp_pool.st_token_amount.0, 0 * FIXED_POINT_SCALE);
        assert_eq!(lp_pool.lp_token_amount.0, 1 * FIXED_POINT_SCALE);
    }

    #[test]
    fn test_add_liquidity() {
        let mut lp_pool = create_test_pool();
        let token_amount = TokenAmount(to_fixed_point(100.0));

        let result = lp_pool.add_liquidity(token_amount).unwrap();

        assert_eq!(lp_pool.token_amount.0, to_fixed_point(101.0)); // Initial + 100
        assert_eq!(lp_pool.lp_token_amount.0, to_fixed_point(101.0)); // Initial + 100
        assert_eq!(result.0, to_fixed_point(100.0)); // Minted LP tokens
    }

    #[test]
    fn test_swap() {
        let mut lp_pool = create_test_pool();
        let staked_token_amount = StakedTokenAmount(to_fixed_point(6.0));

        let result = lp_pool.swap(staked_token_amount).unwrap();

        assert_eq!(lp_pool.token_amount.0, to_fixed_point(100.0)); // Initial - 8.991
        assert_eq!(lp_pool.st_token_amount.0, to_fixed_point(6.0)); // Initial + 6
        assert_eq!(result.0, to_fixed_point(8.991)); // Received tokens after fee
    }

    #[test]
    #[ignore]
    fn test_remove_liquidity() {
        let mut lp_pool = create_test_pool();
        let lp_token_amount = LpTokenAmount(to_fixed_point(100.0));

        let result = lp_pool.remove_liquidity(lp_token_amount).unwrap();

        assert_eq!(lp_pool.token_amount.0, to_fixed_point(91.008)); // Initial - 8.991
        assert_eq!(lp_pool.st_token_amount.0, to_fixed_point(6.0)); // Initial - 6
        assert_eq!(lp_pool.lp_token_amount.0, to_fixed_point(1.0)); // Initial - 100
        assert_eq!(result.0.0, to_fixed_point(8.991)); // Returned tokens
        assert_eq!(result.1.0, to_fixed_point(6.0)); // Returned staked tokens
    }

    fn create_test_pool() -> LpPool {
        let price = Price(to_fixed_point(1.5));
        let min_fee = Percentage(to_fixed_point(0.1));
        let max_fee = Percentage(to_fixed_point(9.0));
        let liquidity_target = TokenAmount(to_fixed_point(90.0));

        LpPool::init(price, min_fee, max_fee, liquidity_target).unwrap()
    }

    fn to_fixed_point(value: f64) -> u64 {
        (value * FIXED_POINT_SCALE as f64) as u64
    }
}
