// 导入Anchor, 只需要导入一个包, 非常简洁
use anchor_lang::prelude::*;

// 使用Anchor, 要声明本程序的公钥， Solana Playground会在构建程序时自动填充程序id,
// 可以不填

declare_id!("7pahNQg1NucaGBEstJeHktok5QTe15iLK2fZMxbihthL");


// program宏， 会自动生成一些必要的样本代码，保证Anchor程序正常运行
#[program]

// 定义hello_world模板，里面可以写函数、变量等
mod hello_world {
    // 导入当前模块的父模块的全部公开项(如变量、函数)
    use super::*;

    // 访问权限为pub公开，入参为Context类型，出参为Result<()>类型
    // Context为Anchor提供的反省，封装调用函数的上下文(如账户信息),
    // Hello 是一个结构体，定义了调用该函数所需的账户信息
    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        msg!("Hello Anchor!");
        Ok(())
    }
}

// derive(Accounts)宏，同样可以自动生成一些样本代码，保证符合函数需要的账户信息
#[derive(Accounts)]

// 具体的Hello结构体,因为仅仅时打印日志不需要任何账户，此处结构体为空
pub struct Hello {}