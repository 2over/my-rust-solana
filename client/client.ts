// 使用Web3.Transaction()创建一个新的Tx
const transaction = new web3.Transaction();

// 创建一个Instruct
// Keys[]中列出了交易中涉及的所有账户和它们各自的访问权限
//(例如, 是否可以读取、写入等)，因为代码不需要与用户账户交互，
// 所以这里keys[]为空
transaction.add(
  new web3.TransactionInstruction({
    keys: [],
    programId: new web3.PublicKey(pg.PROGRAM_ID),
  }),
);


// 调用sendAndConfirmTransaction()方法, 参数为端口、交易、签名的私钥数组
const txHash = await web3.sendAndConfirmTransaction(
  pg.connection,
  transaction,
  [pg.wallet.keypair],
);

// 打印消息记录到控制台， 并显示哈希值
console.log("Transaction sent with hash:", txHash);