# ガラクタ API

Rust で書いた簡素な Web API です。

- GET /healthcheck で OK を返します。
- GET /metrics でホストの CPU Load / Disk Usage / Memory Usage を取得します。
- GET /metrics/cpuload で CPU Load を取得します。
- GET /metrics/memusage でメモリの利用状況を取得します。
- GET /metrics/diskusage でストレージの利用状況を取得します。
- GET /dice/roll_1d6 で 1 〜 6 をランダムで返します。
- GET /sleep/i32 で i32 秒 sleep します。threading 確認用です。e.g, /sleep/30
- POST /convert/bitv4 で IP アドレスの bit を netmask に変換します。e.g, 27 -> 255.255.255.224
- POST /convert/rgb で RGB を hex に変換します。e.g, 55,155,250 -> 379BFA
