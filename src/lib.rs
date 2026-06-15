// 引入Anchor功能
use anchor_lang::prelude::*;

// 设置合约地址, 可为空
declare_id!("8ZdnxfefVK5VU7R1avErDCfd2W6G4RA6S5pit16Deot");


 // 程序宏
#[program]

// 模块
pub mod counter {
    // 将当前模块的父模块中，所有公共项(pub)引入当前作用域
    use super::*;

    // 创建新计数器
    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        msg!("Creating a Counter!!");

        // 获取参数包含的counter账户
        let counter = &mut ctx.accounts.counter;

        // 获取参数包含的 authority账户的公钥
        counter.authority = ctx.accounts.authority.key();

        // 将counter的数字设置为0
        counter.count = 0;

        msg!("Current count is {}", counter.count);
        msg!("The Admin Pubkey is :{}", counter.authority);


        // 结束
        Ok(())
    }


    // 将计数器 + 1
    pub fn update_counter(ctx: Context<UpdateCounter>) -> Result<()> {
        msg!("Adding 1 to the counter !!");

        // 获取counter
        let counter = &mut ctx.accounts.counter;

        // 将其数字 + 1
        counter.count += 1;

        msg!("Current count is {}", counter.count);
        msg!("{} remaining to reach 1000 ", 1000 - counter.count);

        // 结束
        Ok(())
    }
}


// 账户宏, 处理多个账户的验证、访问逻辑
#[derive(Accounts)]
// CreateCounter 账户结构体，创建计数器时传入
pub struct CreateCounter<'info> {
    // #[account(mut)]表示账户可变
    #[account(mut)]

    // 签名者
    pub authority: Signer<'info>,

    // 账户宏, 此处用于定义PDA账户，修饰counter
    // init: 将会被创建
    // seeds: 生成PDA账户的种子，此处为签名者的地址
    // bump: 需要找到有效的PDA
    // payer = authority: 指定由谁来支付费用
    // space = 100: 指定账户分配存储的字节空间
    #[account(
        init,
        seeds = [authority.key().as_ref()],
        bump,
        payer = authority,
        space = 100
    )]
    pub counter: Account<'info, Counter>,
    // Solana系统程序，调用系统功能(创建账户、分配租金等)
    pub system_program: Program<'info, System>,
}

// 账户宏，处理多个账户的验证、访问逻辑
#[derive(Accounts)]
// UpdateCounter账户结构体，将计数器 + 1时使用
pub struct UpdateCounter<'info> {
    // 签名者
    authority: Signer<'info>,
    // 可变，且保证签名者authority与对应计数器账户的地址 保持一致
    #[account(mut, has_one = authority)]
    counter: Account<'info, Counter>,
}


// 账户宏, 定义单个账户的结构体
#[account]
pub struct Counter {
    // 地址
    authority: Pubkey,
    // 计数器数字
    count: u64,
}
