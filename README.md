# File Space Scanner
```text
███████╗███████╗███████╗
██╔════╝██╔════╝██╔════╝
█████╗  ███████╗███████╗
██╔══╝  ╚════██║╚════██║
██║     ███████║███████║
╚═╝     ╚══════╝╚══════╝
                        
``` 


#### **FSS — File Space Scanner**

*FSS (File Space Scanner)*  - tool for displays a list of all file systems by device name, reports their size, used and free space, and mount points. Convenient tree-like output.
Output example:
```bash
./fss -d 1
.  [114.30 MB]
├── deps  [67.84 MB]
├── build  [23.23 MB]
├── fss  [12.72 MB]
├── incremental  [8.82 MB]
├── libfss.rlib  [1.57 MB]
├── .fingerprint  [125.13 KB]
├── examples  [4.00 KB]
├── fss.d  [256 B]
├── libfss.d  [240 B]
└── .cargo-lock  [0 B]
```

Usage:
```bash
  fss [PATH] [OPTIONS]
```
Options:
  -d N, --depth N     Set tree display depth. 0 = show only root
  -h, --help          Show this help message

Behavior:
  - Scanning is ALWAYS full depth (infinite).
  - Display depth is controlled by --depth.
  - Without -r only one level is shown (even if depth > 1).

Examples:
```bash
  fss /etc
  fss /var/log -d 0
```
