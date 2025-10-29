// programs/prediction_market/src/lib.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

// 🚨 ATENÇÃO:
// O Anchor.toml mostra que seu ID local é "Gb4xrAPgL9usuBryCDXqhzRrVAmNTt5tJSgMradvPmm6"
// Quando você fizer o deploy na Devnet, você terá um NOVO ID.
// Por enquanto, vamos usar este que foi gerado.
declare_id!("Gb4xrAPgL9usuBryCDXqhzRrVAmNTt5tJSgMradvPmm6");

#[program]
pub mod prediction_market {
    use super::*;

    /// Cria uma nova conta de mercado de previsão
    ///
    /// Argumentos:
    /// * `ctx` - O contexto com as contas necessárias.
    /// * `description` - A pergunta do mercado (ex: "Palmeiras vence o Brasileirão?").
    /// * `resolution_timestamp` - O timestamp Unix de quando o mercado deve ser resolvido.
    pub fn create_market(
        ctx: Context<CreateMarket>,
        description: String,
        resolution_timestamp: i64,
    ) -> Result<()> {
        
        // Pega a conta do mercado que está sendo criada
        let market = &mut ctx.accounts.market;

        // Preenche os dados da conta 'Market'
        market.authority = *ctx.accounts.authority.key;
        market.description = description; // O tamanho já foi alocado no 'space'
        market.resolution_timestamp = resolution_timestamp;
        market.is_resolved = false;
        market.outcome = 0; // 0 pode representar "indefinido"
        market.yes_token_mint = ctx.accounts.yes_token_mint.key();
        market.no_token_mint = ctx.accounts.no_token_mint.key();
        
        // Placeholder para o estado de liquidez
        market.liquidity_state = LiquidityState { placeholder: 0 };

        msg!("Mercado criado: {}", market.description);
        Ok(())
    }

    // TODO: Adicionar instrução 'trade'
    // TODO: Adicionar instrução 'redeem'
    // TODO: Adicionar instrução 'resolve_market'
}

/// A conta que armazena o estado de um mercado de previsão
#[account]
pub struct Market {
    /// A chave pública que tem permissão para resolver este mercado
    pub authority: Pubkey,
    /// A descrição (pergunta) do mercado
    pub description: String,
    /// Timestamp Unix para resolução
    pub resolution_timestamp: i64,
    /// Flag que indica se o mercado já foi resolvido
    pub is_resolved: bool,
    /// O resultado final (0=Não, 1=Sim)
    pub outcome: u8,
    /// O mint do token SPL para "Sim"
    pub yes_token_mint: Pubkey,
    /// O mint do token SPL para "Não"
    pub no_token_mint: Pubkey,
    /// O estado do AMM ou livro de ordens
    pub liquidity_state: LiquidityState,
}

// Placeholder para a struct LiquidityState
// (Definida no guia, mas não seus campos)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub struct LiquidityState {
    // TODO: Preencher com os campos do AMM (ex: LMSR) ou CLOB
    pub placeholder: u8, // 1 byte
}

/// Contexto de contas para a instrução `create_market`
#[derive(Accounts)]
#[instruction(description: String)] // Permite usar 'description.len()' no 'space'
pub struct CreateMarket<'info> {
    /// A conta Market que será criada e inicializada
    #[account(
        init,
        payer = authority,
        // O espaço é calculado: 8 (discriminator) + 32 (authority) + 
        // (4 + N) (description) + 8 (timestamp) + 1 (is_resolved) + 
        // 1 (outcome) + 32 (yes_mint) + 32 (no_mint) + 1 (liquidity_state)
        space = 8 + 32 + (4 + description.len()) + 8 + 1 + 1 + 32 + 32 + 1
    )]
    pub market: Account<'info, Market>,

    /// A autoridade que está criando o mercado (e pagando as taxas)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// O Mint para o token "Sim". Criamos ele aqui.
    #[account(
        init,
        payer = authority,
        mint::decimals = 6, // Exemplo de 6 decimais
        mint::authority = market, // O programa (via conta Market) controla o mint
    )]
    pub yes_token_mint: Account<'info, Mint>,

    /// O Mint para o token "Não". Criamos ele aqui.
    #[account(
        init,
        payer = authority,
        mint::decimals = 6, // Exemplo de 6 decimais
        mint::authority = market, // O programa (via conta Market) controla o mint
    )]
    pub no_token_mint: Account<'info, Mint>,

    /// O Programa de Tokens SPL, necessário para criar mints
    pub token_program: Program<'info, Token>,
    
    /// O System Program, necessário para criar contas
    pub system_program: Program<'info, System>,
    
    /// Necessário para criar mints (Sysvar Rent)
    pub rent: Sysvar<'info, Rent>,
}