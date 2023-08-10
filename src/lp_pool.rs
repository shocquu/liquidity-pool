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

pub const FIXED_POINT_SCALE: u64 = 1_000_000;

pub struct LpPool {
    price: Price,
    token_amount: TokenAmount,
    st_token_amount: StakedTokenAmount,
    lp_token_amount: LpTokenAmount,
    liquidity_target: TokenAmount,
    min_fee: Percentage,
    max_fee: Percentage,
}

pub enum Errors {
    InvalidTokenAmount,
    InvalidStakedTokenAmount,
    InvalidLpTokenAmount,
    InsufficientLiquidity
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
            lp_token_amount: LpTokenAmount(1),
            st_token_amount: StakedTokenAmount(10 * FIXED_POINT_SCALE),
        })
    }

    pub fn add_liquidity(
        self: &mut Self,
        token_amount: TokenAmount
    ) -> Result<LpTokenAmount, Errors> {
        if token_amount.0 == 0 {
            return Err(Errors::InvalidTokenAmount);
        }

        let lp_tokens = LpTokenAmount((token_amount.0 * self.lp_token_amount.0) / FIXED_POINT_SCALE);

        self.token_amount.0 += token_amount.0;
        self.lp_token_amount.0 += lp_tokens.0;

        Ok(lp_tokens)
    }

    pub fn remove_liquidity(
        self: &mut Self,
        lp_token_amount: LpTokenAmount
    ) -> Result<(TokenAmount, StakedTokenAmount), Errors> {
        if lp_token_amount.0 == 0 || lp_token_amount.0 > self.lp_token_amount.0 {
            return Err(Errors::InvalidLpTokenAmount);
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
        println!("liquidity_target: {}, token_amount: {}", self.liquidity_target.0, self.token_amount.0);
        
        if staked_token_amount.0 == 0 || staked_token_amount.0 > self.st_token_amount.0 {
            return Err(Errors::InvalidStakedTokenAmount);
        }

        let received_tokens = TokenAmount((staked_token_amount.0 * self.price.0) / FIXED_POINT_SCALE);

        if self.liquidity_target.0 < self.token_amount.0 {
            return Err(Errors::InsufficientLiquidity);
        }

        let target_diff = self.liquidity_target.0 - self.token_amount.0;
        let fee_percentage = Percentage(((target_diff * (self.max_fee.0 - self.min_fee.0)) / self.liquidity_target.0) + self.min_fee.0);
        let fee = Percentage((received_tokens.0 * fee_percentage.0) / FIXED_POINT_SCALE );
        let received_tokens_after_fee = TokenAmount((received_tokens.0 * (FIXED_POINT_SCALE  - fee.0)) / FIXED_POINT_SCALE );

        self.token_amount.0 -= received_tokens_after_fee.0;
        self.st_token_amount.0 += staked_token_amount.0;

        Ok(received_tokens_after_fee)
    }
}
