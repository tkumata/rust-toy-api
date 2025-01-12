# 🗑️ Clutter Web API 🌐

A clutter Web API built with Rust.

## 📥 GET Method

- **GET /healthcheck**: Returns `OK`.
- **GET /metrics**: Retrieves host metrics, including CPU Load, Disk Usage, and Memory Usage.
- **GET /metrics/cpu**: Retrieves CPU Load metrics.
- **GET /metrics/memory**: Retrieves memory usage metrics.
- **GET /metrics/storage**: Retrieves storage usage metrics.
- **GET /dice/roll_1d6**: Returns a random number between 1 and 6.
- **GET /sleep/{i32}**: Sleeps for `{i32}` seconds (for threading tests). e.g, `/sleep/30`.

### 🔄 Request (GET)

```shell
curl --location 'http://localhost:4000/metrics'
```

### ✅ Response (GET)

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

## 📤 POST Method

- **POST /convert/bitv4**: Converts an IP bit length to a netmask. Example: `27 -> 255.255.255.224`.
- **POST /convert/rgb**: Converts RGB values to a hexadecimal color code. Example: `55, 155, 250 -> #379BFA`.

### 🔄 Request (POST)

```shell
curl --location 'http://localhost:4000/convert/bitv4' \
--header 'Content-Type: application/json' \
--data '{
    "bit_length": 17
}'
```

### ✅ Response (POST)

```json
{
    "mask": "255.255.128.0"
}
```

## Proxy to UDP 123

- Listen UDP port 4123.
- Proxy to UDP port 123 at NICT Time Server.
