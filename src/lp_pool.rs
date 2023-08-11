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
    pub price: Price,
    pub token_amount: TokenAmount,
    pub st_token_amount: StakedTokenAmount,
    pub lp_token_amount: LpTokenAmount,
    pub liquidity_target: TokenAmount,
    pub min_fee: Percentage,
    pub max_fee: Percentage,
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
            token_amount: TokenAmount(1 * FIXED_POINT_SCALE),
            lp_token_amount: LpTokenAmount(1 * FIXED_POINT_SCALE),
            st_token_amount: StakedTokenAmount(0 * FIXED_POINT_SCALE),
        })
    }

    pub fn add_liquidity(
        self: &mut Self,
        token_amount: TokenAmount
    ) -> Result<LpTokenAmount, Errors> {
        if self.token_amount.0 == 0 {
            return Err(Errors::InvalidTokenAmount);
        }
    
        let minted_lp_tokens = LpTokenAmount((token_amount.0 * self.lp_token_amount.0) / self.token_amount.0);
    
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
    
        let target_diff = self.liquidity_target.0 - self.token_amount.0;
        let fee_percentage = (((target_diff * (self.max_fee.0 - self.min_fee.0)) / self.liquidity_target.0) + self.min_fee.0 ) / FIXED_POINT_SCALE;
        let fee = (received_tokens * fee_percentage) / FIXED_POINT_SCALE;
        // let received_tokens_after_fee = (received_tokens * (FIXED_POINT_SCALE - fee)) / FIXED_POINT_SCALE;
        
        let received_tokens_after_fee = received_tokens - fee;
        println!("received_tokens {} | fee {}", received_tokens_after_fee, self.token_amount.0);
        
        self.token_amount.0 -= received_tokens_after_fee;
        self.st_token_amount.0 += staked_token_amount.0;
    
        Ok(TokenAmount(received_tokens_after_fee))
    }
}
