// Client
import { serialize, deserialize, deserializeUnchecked } from "borsh";
import { Buffer } from "buffer";
import web3 from "@solana/web3";

class GreetingAccount {
  message = "1234567890";
  constructor(fields: { message: string } | undefined = undefined) {
    if (fields) {
      this.message = fields.message;
    }
  }
}

console.log("program id: ", pg.PROGRAM_ID.toBase58());

const GreetingSchema = new Map([
  [
    GreetingAccount,
    {
      kind: "struct",
      fields: [["message", "string"]],
    },
  ],
]);

enum InstructionVariant {
  Create = 0,
  Modify = 1,
  Delete = 2,
}

class Assignable {
  constructor(properties) {
    Object.keys(properties).map((key) => {
      return (this[key] = properties[key]);
    });
  }
}

class CreateInstruction extends Assignable {}
class ModifyInstruction extends Assignable {}
class DeleteInstruction extends Assignable {}

const helloWorldInstructionSchema = new Map([
  [
    CreateInstruction,
    {
      kind: "struct",
      fields: [
        ["id", "u8"],
        ["msg", "string"],
      ],
    },
  ],
  [
    ModifyInstruction,
    {
      kind: "struct",
      fields: [
        ["id", "u8"],
        ["msg", "string"],
      ],
    },
  ],
  [DeleteInstruction, { kind: "struct", fields: [["id", "u8"]] }],
]);

const GREETING_SIZE = serialize(
  GreetingSchema,
  new GreetingAccount()
).length;

const greetingAccountKp = new web3.Keypair();
const lamports = await pg.connection.getMinimumBalanceForRentExemption(
  GREETING_SIZE
);
const createGreetingAccountIx = web3.SystemProgram.createAccount({
  fromPubkey: pg.wallet.publicKey,
  lamports,
  newAccountPubkey: greetingAccountKp.publicKey,
  programId: pg.PROGRAM_ID,
  space: GREETING_SIZE,
});
const createIxObj = new CreateInstruction({
  id: InstructionVariant.Create,
  msg: "abc",
});
const createIxBuf = Buffer.from(
  serialize(helloWorldInstructionSchema, createIxObj)
);
const createIx = new web3.TransactionInstruction({
  data: createIxBuf,
  keys: [
    {
      pubkey: greetingAccountKp.publicKey,
      isSigner: false,
      isWritable: true,
    },
  ],
  programId: pg.PROGRAM_ID,
});
const tx = new web3.Transaction();
tx.add(createGreetingAccountIx, createIx);
console.log("greetingAccountKp: ", greetingAccountKp.publicKey.toBase58());
const txHash = await web3.sendAndConfirmTransaction(pg.connection, tx, [
  pg.wallet.keypair,
  greetingAccountKp,
]);
console.log("to see logs, use: solana confirm -v ", txHash);

const greetingAccount = await pg.connection.getAccountInfo(
  greetingAccountKp.publicKey
);
console.log("data: ", greetingAccount.data);

const deserializeAccountData = deserialize(
  GreetingSchema,
  GreetingAccount,
  greetingAccount.data.slice(0, 7)
);
console.log("deserializeAccountData.message: ", deserializeAccountData.message);

const modifyIxObj = new ModifyInstruction({
  id: InstructionVariant.Modify,
  msg: "def",
});
const modifyIxBuf = Buffer.from(
  serialize(helloWorldInstructionSchema, modifyIxObj)
);
const modifyIx = new web3.TransactionInstruction({
  data: modifyIxBuf,
  keys: [
    {
      pubkey: greetingAccountKp.publicKey,
      isSigner: false,
      isWritable: true,
    },
  ],
  programId: pg.PROGRAM_ID,
});
const tx2 = new web3.Transaction();
tx2.add(modifyIx);
const tx2Hash = await web3.sendAndConfirmTransaction(pg.connection, tx2, [
  pg.wallet.keypair,
  greetingAccountKp,
]);
console.log("to see logs use:  solana confirm -v ", tx2Hash);

const greetingAccount2 = await pg.connection.getAccountInfo(
  greetingAccountKp.publicKey
);
const deserializeAccountData2 = deserialize(
  GreetingSchema,
  GreetingAccount,
  greetingAccount2.data.slice(0, 7)
);
console.log(
  "after modify deserializeAccountData.message: ",
  deserializeAccountData2.message
);

const deleteIxObj = new DeleteInstruction({
  id: InstructionVariant.Delete,
});
const deleteIxBuf = Buffer.from(
  serialize(helloWorldInstructionSchema, deleteIxObj)
);
const deleteIx = new web3.TransactionInstruction({
  data: deleteIxBuf,
  keys: [
    {
      pubkey: pg.wallet.keypair.publicKey,
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: greetingAccountKp.publicKey,
      isSigner: true,
      isWritable: true,
    },
  ],
  programId: pg.PROGRAM_ID,
});
const tx3 = new web3.Transaction();
tx3.add(deleteIx);
const tx3Hash = await web3.sendAndConfirmTransaction(pg.connection, tx3, [
  pg.wallet.keypair,
  greetingAccountKp,
]);
console.log("to see logs use: solana confirm -v ", tx3Hash);

const greetingAccount3 = await pg.connection.getAccountInfo(
  greetingAccountKp.publicKey
);
const deserializeAccountData3 = deserialize(
  GreetingSchema,
  GreetingAccount,
  greetingAccount3.data
);
console.log(
  "after delete deserializeAccountData.message: ",
  deserializeAccountData.message
);