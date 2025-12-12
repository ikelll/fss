# File Space Scanner
```text
███████╗███████╗███████╗
██╔════╝██╔════╝██╔════╝
█████╗  ███████╗███████╗
██╔══╝  ╚════██║╚════██║
██║     ███████║███████║
╚═╝     ╚══════╝╚══════╝
                        
``` 


#### **"FSS — File Space Scanner**

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
