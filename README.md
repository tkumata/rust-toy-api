# ガラクタ API

Rust で書いた簡素な Web API です。

## GET Method

- GET /healthcheck で OK を返します。
- GET /metrics でホストの CPU Load / Disk Usage / Memory Usage を取得します。
- GET /metrics/cpu で CPU Load を取得します。
- GET /metrics/memory でメモリの利用状況を取得します。
- GET /metrics/storage でストレージの利用状況を取得します。
- GET /dice/roll_1d6 で 1 〜 6 をランダムで返します。
- GET /sleep/i32 で i32 秒 sleep します。threading 確認用です。例 /sleep/30

e.g,

リクエスト

```shell
curl --location 'http://localhost:4000/metrics'
```

結果

```json
{
    "cpu_load": {
        "load_01": 2.41,
        "load_05": 2.02,
        "load_15": 1.79
    },
    "disk_info": [
        {
            "available_space": "143.12 GB",
            "mount_point": "/",
            "total_space": "229.00 GB"
        },
        {
            "available_space": "143.12 GB",
            "mount_point": "/vscode",
            "total_space": "229.00 GB"
        },
        {
            "available_space": "143.12 GB",
            "mount_point": "/workspaces",
            "total_space": "229.00 GB"
        },
        {
            "available_space": "143.12 GB",
            "mount_point": "/etc/resolv.conf",
            "total_space": "229.00 GB"
        },
        {
            "available_space": "143.12 GB",
            "mount_point": "/etc/hostname",
            "total_space": "229.00 GB"
        },
        {
            "available_space": "143.12 GB",
            "mount_point": "/etc/hosts",
            "total_space": "229.00 GB"
        }
    ],
    "kernel_info": "Linux 12 Debian GNU/Linux 6.1.75-vendor-rk35xx",
    "memory_usage": {
        "memory_total": "15.35 GB",
        "memory_usage": "7.76 GB"
    }
}
```

## POST Method

- POST /convert/bitv4 で IP アドレスの bit を netmask に変換します。例 27 -> 255.255.255.224
- POST /convert/rgb で RGB を hex に変換します。例 55,155,250 -> 379BFA

e.g,

リクエスト

```shell
curl --location 'http://localhost:4000/convert/bitv4' \
--header 'Content-Type: application/json' \
--data '{
    "bit_length": 17
}'
```

結果

```text
255.255.128.0
```
