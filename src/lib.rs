// 导入solana-program
// Account_info: 账户详细信息
// entrypoint 程序入口
// msg: 在Solana上打印消息
use solana_program:: {
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg
};

// 定义代码的入口,开始执行智能合约
entrypoint!(process_instruction);


// 定义一个名为process_instruction的公共函数, 
// 参数为程序id、账户和指令数据字段
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    // 将""Hello Solana"消息打印在Solana区块链上
    msg!("Hello Web3 Solana!");

    // 像系统返回状态代码来退出程序
    Ok(())
}