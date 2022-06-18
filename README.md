# Covid IPFS
Ken Labs Dorahacks Match

### Solving the problem of nucleic acid detection results during epidemic based on IPFS

Technology comes from solving practical problems in life. We are an open source team. We have been working on the framework of web2 and web3. 
We have implemented the IPFS Client through the rust language. 

In this competition, we used the framework to make a demo of nucleic acid. We didn't have much bright spots. The technology is very ordinary. We just use technology to solve the problems of life phenomena, If we can make some technical contribution to the epidemic situation in Shanghai, it is our dream.

We used IPFS multi node storage, deployed Go IPFS Server, and used our Summer IPFS Client to do some nucleic acid detection API, nucleic acid entry, nucleic acid result query, antigen query, etc. 

one of its target users is to provide nucleic acid entry services for nucleic acid institutions, and the other is to provide nucleic acid result query services for all users.

Finally, we hope to overcome the epidemic and resume normal life as soon as possible!

## Quick Start

Start the service first, and then call the service API

```Shell
~ cargo build

~ cargo run
```
## API Doc

### POST 保存核酸信息到IPFS

POST /nucleic/save

> Body 请求参数

```json
{
  "name": "赵伟",
  "date": "2022-05-29",
  "result": "阴性"
}
```

#### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» name|body|string| 是 | 姓名|none|
|» sex|body|integer| 是 | 性别|0-女 1-男|
|» phone|body|string| 是 | 电话|none|
|» address|body|string| 是 | 住址|none|
|» date|body|string| 是 | 日期|暂时用String|
|» result|body|string| 是 | 核酸结果|NEGATIVE-阴性，POSITIVE-阳性|

> 返回示例

> 成功

```json
{
  "code": 200,
  "msg": "成功",
  "data": "QmYovbj7PzTpsJoEXmrbgGTFTpoesRFU3HK4FK8fMrhUE4"
}
```

#### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

#### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» msg|string|true|none|消息|none|
|» data|string|true|none|hash ipfs|none|

### GET 查询核酸信息IPFS

GET /nucleic/query/{hash}

#### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|hash|path|string| 是 ||hash值|

> 返回示例

> 成功

```json
{
  "code": 200,
  "msg": "成功",
  "data": "法外狂徒张三-2022-06-18 14:51-阳性"
}
```

#### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

#### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» msg|string|true|none|消息|none|
|» data|string|true|none|核酸结果|none|
