



# 1, 创建一个新账号

创建一个新账号
```sh
$ solana-keygen new --force
```

显示公钥
```sh
$ solana-keygen pubkey
9eep9kT9D2MovMT7YcuXiPLLCX2pykYAHQrJt9jfdfBk
```

经过多次airdrop及transfer操作后查看该账号余额
```sh
$ solana balance
1.9299736 SOL
```


# 2, 实时展示余额变化


使用curl 调用JSON RPC 查询该账号余额
```sh
$ curl -X POST https://api.devnet.solana.com -H "Content-Type: application/json" -d '
>   {
>     "jsonrpc": "2.0", "id": 1,
>     "method": "getBalance",
>     "params": [
>       "9eep9kT9D2MovMT7YcuXiPLLCX2pykYAHQrJt9jfdfBk"
>     ]
>   }
> '
{"jsonrpc":"2.0","result":{"context":{"apiVersion":"1.16.6","slot":235276018},"value":1929973600},"id":1}
```


在websocketking网站上连接 wss://api.devnet.solana.com, 连接上之后, 订阅该账户变化:
发送:
```
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "accountSubscribe",
  "params": [
    "9eep9kT9D2MovMT7YcuXiPLLCX2pykYAHQrJt9jfdfBk",
    {
      "encoding": "jsonParsed",
      "commitment": "confirmed"
    }
  ]
}
```
收到返回信息
```
{
  "jsonrpc": "2.0",
  "result": 2721883,
  "id": 1
}
```


在phantom中从该账户向其它人的账户转0.05,稍等片刻, 收到该账户变动的信息:
```{
  "jsonrpc": "2.0",
  "method": "accountNotification",
  "params": {
    "result": {
      "context": {
        "slot": 235276690
      },
      "value": {
        "lamports": 1879967000,
        "data": [
          "",
          "base64"
        ],
        "owner": "11111111111111111111111111111111",
        "executable": false,
        "rentEpoch": 0,
        "space": 0
      }
    },
    "subscription": 2721883
  }
}
```


# 3, 列出已知SPL-Token的余额


使用新账户创建一个新的SPL-token
```sh
$ spl-token create-token
Creating token CEbaSH6yMvusuEVKrSqteorac3gfWMqA5dCewNFWZb8L under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Address:  CEbaSH6yMvusuEVKrSqteorac3gfWMqA5dCewNFWZb8L
Decimals:  9

Signature: 5WGjo7rfb4YumL2SMBgsMAmVU9bnuDhsGsL5yP2tm1iHyZgFWjWxjTpxurXpU961iHNfBgPjfykberEgF6u2999D
```


创建一个ATA存放该SPL-token
```sh
$ spl-token create-account CEbaSH6yMvusuEVKrSqteorac3gfWMqA5dCewNFWZb8L
Creating account HczZGUWCBQCbjgaiQUkyJ713fUeSXeQ4uTNKdHhdqcVJ

Signature: 4B8x9mBUF4YPoimt9STv2fvMCfpXadGcyj2RLNh8xHWjr3rQQTY7d2mzvW65fNp8HecTm95wF7CUFYZD3HwLNhdG
```

铸造100个该代币
```sh
$ spl-token mint CEbaSH6yMvusuEVKrSqteorac3gfWMqA5dCewNFWZb8L 100
Minting 100 tokens
  Token: CEbaSH6yMvusuEVKrSqteorac3gfWMqA5dCewNFWZb8L
  Recipient: HczZGUWCBQCbjgaiQUkyJ713fUeSXeQ4uTNKdHhdqcVJ

Signature: 5C6ZcoM5Xg5kg77S7HEkN6DT8PRxp4H13DkmaAjDkeirZKcUaXtjqSk38XFC2jAyeqU7BcZ68ReknVy411aW9rwx
```

# 4, 实时展示余额变化


用curl 调用 JSON RPC 查询该ATA账号中SPL-token余额
```sh
$ curl -X POST https://api.devnet.solana.com -H "Content-Type: application/json" -d '
>   {
>     "jsonrpc": "2.0", "id": 1,
>     "method": "getTokenAccountBalance",
>     "params": [
>       "HczZGUWCBQCbjgaiQUkyJ713fUeSXeQ4uTNKdHhdqcVJ"
>     ]
>   }
> '
{"jsonrpc":"2.0","result":{"context":{"apiVersion":"1.16.6","slot":235279515},"value":{"amount":"100000000000","decimals":9,"uiAmount":100.0,"uiAmountString":"100"}},"id":1}
```

在websocketking网站上订阅该ATA账号中SPL-token余额的变化
发送:
```
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "accountSubscribe",
  "params": [
    "HczZGUWCBQCbjgaiQUkyJ713fUeSXeQ4uTNKdHhdqcVJ",
    {
      "encoding": "jsonParsed",
      "commitment": "confirmed"
    }
  ]
}
```
收到返回信息:
```
{
  "jsonrpc": "2.0",
  "result": 2759232,
  "id": 1
}
```


先在phantom上给其它账号转30个代币


在websocketking网站上收到该SPL-token 的 amount变成了70的消息:
```
{
  "jsonrpc": "2.0",
  "method": "accountNotification",
  "params": {
    "result": {
      "context": {
        "slot": 235280049
      },
      "value": {
        "lamports": 2039280,
        "data": {
          "program": "spl-token",
          "parsed": {
            "info": {
              "isNative": false,
              "mint": "CEbaSH6yMvusuEVKrSqteorac3gfWMqA5dCewNFWZb8L",
              "owner": "9eep9kT9D2MovMT7YcuXiPLLCX2pykYAHQrJt9jfdfBk",
              "state": "initialized",
              "tokenAmount": {
                "amount": "70000000000",
                "decimals": 9,
                "uiAmount": 70,
                "uiAmountString": "70"
              }
            },
            "type": "account"
          },
          "space": 165
        },
        "owner": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        "executable": false,
        "rentEpoch": 0,
        "space": 165
      }
    },
    "subscription": 2759232
  }
}
```

再给其它账号转10个代币

此时在websocketking网站上收到该SPL-token 的 amount变成了60的消息
```
{
  "jsonrpc": "2.0",
  "method": "accountNotification",
  "params": {
    "result": {
      "context": {
        "slot": 235280132
      },
      "value": {
        "lamports": 2039280,
        "data": {
          "program": "spl-token",
          "parsed": {
            "info": {
              "isNative": false,
              "mint": "CEbaSH6yMvusuEVKrSqteorac3gfWMqA5dCewNFWZb8L",
              "owner": "9eep9kT9D2MovMT7YcuXiPLLCX2pykYAHQrJt9jfdfBk",
              "state": "initialized",
              "tokenAmount": {
                "amount": "60000000000",
                "decimals": 9,
                "uiAmount": 60,
                "uiAmountString": "60"
              }
            },
            "type": "account"
          },
          "space": 165
        },
        "owner": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        "executable": false,
        "rentEpoch": 0,
        "space": 165
      }
    },
    "subscription": 2759232
  }
}
```