// 发起调用的智能合约(中转合约)
const MAIN_PROGRAM_ID = new web3.PublicKey('FNFYU1WUuQMVLHbVU891WMDptbEFZmEKwgdAvd9GD9HT')


// 目标智能合约(Hello Web3 Solana)
const HELLO_PROGRAM_ID = new web3.PublicKey('GeJHumojz9pD97fJC76mrWpbNEBWKHFXAjoY1TXQhD38');


 // 创建一个新交易
 const transaction = new web3.Transaction();

// 创建一个指令，发送给MAIN_PROGRAM_ID发起调用的合约
transaction.add(
  new web3.TransactionInstruction({
    programId: MAIN_PROGRAM_ID,
    keys: [
      // isSinger 标识该账户是否作为交易的签名者
      // isWritable 表示该账户是否是可写的，可写代表可修改

      // 发起调用的个人账户
      {pubkey:pg.wallet.keypair.publicKey, isSigner: true, isWritable: true},
      // 要调用的智能合约
      { pubkey:HELLO_PROGRAM_ID, isSigner: false, isWritable: false},
    ],
    data: Buffer.alloc(0), // 传递给合约的数据
  })
);

// 发起并确认交易
const txHash = await web3.sendAndConfirmTransaction(
  pg.connection,
  transaction,
  [pg.wallet.keypair]
);

// 打印交易哈希
console.log("交易哈希:", txHash)