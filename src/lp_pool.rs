pub const FIXED_POINT_SCALE: u64 = 1_000_000;

#[derive(Debug)]
pub struct TokenAmount(pub u64);
#[derive(Debug)]
pub struct StakedTokenAmount(pub u64);
#[derive(Debug)]
pub struct LpTokenAmount(pub u64);
#[derive(Debug)]
pub struct Price(pub u64);
#[derive(Debug)]
pub struct Percentage(pub u64);

#[derive(Debug)]
pub enum Errors {
    InvalidTokenAmount,
    InvalidStakedTokenAmount,
    InvalidLpTokenAmount,
    InsufficientLiquidity
}

pub struct LpPool {
    price: Price,
    token_amount: TokenAmount,
    st_token_amount: StakedTokenAmount,
    lp_token_amount: LpTokenAmount,
    liquidity_target: TokenAmount,
    min_fee: Percentage,
    max_fee: Percentage,
}

impl LpPool {
    pub fn init(
        price: Price,
        min_fee: Percentage,
        max_fee: Percentage,
        liquidity_target: TokenAmount
    ) -> Result<Self, Errors> {
        Ok(LpPool {
            price,
            min_fee,
            max_fee,
            liquidity_target,
            token_amount: TokenAmount(0),
            lp_token_amount: LpTokenAmount(0),
            st_token_amount: StakedTokenAmount(0),
        })
    }

    pub fn add_liquidity(
        self: &mut Self,
        token_amount: TokenAmount
    ) -> Result<LpTokenAmount, Errors> {
        if token_amount.0 == 0 {
            return Err(Errors::InvalidTokenAmount);
        }
    
        if self.token_amount.0 == 0 {
            self.token_amount.0 += token_amount.0;
            self.lp_token_amount.0 += token_amount.0;
            return Ok(LpTokenAmount(token_amount.0));
        }
    
        let diff = (token_amount.0 * self.lp_token_amount.0) / self.token_amount.0 / FIXED_POINT_SCALE;
        let minted_lp_tokens = LpTokenAmount(token_amount.0 - diff);
    
        self.token_amount.0 += token_amount.0;
        self.lp_token_amount.0 += minted_lp_tokens.0;
    
        Ok(minted_lp_tokens)
    }

    pub fn remove_liquidity(
        self: &mut Self,
        lp_token_amount: LpTokenAmount
    ) -> Result<(TokenAmount, StakedTokenAmount), Errors> {
        if lp_token_amount.0 == 0 {
            return Err(Errors::InvalidLpTokenAmount);
        }

        if lp_token_amount.0 > self.lp_token_amount.0 {
            return Err(Errors::InsufficientLiquidity);
        }

        let tokens_to_return = TokenAmount((lp_token_amount.0 * self.token_amount.0) / self.lp_token_amount.0);
        let staked_tokens_to_return = StakedTokenAmount((lp_token_amount.0 * self.st_token_amount.0) / self.lp_token_amount.0);

        self.token_amount.0 -= tokens_to_return.0;
        self.st_token_amount.0 -= staked_tokens_to_return.0;
        self.lp_token_amount.0 -= lp_token_amount.0;
        
        Ok((tokens_to_return, staked_tokens_to_return))
    }

    pub fn swap(
        self: &mut Self,
        staked_token_amount: StakedTokenAmount
    ) -> Result<TokenAmount, Errors> {
        if staked_token_amount.0 == 0 {
            return Err(Errors::InvalidStakedTokenAmount);
        }
    
        let received_tokens = (staked_token_amount.0 * self.price.0) / FIXED_POINT_SCALE;
        
        let target_diff = (self.liquidity_target.0 as i64 - self.token_amount.0 as i64).abs() as u64;
        let is_excess = self.liquidity_target.0 >= self.token_amount.0;
        
        let fee_percentage = if is_excess {
            (((target_diff * (self.max_fee.0 - self.min_fee.0)) / self.liquidity_target.0) + self.min_fee.0) / FIXED_POINT_SCALE
        } else {
            let reduction_factor = (target_diff * (self.max_fee.0 - self.min_fee.0)) / self.liquidity_target.0;
            let adjusted_fee_percentage = self.max_fee.0 - reduction_factor;
            adjusted_fee_percentage / FIXED_POINT_SCALE
        };
        
        let fee = (received_tokens * fee_percentage) / FIXED_POINT_SCALE;
        let received_tokens_after_fee = received_tokens - fee;
        
        if received_tokens_after_fee > self.token_amount.0 {
            return Err(Errors::InsufficientLiquidity);
        }

        self.token_amount.0 -= received_tokens_after_fee;
        self.st_token_amount.0 += staked_token_amount.0;
    
        Ok(TokenAmount(received_tokens_after_fee))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        
        assert_eq!(lp_pool.token_amount.0, 0);
        assert_eq!(lp_pool.st_token_amount.0, 0);
        assert_eq!(lp_pool.lp_token_amount.0, 0);
    }

    #[test]
    fn test_add_liquidity() {
        let mut lp_pool = create_test_pool();
        let token_amount = TokenAmount(to_fixed_point(100.0));

        let result = lp_pool.add_liquidity(token_amount).unwrap();

        // assert_eq!(lp_pool.token_amount.0, to_fixed_point(100.0));
        // assert_eq!(lp_pool.lp_token_amount.0, to_fixed_point(101.0));
        assert_eq!(result.0, to_fixed_point(100.0)); // Minted LP tokens
    }

    #[test]
    fn test_swap() {
        let mut lp_pool = create_test_pool();
        lp_pool.add_liquidity(TokenAmount(to_fixed_point(100.0)));

        let staked_token_amount = StakedTokenAmount(to_fixed_point(6.0));
        
        let result = lp_pool.swap(staked_token_amount).unwrap();
        
        // assert_eq!(lp_pool.token_amount.0, to_fixed_point(91.009));
        // assert_eq!(lp_pool.st_token_amount.0, to_fixed_point(6.0));
        assert_eq!(result.0, to_fixed_point(8.991));
    }
    
    #[test]
        fn test_remove_liquidity() {
        let mut lp_pool = create_test_pool();
        let lp_token_amount = LpTokenAmount(to_fixed_point(109.9991));

        lp_pool.lp_token_amount = LpTokenAmount(to_fixed_point(200.0));

        let result = lp_pool.remove_liquidity(lp_token_amount).unwrap();

        assert_eq!(result.0.0, to_fixed_point(57.56663)); // Returned tokens
        assert_eq!(result.1.0, to_fixed_point(36.0)); // Returned staked tokens
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

