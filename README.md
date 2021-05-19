# Random Load image

This is a simple docker image to situmlate the pod usage(cpu & memory).

When you use this, you must write a config file like `config-1.yaml`, and pass it to pod.

cpu stress supported by `stress-ng`

## Config

```
mod: <cpu> | <memory>  # mod can run cpu or memory
delayStart(second): a array of limit range
cpuRange(percentage) | memory(MB): a array of limit range
runRange(second): a array of excution time range
```


## Example
```yaml
mod: cpu
delayStart:
  - 0
  - 5
runRange:
  - 40
  - 50
cpuRange:
  - 30
  - 70
---
mod: memory
delayStart:
  - 0
  - 5
runRange:
  - 40
  - 50
memRange:
  - 1024
  - 2048
```

simply you can run:
```bash
cargo run -- -c config-1.yaml
```
