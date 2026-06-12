use std::slice::Iter;

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program::{invoke},
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
};


// 定义代码的入口, process_instruction是程序启动时被调用的函数
entrypoint!(process_instruction);

// 参数为当前程序的id、传入的账户数组 以及传入的指令数据字段
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {

    // 获取传入账户数组的迭代器
    let accounts_iter: &mut Iter<AccountInfo> = &mut accounts.iter();

    // 获取调用该程序的账户
    let caller_account: &AccountInfo = accounts_iter.next().ok_or_else(|| {
        msg!("Failed to get caller account");
        ProgramError::NotEnoughAccountKeys
    })?;

    // 被调用的账户程序
    let called_program_account: &AccountInfo = accounts_iter.next().ok_or_else(|| {
        msg!("Failed to get called program account");
        ProgramError::NotEnoughAccountKeys
    })?;

    // 创建指向"Hello, Web3 Solana" 合约的指令
    let instruction: Instruction = Instruction {
        program_id: *called_program_account.key,
        accounts: vec![
            // true标识需要签名 new_readonly 只读
            AccountMeta::new(*caller_account.key, true),
            AccountMeta::new_readonly(*called_program_account.key, false),

        ],
        data: vec![], // 这里可以传递给被调用程序的数据
    };

    // 调用""Hello Web3 Solana" 合约
    invoke(
        &instruction,
        // 再传入一边地址，顺序与instruction中的accounts账户顺序一致
        &[
            // 签名者
            caller_account.clone(),
            // 被调用的程序
            called_program_account.clone(),
        ],
    )?;


    // 打印消息
    msg!("调用合约成功");
    msg!("中转合约的地址program_id是{}", program_id.to_string());
    msg!("发起调用的用户账户 caller_account 是{}", caller_account.key.to_string());
    msg!("最终调用的目标地址 called_program_account 是:{}", called_program_account.key.to_string());

    // 返回成功
    Ok(())
}